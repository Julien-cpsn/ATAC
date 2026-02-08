use crate::app::files::key_bindings::{CustomTextArea, TextAreaMode, KEY_BINDINGS};
use crate::app::files::theme::THEME;
use crate::tui::utils::syntax_highlighting::{ENV_VARIABLE_SYNTAX_REF, ENV_VARIABLE_SYNTAX_SET, SYNTAX_SET, SYNTAX_THEME, THEME_SET};
use crokey::KeyCombination;
use edtui::actions::insert::PushLine;
use edtui::actions::motion::MoveToFirstRow;
use edtui::actions::search::StartSearch;
use edtui::actions::{Composed, CopySelection, DeleteChar, FindNext, FindPrevious, InsertChar, LineBreak, MoveBackward, MoveDown, MoveForward, MoveToEndOfLine, MoveToStartOfLine, MoveUp, MoveWordBackward, MoveWordForward, OpenSystemEditor, Paste, Redo, RemoveChar, RemoveCharFromSearch, SelectCurrentSearch, StopSearch, SwitchMode, Undo};
use edtui::events::{KeyEventHandler, KeyEventRegister, KeyInput};
use edtui::{system_editor, EditorEventHandler, EditorMode, EditorState, EditorStatusLine, EditorTheme, EditorView, LineNumbers, SyntaxHighlighter};
use ratatui::backend::CrosstermBackend;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
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
    pub insert_mode_only: bool,
    pub highlight_text: bool,
    pub highlight_block: bool,
    pub display_cursor: bool
}

pub struct SingleLineTextInput<'a>(pub &'a mut TextInput);
pub struct MultiLineTextInput<'a>(pub &'a mut TextInput, pub SyntaxReference);

impl<'a> Widget for SingleLineTextInput<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let new_area = match &self.0.block_title {
            None => area,
            Some(block_title) => {
                let mut block = Block::new()
                    .title(block_title.as_str())
                    .borders(Borders::ALL)
                    .padding(Padding::horizontal(1));

                block = match self.0.highlight_block {
                    true => block.fg(THEME.read().others.selection_highlight_color),
                    false => block.fg(THEME.read().ui.main_foreground_color),
                };

                let block_area = block.inner(area);

                block.render(area, buf);

                block_area
            }
        };


        let should_display_status = self.0.display_cursor &&
            !self.0.insert_mode_only &&
            !(self.0.default_mode == EditorMode::Insert && self.0.state.mode == EditorMode::Insert);

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

        theme = match self.0.highlight_text {
            true => theme.base(Style::new().fg(THEME.read().ui.font_color)),
            false => theme.base(Style::new().fg(THEME.read().others.selection_highlight_color)),
        };

        if !self.0.display_cursor {
            theme = theme
                .cursor_style(Style::new())
                .selection_style(Style::new());
        }
        else if self.0.state.mode == EditorMode::Search {
            theme = theme.cursor_style(Style::new());
        }

        let syntax_highlighter = SyntaxHighlighter::with_sets(
            SYNTAX_THEME.clone(),
            THEME_SET.clone(),
            ENV_VARIABLE_SYNTAX_REF.clone(),
            ENV_VARIABLE_SYNTAX_SET.clone()
        );

        let editor = EditorView::new(&mut self.0.state)
            .theme(theme)
            .syntax_highlighter(Some(syntax_highlighter))
            .wrap(false);

        editor.render(layout[0], buf);

        if should_display_status {
            let status_line_bg_color = get_color_from_mode(&self.0.state.mode);

            let status_line = Span::raw(self.0.state.mode.name())
                .style(Style::new().fg(THEME.read().ui.font_color).bg(status_line_bg_color));

            status_line.render(layout[1], buf);
        }
    }
}

