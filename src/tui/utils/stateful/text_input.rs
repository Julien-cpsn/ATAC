use crate::app::files::key_bindings::{TextAreaMode, KEY_BINDINGS};
use crate::app::files::theme::THEME;
use crate::tui::utils::syntax_highlighting::{ENV_VARIABLE_SYNTAX_REF, ENV_VARIABLE_SYNTAX_SET, SYNTAX_SET, SYNTAX_THEME, THEME_SET};
use crokey::KeyCombination;
use edtui::actions::insert::PushLine;
use edtui::actions::motion::MoveToFirstRow;
use edtui::actions::search::StartSearch;
use edtui::actions::{Composed, CopySelection, DeleteChar, FindNext, FindPrevious, InsertChar, LineBreak, MoveBackward, MoveDown, MoveForward, MoveToEndOfLine, MoveToStartOfLine, MoveUp, MoveWordBackward, MoveWordForward, OpenSystemEditor, Paste, Redo, RemoveChar, RemoveCharFromSearch, SelectCurrentSearch, StopSearch, SwitchMode, Undo};
use edtui::events::{KeyEventHandler, KeyEventRegister};
use edtui::{system_editor, EditorEventHandler, EditorMode, EditorState, EditorStatusLine, EditorTheme, EditorView, LineNumbers, SyntaxHighlighter};
use ratatui::backend::CrosstermBackend;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::HorizontalAlignment;
use ratatui::prelude::{Constraint, Layout, Rect, Span, Style, Stylize, Widget};
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders, Padding};
use ratatui::Terminal;
use std::collections::HashMap;
use std::io::Stdout;
use syntect::parsing::SyntaxReference;

pub struct TextInput {
    pub state: EditorState,
    pub event_handler: Option<EditorEventHandler>,
    pub default_mode: EditorMode,
    pub block_title: Option<String>,
    pub is_single_line: bool,
    pub highlight_text: bool,
    pub highlight_block: bool,
    pub display_cursor: bool
}

impl Widget for &mut TextInput {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let new_area = match &self.block_title {
            None => area,
            Some(block_title) => {
                let mut block = Block::new()
                    .title(block_title.as_str())
                    .borders(Borders::ALL)
                    .padding(Padding::horizontal(1));

                block = match self.highlight_block {
                    true => block.fg(THEME.read().others.selection_highlight_color),
                    false => block.fg(THEME.read().ui.main_foreground_color),
                };

                let block_area = block.inner(area);

                block.render(area, buf);

                block_area
            }
        };

        let should_display_status = self.display_cursor && !(self.default_mode == EditorMode::Insert && self.state.mode == EditorMode::Insert);

        let constraints = match should_display_status {
            true => vec![
                Constraint::Fill(1),
                Constraint::Length(6),
            ],
            false => vec![
                Constraint::Fill(1)
            ]
        };

        let layout = Layout::horizontal(constraints).split(new_area);

        let mut theme = EditorTheme::default().hide_status_line();

        theme = match self.highlight_text {
            true => theme.base(Style::new().fg(THEME.read().ui.font_color)),
            false => theme.base(Style::new().fg(THEME.read().others.selection_highlight_color)),
        };

        if !self.display_cursor {
            theme = theme
                .cursor_style(Style::new())
                .selection_style(Style::new());
        }
        else if self.state.mode == EditorMode::Search {
            theme = theme.cursor_style(Style::new());
        }

        let syntax_highlighter = SyntaxHighlighter::new_custom(
            SYNTAX_THEME.clone(),
            &THEME_SET,
            ENV_VARIABLE_SYNTAX_REF.clone(),
            &ENV_VARIABLE_SYNTAX_SET
        );

        let editor = EditorView::new(&mut self.state)
            .theme(theme)
            .syntax_highlighter(Some(syntax_highlighter))
            .wrap(false);

        editor.render(layout[0], buf);

        if should_display_status {
            let status_line_bg_color = get_color_from_mode(&self.state.mode);

            let status_line = Span::raw(self.state.mode.name())
                .style(Style::new().fg(THEME.read().ui.font_color).bg(status_line_bg_color));

            status_line.render(layout[1], buf);
        }
    }
}

impl TextInput {
    pub fn new(block_title: Option<String>) -> Self {
        let state = EditorState::default();

        Self {
            state,
            event_handler: None,
            default_mode: EditorMode::Normal,
            block_title,
            is_single_line: true, // Placeholder
            highlight_text: false, // Placeholder
            highlight_block: false, // Placeholder
            display_cursor: false, // Placeholder
        }
    }

    pub fn update_handler(&mut self) {
        let editor_handler = EditorEventHandler::new(generate_key_handler(self.is_single_line));
        self.event_handler = Some(editor_handler);
    }

    pub fn multi_line_editor<'a, 'b, 'c>(&'a mut self, block_title: Option<&'a str>, syntax_highlighting: SyntaxReference) -> EditorView<'a, 'a, 'b, 'c> {
        let mut theme = EditorTheme::default()
            .base(Style::new().fg(THEME.read().ui.font_color))
            .line_numbers_style(Style::new().fg(THEME.read().ui.secondary_foreground_color));

