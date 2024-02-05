use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::io::Result;
use tui_textarea::CursorMove;
use crate::app::app::{App, AppState};
use crate::app::tabs::tabs::{RequestTabs};

impl App<'_> {
    /// Handle events
    pub async fn handle_events(&mut self) -> Result<bool> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {

                match self.state {
                    AppState::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(true),

                        KeyCode::Up => self.collection.previous(),
                        KeyCode::Down => self.collection.next(),
                        KeyCode::Right => self.select_request(),
                        KeyCode::Left => self.unselect_request(),

                        KeyCode::Char('n') => self.state = AppState::CreatingNewRequest,
                        KeyCode::Char('d') => self.delete_request(),

                        _ => {}
                    },

                    AppState::CreatingNewRequest => match key.code {
                        KeyCode::Char(char) => self.new_request_input.enter_char(char),
                        KeyCode::Esc => self.state = AppState::Normal,
                        KeyCode::Enter => self.new_request(),
                        KeyCode::Backspace => self.new_request_input.delete_char(),
                        KeyCode::Left => self.new_request_input.move_cursor_left(),
                        KeyCode::Right => self.new_request_input.move_cursor_right(),
                        _ => {}
                    },

                    _ => {}
                };

                // A request is selected
                if self.collection.selected.is_some() {
                    match self.state {
                        AppState::Normal => match key.code {
                            KeyCode::Char('b') => {
                                self.request_tab = RequestTabs::Body;
                                self.state = AppState::EditingBody;
                            },
                            //next_request_tab(self.request_tab),

                            KeyCode::Char('u') => self.state = AppState::EditingUrl,
                            KeyCode::Char('m') => self.modify_request_method(),

                            KeyCode::Enter => self.send_request().await,

                            KeyCode::PageUp => self.result_scrollbar.page_up(),
                            KeyCode::PageDown => self.result_scrollbar.page_down(),

                            _ => {}

                        },

                        AppState::EditingUrl => match key.code {
                            KeyCode::Char(char) => self.url_text_input.enter_char(char),
                            KeyCode::Esc => self.state = AppState::Normal,
                            KeyCode::Enter => self.modify_request_url(),
                            KeyCode::Backspace => self.url_text_input.delete_char(),
                            KeyCode::Left => self.url_text_input.move_cursor_left(),
                            KeyCode::Right => self.url_text_input.move_cursor_right(),
                            _ => {}
                        }

                        AppState::EditingBody => match key.code {
                            KeyCode::Char(char) => self.body_text_area.insert_char(char),

                            KeyCode::Esc => self.modify_request_body(),
                            KeyCode::Enter => self.body_text_area.insert_newline(),

                            KeyCode::Tab => {
                                self.body_text_area.insert_tab();
                            },
                            KeyCode::Backspace => {
                                self.body_text_area.delete_char();
                            },
                            KeyCode::Delete => {
                                self.body_text_area.delete_next_char();
                            },

                            KeyCode::Up => self.body_text_area.move_cursor(CursorMove::Up),
                            KeyCode::Down => self.body_text_area.move_cursor(CursorMove::Bottom),
                            KeyCode::Right => self.body_text_area.move_cursor(CursorMove::Forward),
                            KeyCode::Left => self.body_text_area.move_cursor(CursorMove::Back),

                            _ => {}
                        },

                        _ => {}
                    };
                }
            }
        }

        return Ok(false);
    }
}