impl<'a> Widget for MultiLineTextInput<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let mut theme = EditorTheme::default()
            .base(Style::new().fg(THEME.read().ui.font_color))
            .line_numbers_style(Style::new().fg(THEME.read().ui.secondary_foreground_color));

        if !self.0.display_cursor {
            theme = theme
                .hide_status_line()
                .cursor_style(Style::new())
                .selection_style(Style::new());
        }
        else {
            if self.0.default_mode != EditorMode::Insert || self.0.state.mode == EditorMode::Search {
                let status_line_bg_color = get_color_from_mode(&self.0.state.mode);

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

        if let Some(block_title) = &self.0.block_title {
            let mut block = Block::new()
                .title(block_title.as_str())
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1));

            block = match self.0.highlight_block {
                true => block.fg(THEME.read().others.selection_highlight_color),
                false => block.fg(THEME.read().ui.main_foreground_color),
            };

            theme = theme.block(block);
        }

        let syntax_set = match &self.1.name == "Double Brace Variables" {
            true => &*ENV_VARIABLE_SYNTAX_SET,
            false => &*SYNTAX_SET
        };


        let syntax_highlighter = SyntaxHighlighter::with_sets(
            SYNTAX_THEME.clone(),
            THEME_SET.clone(),
            self.1,
            syntax_set.clone()
        );

        let editor = EditorView::new(&mut self.0.state)
            .theme(theme)
            .syntax_highlighter(Some(syntax_highlighter))
            .wrap(false)
            .line_numbers(LineNumbers::Absolute)
            .tab_width(4);

        editor.render(area, buf)
    }
}

impl TextInput {
    pub fn new(block_title: Option<String>) -> Self {
        let state = EditorState::default();

        Self {
            state,
            event_handler: None,
            default_mode: EditorMode::Insert, // Placeholder
            block_title,
            is_single_line: true, // Placeholder
            insert_mode_only: false, // Placeholder
            highlight_text: false, // Placeholder
            highlight_block: false, // Placeholder
            display_cursor: false, // Placeholder
        }
    }

    pub fn update_handler(&mut self) {
        let editor_handler = EditorEventHandler::new(generate_key_handler(self.is_single_line, self.insert_mode_only));
        self.event_handler = Some(editor_handler);
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
        let key_event: KeyEvent = key.into();

        match key_event.code {
            KeyCode::Char(_) | KeyCode::Enter |
            KeyCode::Esc | KeyCode::Backspace | KeyCode::Delete |
            KeyCode::Tab | KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down |
            KeyCode::Home | KeyCode::End => {}
            _ => return,
        };

        self.event_handler.as_mut().unwrap().on_key_event(key_event, &mut self.state);

        if let Some(terminal) = terminal && system_editor::is_pending(&self.state) {
            system_editor::open(&mut self.state, terminal).ok();
        }
    }
}