        if !self.display_cursor {
            theme = theme
                .hide_status_line()
                .cursor_style(Style::new())
                .selection_style(Style::new());
        }
        else {
            if self.default_mode != EditorMode::Insert || self.state.mode == EditorMode::Search {
                let status_line_bg_color = get_color_from_mode(&self.state.mode);

                let status_line = EditorStatusLine::default()
                    .alignment(HorizontalAlignment::Center)
                    .style_mode(Style::new().fg(THEME.read().ui.font_color).bg(status_line_bg_color))
                    .style_search(Style::new().fg(THEME.read().ui.font_color))
                    .style_line(Style::new());

                theme = theme.status_line(status_line);
            }
            else {
                theme = theme.hide_status_line();
            }
        }

        if let Some(block_title) = block_title {
            let mut block = Block::new()
                .title(block_title)
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1));

            block = match self.highlight_block {
                true => block.fg(THEME.read().others.selection_highlight_color),
                false => block.fg(THEME.read().ui.main_foreground_color),
            };

            theme = theme.block(block);
        }

        let syntax_set = match &syntax_highlighting.name == "Double Brace Variables" {
            true => &*ENV_VARIABLE_SYNTAX_SET,
            false => &*SYNTAX_SET
        };


        let syntax_highlighter = SyntaxHighlighter::new_custom(
            SYNTAX_THEME.clone(),
            &THEME_SET,
            syntax_highlighting,
            &syntax_set
        );

        EditorView::new(&mut self.state)
            .theme(theme)
            .syntax_highlighter(Some(syntax_highlighter))
            .wrap(false)
            .line_numbers(LineNumbers::Absolute)
            .tab_width(4)
    }

    pub fn reset_mode(&mut self) {
        self.state.mode = self.default_mode;
    }

    pub fn reset_cursor_position(&mut self) {
        if self.is_single_line {
            self.move_cursor_line_end();
        }
        else {
            self.move_cursor_start();
        }
    }

    pub fn reset_selection(&mut self) {
        self.state.selection = None;
    }

    pub fn clear(&mut self) {
        self.state.lines.clear();
    }

    pub fn to_string(&self) -> String {
        self.state.lines.to_string()
    }

    pub fn to_lines(&self) -> Vec<String> {
        self.state.lines.to_vecs().iter().map(|line| line.iter().collect()).collect()
    }

    pub fn is_in_default_mode(&self) -> bool {
        self.state.mode == self.default_mode
    }

    #[allow(unused)]
    pub fn delete_char_forward(&mut self) {
        self.state.execute(RemoveChar(1));
    }

    #[allow(unused)]
    pub fn delete_char_backward(&mut self) {
        self.state.execute(DeleteChar(1));
    }

    #[allow(unused)]
    pub fn move_cursor_left(&mut self) {
        self.state.execute(MoveBackward(1));
    }

    #[allow(unused)]
    pub fn move_cursor_right(&mut self) {
        self.state.execute(MoveForward(1));
    }

    pub fn move_cursor_start(&mut self) {
        self.state.execute(MoveToFirstRow())
    }

    #[allow(unused)]
    pub fn move_cursor_line_start(&mut self) {
        self.state.execute(MoveToStartOfLine())
    }

    pub fn move_cursor_line_end(&mut self) {
        self.state.execute(MoveToEndOfLine())
    }

    #[allow(unused)]
    pub fn push_char(&mut self, char: char) {
        self.state.execute(InsertChar(char));
    }

    pub fn push_str(&mut self, line: &str) {
        for line in line.split('\n') {
            self.state.execute(PushLine(line));
        }
    }

    pub fn key_event(&mut self, key: KeyCombination, terminal: Option<&mut Terminal<CrosstermBackend<Stdout>>>) {
        self.event_handler.as_mut().unwrap().on_key_event::<KeyEvent>(key.into(), &mut self.state);

        if let Some(terminal) = terminal && system_editor::is_pending(&self.state) {
            system_editor::open(&mut self.state, terminal).ok();
        }
    }
}

