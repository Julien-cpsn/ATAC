use std::fmt::{Display, Error, Formatter};

use crokey::{key, KeyCombination};
use crokey::OneToThree::One;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Alignment;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use ratatui::widgets::block::Position;
use tui_textarea::{CursorMove, Scrolling, TextArea};

use crate::app::app_states::EMPTY_KEY;

// State of Vim emulation
#[derive(Copy, Clone)]
pub struct Vim {
    pub(crate) mode: VimMode,
    pending: KeyCombination, // Pending input to handle a sequence with two keys like gg
}

impl Default for Vim {
    fn default() -> Self {
        Vim {
            mode: VimMode::default(),
            pending: *EMPTY_KEY, 
        }
    }
}

impl Vim {
    pub fn new(mode: VimMode) -> Self {
        Self {
            mode,
            pending: *EMPTY_KEY,
        }
    }

    pub fn with_pending(self, pending: KeyCombination) -> Self {
        Self {
            mode: self.mode,
            pending,
        }
    }

    pub fn transition(&self, input: KeyCombination, textarea: &mut TextArea<'_>) -> VimTransition {
        if input == *EMPTY_KEY {
            return VimTransition::Nop;
        }

        match self.mode {
            VimMode::Normal | VimMode::Visual | VimMode::Operator(_) => {
                match input {
                    key!(h) => textarea.move_cursor(CursorMove::Back),
                    key!(j) => textarea.move_cursor(CursorMove::Down),
                    key!(k) => textarea.move_cursor(CursorMove::Up),
                    key!(l) => textarea.move_cursor(CursorMove::Forward),
                    key!(w) => textarea.move_cursor(CursorMove::WordForward),
                    key!(b) => textarea.move_cursor(CursorMove::WordBack),
                    key!('^') => textarea.move_cursor(CursorMove::Head),
                    key!('$') => textarea.move_cursor(CursorMove::End),
                    key!(shift-D) => {
                        textarea.delete_line_by_end();
                        return VimTransition::Mode(VimMode::Normal);
                    }
                    key!(shift-C) => {
                        textarea.delete_line_by_end();
                        textarea.cancel_selection();
                        return VimTransition::Mode(VimMode::Insert);
                    }
                    key!(p) => {
                        textarea.paste();
                        return VimTransition::Mode(VimMode::Normal);
                    }
                    key!(u) => {
                        textarea.undo();
                        return VimTransition::Mode(VimMode::Normal);
                    }
                    key!(ctrl-r) => {
                        textarea.redo();
                        return VimTransition::Mode(VimMode::Normal);
                    }
                    key!(x) => {
                        textarea.delete_next_char();
                        return VimTransition::Mode(VimMode::Normal);
                    }
                    key!(i) => {
                        textarea.cancel_selection();
                        return VimTransition::Mode(VimMode::Insert);
                    }
                    key!(a) => {
                        textarea.cancel_selection();
                        textarea.move_cursor(CursorMove::Forward);
                        return VimTransition::Mode(VimMode::Insert);
                    }
                    key!(shift-A) => {
                        textarea.cancel_selection();
                        textarea.move_cursor(CursorMove::End);
                        return VimTransition::Mode(VimMode::Insert);
                    }
                    key!(o) => {
                        textarea.move_cursor(CursorMove::End);
                        textarea.insert_newline();
                        return VimTransition::Mode(VimMode::Insert);
                    }
                    key!(shift-O) => {
                        textarea.move_cursor(CursorMove::Head);
                        textarea.insert_newline();
                        textarea.move_cursor(CursorMove::Up);
                        return VimTransition::Mode(VimMode::Insert);
                    }
                    key!(shift-I) => {
                        textarea.cancel_selection();
                        textarea.move_cursor(CursorMove::Head);
                        return VimTransition::Mode(VimMode::Insert);
                    }
                    key!(q) => return VimTransition::Quit,
                    key!(ctrl-s) => return VimTransition::SaveAndQuit,
                    key!(ctrl-e) => textarea.scroll((1, 0)),
                    key!(ctrl-y) => textarea.scroll((-1, 0)),
                    key!(ctrl-d) => textarea.scroll(Scrolling::HalfPageDown),
                    key!(ctrl-u) => textarea.scroll(Scrolling::HalfPageUp),
                    key!(ctrl-f) => textarea.scroll(Scrolling::PageDown),
                    key!(ctrl-b) => textarea.scroll(Scrolling::PageUp),
                    key!(v) if self.mode == VimMode::Normal => {
                        textarea.start_selection();
                        return VimTransition::Mode(VimMode::Visual);
                    }
                    key!(shift-V) if self.mode == VimMode::Normal => {
                        textarea.move_cursor(CursorMove::Head);
                        textarea.start_selection();
                        textarea.move_cursor(CursorMove::End);
                        return VimTransition::Mode(VimMode::Visual);
                    }
                    key!(esc) | key!(v) if self.mode == VimMode::Visual => {
                        textarea.cancel_selection();
                        return VimTransition::Mode(VimMode::Normal);
                    }
                    key!(g) if matches!(self.pending, key!(g)) => {
                        textarea.move_cursor(CursorMove::Top)
                    }
                    key!(shift-G) => textarea.move_cursor(CursorMove::Bottom),
                    KeyCombination { codes: One(KeyCode::Char(char)), modifiers: KeyModifiers::NONE } if self.mode == VimMode::Operator(char) => {
                        // Handle yy, dd, cc. (This is not strictly the same behavior as Vim)
                        textarea.move_cursor(CursorMove::Head);
                        textarea.start_selection();
                        let cursor = textarea.cursor();
                        textarea.move_cursor(CursorMove::Down);
                        if cursor == textarea.cursor() {
                            textarea.move_cursor(CursorMove::End); // At the last line, move to end of the line instead
                        }
                    }
                    op @ (key!(y) | key!(d) | key!(c)) if self.mode == VimMode::Normal => {
                        textarea.start_selection();
                        return VimTransition::Mode(VimMode::Operator(op.as_letter().unwrap()));
                    }
                    key!(y) if self.mode == VimMode::Visual => {
                        textarea.copy();
                        return VimTransition::Mode(VimMode::Normal);
                    }
                    key!(d) if self.mode == VimMode::Visual => {
                        textarea.cut();
                        return VimTransition::Mode(VimMode::Normal);
                    }
                    key!(c) if self.mode == VimMode::Visual => {
                        textarea.cut();
                        return VimTransition::Mode(VimMode::Insert);
                    }
                    input => return VimTransition::Pending(input),
                }

                // Handle the pending operator
                match self.mode {
                    VimMode::Operator('y') => {
                        textarea.copy();
                        VimTransition::Mode(VimMode::Normal)
                    }
                    VimMode::Operator('d') => {
                        textarea.cut();
                        VimTransition::Mode(VimMode::Normal)
                    }
                    VimMode::Operator('c') => {
                        textarea.cut();
                        VimTransition::Mode(VimMode::Insert)
                    }
                    _ => VimTransition::Nop,
                }
            }
            VimMode::Insert => match input {
                key!(esc) | key!(ctrl-c) => VimTransition::Mode(VimMode::Normal),
                input => {
                    let key_event: KeyEvent = input.into();
                    textarea.input(key_event); // Use default key mappings in insert mode
                    VimTransition::Mode(VimMode::Insert)
                }
            },
        }
    }
}


#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum VimMode {
    #[default]
    Normal,
    Insert,
    Visual,
    Operator(char),
}

impl VimMode {
    pub fn block<'a>(&self) -> Block<'a> {
        let title = format!("-- {} MODE --", self);
        Block::default()
            .title(title.dark_gray())
            .title_position(Position::Top)
            .title_alignment(Alignment::Center)
    }

    pub fn cursor_style(&self) -> Style {
        let color = match self {
            Self::Normal => Color::Reset,
            Self::Insert => Color::LightBlue,
            Self::Visual => Color::LightYellow,
            Self::Operator(_) => Color::LightGreen,
        };
        Style::default().fg(color).add_modifier(Modifier::REVERSED)
    }
}

impl Display for VimMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
            Self::Visual => write!(f, "VISUAL"),
            Self::Operator(c) => write!(f, "OPERATOR({})", c),
        }
    }
}

// How the Vim emulation state transitions
pub enum VimTransition {
    Nop,
    Mode(VimMode),
    Pending(KeyCombination),
    Quit,
    SaveAndQuit
}