fn generate_key_handler(is_single_line: bool, insert_mode_only: bool) -> KeyEventHandler {
    let text_input = KEY_BINDINGS.read().generic.text_input;
    match text_input.mode {
        TextAreaMode::Vim => {
            let mut handler = KeyEventHandler::vim_mode();

            if is_single_line {
                // Remove new line
                handler.remove(&KeyEventRegister::i(vec![KeyInput::new(KeyCode::Enter)]));
                handler.remove(&KeyEventRegister::n(vec![KeyInput::new(KeyCode::Char('o'))]));
                handler.remove(&KeyEventRegister::n(vec![KeyInput::new(KeyCode::Char('O'))]));
                handler.remove(&KeyEventRegister::n(vec![KeyInput::new(KeyCode::Char('J'))]));

                // External system editor
                handler.remove(&KeyEventRegister::n(vec![KeyInput::ctrl(KeyCode::Char('e'))]));
            }

            if insert_mode_only {
                handler.remove(&KeyEventRegister::i(vec![KeyInput::new(KeyCode::Esc)]));
            }

            handler
        },
        TextAreaMode::Emacs => {
            let mut handler = KeyEventHandler::emacs_mode();

            if is_single_line {
                // Remove new line
                handler.remove(&KeyEventRegister::i(vec![KeyInput::ctrl(KeyCode::Char('o'))]));
                handler.remove(&KeyEventRegister::i(vec![KeyInput::new(KeyCode::Enter)]));
                handler.remove(&KeyEventRegister::i(vec![KeyInput::ctrl(KeyCode::Char('j'))]));

                // External system editor
                handler.remove(&KeyEventRegister::i(vec![KeyInput::alt(KeyCode::Char('e'))]))
            }

            if insert_mode_only {
                handler.remove(&KeyEventRegister::i(vec![KeyInput::ctrl('s')]));
            }

            handler
        },
        _ => {
            let custom_text_input = match text_input.mode {
                TextAreaMode::Default => CustomTextArea::default(),
                TextAreaMode::Custom(custom_text_input) => custom_text_input,
                _ => unreachable!()
            };

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
                (KeyEventRegister::s(vec![KeyInput::from(delete_forward)]), RemoveCharFromSearch.into()),
                // INSERT
                (KeyEventRegister::i(vec![KeyInput::from(move_cursor_right)]), MoveForward(1).into()),
                (KeyEventRegister::i(vec![KeyInput::from(move_cursor_left)]), MoveBackward(1).into()),
                (KeyEventRegister::i(vec![KeyInput::from(move_cursor_up)]), MoveUp(1).into()),
                (KeyEventRegister::i(vec![KeyInput::from(move_cursor_down)]), MoveDown(1).into()),
                (KeyEventRegister::i(vec![KeyInput::from(skip_word_right)]), MoveWordForward(1).into()),
                (KeyEventRegister::i(vec![KeyInput::from(skip_word_left)]), MoveWordBackward(1).into()),
                (KeyEventRegister::i(vec![KeyInput::from(move_cursor_line_start)]), MoveToStartOfLine().into()),
                (KeyEventRegister::i(vec![KeyInput::from(move_cursor_line_end)]), MoveToEndOfLine().into()),
                (KeyEventRegister::i(vec![KeyInput::from(delete_forward)]), DeleteChar(1).into()),
                (KeyEventRegister::i(vec![KeyInput::from(delete_backward)]), RemoveChar(1).into()),
                (KeyEventRegister::i(vec![KeyInput::from(undo)]), Undo.into()),
                (KeyEventRegister::i(vec![KeyInput::from(redo)]), Redo.into()),
                (KeyEventRegister::i(vec![KeyInput::from(copy)]), CopySelection.into()),
                (KeyEventRegister::i(vec![KeyInput::from(paste)]), Paste.into()),
                (KeyEventRegister::i(vec![KeyInput::from(system_editor)]), OpenSystemEditor.into())
            ]);

            if !is_single_line {
                keys.insert(KeyEventRegister::i(vec![KeyInput::from(new_line)]), LineBreak(1).into());
            }

            if !insert_mode_only {
                keys.extend([
                    // SEARCH
                    (
                        KeyEventRegister::i(vec![KeyInput::from(search)]),
                        Composed::new(StartSearch)
                            .chain(SwitchMode(EditorMode::Search))
                            .into()
                    ),
                    (KeyEventRegister::s(vec![KeyInput::from(move_cursor_down)]), FindNext.into()),
                    (KeyEventRegister::s(vec![KeyInput::from(move_cursor_up)]), FindPrevious.into()),
                    (
                        KeyEventRegister::s(vec![KeyInput::from(new_line)]),
                        Composed::new(SelectCurrentSearch)
                            .chain(SwitchMode(EditorMode::Insert))
                            .into()
                    ),
                    (
                        KeyEventRegister::s(vec![KeyInput::from(search)]),
                        Composed::new(StopSearch)
                            .chain(SwitchMode(EditorMode::Insert))
                            .into()
                    ),
                    (
                        KeyEventRegister::s(vec![KeyInput::from(quit_without_saving)]),
                        Composed::new(StopSearch)
                            .chain(SwitchMode(EditorMode::Insert))
                            .into()
                    ),
                ]);
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