fn generate_key_handler(is_single_line: bool) -> KeyEventHandler {
    let text_input = KEY_BINDINGS.read().generic.text_input;
    match text_input.mode {
        TextAreaMode::Vim => {
            let mut handler = KeyEventHandler::vim_mode();

            if is_single_line {
                // Remove new line
                handler.remove(&KeyEventRegister::i(vec![edtui::events::KeyEvent::Enter]));
                handler.remove(&KeyEventRegister::n(vec![edtui::events::KeyEvent::Char('o')]));
                handler.remove(&KeyEventRegister::n(vec![edtui::events::KeyEvent::Char('O')]));
                handler.remove(&KeyEventRegister::n(vec![edtui::events::KeyEvent::Char('J')]));

                // External system editor
                handler.remove(&KeyEventRegister::n(vec![edtui::events::KeyEvent::Ctrl('e')]));
            }

            handler
        },
        TextAreaMode::Emacs => {
            let mut handler = KeyEventHandler::emacs_mode();

            if is_single_line {
                // Remove new line
                handler.remove(&KeyEventRegister::i(vec![edtui::events::KeyEvent::Ctrl('o')]));
                handler.remove(&KeyEventRegister::i(vec![edtui::events::KeyEvent::Enter]));
                handler.remove(&KeyEventRegister::i(vec![edtui::events::KeyEvent::Ctrl('j')]));

                // External system editor
                handler.remove(&KeyEventRegister::i(vec![edtui::events::KeyEvent::Alt('e')]))
            }

            handler
        },
        TextAreaMode::Custom(custom_text_input) => {
            let copy: KeyEvent = custom_text_input.copy.into();
            let paste: KeyEvent = custom_text_input.paste.into();

            let undo: KeyEvent = custom_text_input.undo.into();
            let redo: KeyEvent = custom_text_input.redo.into();

            let system_editor: KeyEvent = custom_text_input.system_editor.into();

            let search: KeyEvent = custom_text_input.search.into();
            let quit_without_saving: KeyEvent = text_input.quit_without_saving.into();

            let new_line: KeyEvent = custom_text_input.new_line.into();

            let delete_backward: KeyEvent = custom_text_input.delete_backward.into();
            let delete_forward: KeyEvent = custom_text_input.delete_forward.into();

            let skip_word_left: KeyEvent = custom_text_input.skip_word_left.into();
            let skip_word_right: KeyEvent = custom_text_input.skip_word_right.into();

            let move_cursor_right: KeyEvent = custom_text_input.move_cursor_right.into();
            let move_cursor_left: KeyEvent = custom_text_input.move_cursor_left.into();
            let move_cursor_up: KeyEvent = custom_text_input.move_cursor_up.into();
            let move_cursor_down: KeyEvent = custom_text_input.move_cursor_down.into();
            let move_cursor_line_start: KeyEvent = custom_text_input.move_cursor_line_start.into();
            let move_cursor_line_end: KeyEvent = custom_text_input.move_cursor_line_end.into();

            let mut keys = HashMap::from([
                // SEARCH
                (
                    KeyEventRegister::i(vec![search.into()]),
                    Composed::new(StartSearch)
                        .chain(SwitchMode(EditorMode::Search))
                        .into()
                ),
                (KeyEventRegister::s(vec![move_cursor_down.into()]), FindNext.into()),
                (KeyEventRegister::s(vec![move_cursor_up.into()]), FindPrevious.into()),
                (
                    KeyEventRegister::s(vec![new_line.into()]),
                    Composed::new(SelectCurrentSearch)
                        .chain(SwitchMode(EditorMode::Insert))
                        .into()
                ),
                (
                    KeyEventRegister::s(vec![search.into()]),
                    Composed::new(StopSearch)
                        .chain(SwitchMode(EditorMode::Insert))
                        .into()
                ),
                (
                    KeyEventRegister::s(vec![quit_without_saving.into()]),
                    Composed::new(StopSearch)
                        .chain(SwitchMode(EditorMode::Insert))
                        .into()
                ),
                (KeyEventRegister::s(vec![delete_forward.into()]), RemoveCharFromSearch.into()),
                // INSERT
                (KeyEventRegister::i(vec![move_cursor_right.into()]), MoveForward(1).into()),
                (KeyEventRegister::i(vec![move_cursor_left.into()]), MoveBackward(1).into()),
                (KeyEventRegister::i(vec![move_cursor_up.into()]), MoveUp(1).into()),
                (KeyEventRegister::i(vec![move_cursor_down.into()]), MoveDown(1).into()),
                (KeyEventRegister::i(vec![skip_word_right.into()]), MoveWordForward(1).into()),
                (KeyEventRegister::i(vec![skip_word_left.into()]), MoveWordBackward(1).into()),
                (KeyEventRegister::i(vec![move_cursor_line_start.into()]), MoveToStartOfLine().into()),
                (KeyEventRegister::i(vec![move_cursor_line_end.into()]), MoveToEndOfLine().into()),
                (KeyEventRegister::i(vec![delete_forward.into()]), DeleteChar(1).into()),
                (KeyEventRegister::i(vec![delete_backward.into()]), RemoveChar(1).into()),
                (KeyEventRegister::i(vec![undo.into()]), Undo.into()),
                (KeyEventRegister::i(vec![redo.into()]), Redo.into()),
                (KeyEventRegister::i(vec![copy.into()]), CopySelection.into()),
                (KeyEventRegister::i(vec![paste.into()]), Paste.into()),
                (KeyEventRegister::i(vec![system_editor.into()]), OpenSystemEditor.into())
            ]);

            if !is_single_line {
                keys.insert(KeyEventRegister::i(vec![new_line.into()]), LineBreak(1).into());
            }

            KeyEventHandler::new(keys, false)
        }
    }
}

fn get_color_from_mode(mode: &EditorMode) -> Color {
    match mode {
        EditorMode::Normal => THEME.read().ui.secondary_foreground_color,
        EditorMode::Insert => Color::Green,
        EditorMode::Visual => Color::Yellow,
        EditorMode::Search => Color::Magenta
    }
}