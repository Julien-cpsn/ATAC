use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use tui_textarea::CursorMove;
use crate::app::app::{App};
use crate::app::app_states::AppState;
use crate::app::ui::param_tabs::param_tabs::RequestParamsTabs;

impl App<'_> {
    /// Handle events
    pub async fn handle_events(&mut self) {
        self.result_throbber_state.calc_next();

        // Refreshes the app every tick_rate
        if event::poll(self.tick_rate).unwrap() {
            // Block while a key is pressed
            if let Event::Key(key) = event::read().unwrap() {
                let mut miss_input = false;

                let previous_app_state = self.state;
                let control_pressed: bool = key.modifiers == KeyModifiers::CONTROL;
                let shift_pressed: bool = key.modifiers == KeyModifiers::SHIFT;

                // Debug tool
                //println!("{:?} {:?}", key.modifiers, key.code);

                if key.kind == KeyEventKind::Press {
                    match self.state {
                        AppState::Normal => match key.code {
                            KeyCode::Char('c') if control_pressed => self.should_quit = true,
                            KeyCode::Char('q') => self.should_quit = true,

                            KeyCode::Up => self.collections_tree.up(),
                            KeyCode::Down => self.collections_tree.down(),
                            KeyCode::Left => self.unselect_request(),
                            KeyCode::Right => self.collections_tree.state.toggle_selected(),
                            KeyCode::Enter => self.select_request(),

                            KeyCode::Char('c') => self.create_new_collection_state(),
                            KeyCode::Char('r') => self.create_new_request_state(),
                            KeyCode::Char('d') => self.delete_element(),

                            _ => miss_input = true
                        },

                        AppState::CreatingNewCollection => match key.code {
                            KeyCode::Char(char) => self.new_collection_input.enter_char(char),

                            KeyCode::Esc => self.normal_state(),
                            KeyCode::Enter => self.new_collection(),

                            KeyCode::Backspace => self.new_collection_input.delete_char_backward(),
                            KeyCode::Left => self.new_collection_input.move_cursor_left(),
                            KeyCode::Right => self.new_collection_input.move_cursor_right(),

                            _ => miss_input = true
                        },

                        AppState::CreatingNewRequest => match key.code {
                            KeyCode::Char(char) => self.new_request_popup.text_input.enter_char(char),

                            KeyCode::Esc => self.normal_state(),
                            KeyCode::Enter => self.new_request(),

                            KeyCode::Backspace => self.new_request_popup.text_input.delete_char_backward(),
                            KeyCode::Left => self.new_request_popup.text_input.move_cursor_left(),
                            KeyCode::Right => self.new_request_popup.text_input.move_cursor_right(),

                            KeyCode::Up => self.new_request_popup.previous_collection(),
                            KeyCode::Down => self.new_request_popup.next_collection(),

                            _ => miss_input = true
                        },

                        AppState::DeletingCollection => match key.code {
                            KeyCode::Esc => self.normal_state(),

                            KeyCode::Enter if self.delete_collection_popup.state => self.delete_collection(),
                            KeyCode::Enter if !self.delete_collection_popup.state => self.normal_state(),

                            KeyCode::Left => self.delete_collection_popup.change_state(),
                            KeyCode::Right => self.delete_collection_popup.change_state(),

                            _ => miss_input = true
                        },

                        AppState::DeletingRequest => match key.code {
                            KeyCode::Esc => self.normal_state(),

                            KeyCode::Enter if self.delete_request_popup.state => self.delete_request(),
                            KeyCode::Enter if !self.delete_request_popup.state => self.normal_state(),

                            KeyCode::Left => self.delete_request_popup.change_state(),
                            KeyCode::Right => self.delete_request_popup.change_state(),

                            _ => miss_input = true
                        },

                        /* /!\ Below, consider that a request has been selected /!\ */

                        AppState::SelectedRequest => {
                            // Param tabs
                            match self.request_param_tab {
                                RequestParamsTabs::QueryParams => match key.code {
                                    KeyCode::Enter if !control_pressed && self.request_param_table.is_selected() => self.edit_request_param_state(),

                                    KeyCode::Up => self.request_param_table.up(),
                                    KeyCode::Down => self.request_param_table.down(),
                                    KeyCode::Left | KeyCode::Right => self.request_param_table.change_y(),

                                    KeyCode::Char('p') if control_pressed => self.toggle_params_table_row(),

                                    _ => {}
                                },
                                RequestParamsTabs::Auth if self.auth_text_input_selection.usable => match key.code {
                                    KeyCode::Enter if !control_pressed => self.select_request_auth_input_text(),

                                    KeyCode::Up => self.auth_text_input_selection.previous(),
                                    KeyCode::Down => self.auth_text_input_selection.next(),

                                    _ => {}
                                }
                                RequestParamsTabs::Headers => {}
                                RequestParamsTabs::Body => match key.code {
                                    KeyCode::Enter if !control_pressed => self.edit_request_body_state(),
                                    _ => {}
                                }
                                RequestParamsTabs::Cookies => {},
                                _ => {}
                            }

                            match key.code {
                                KeyCode::Esc => self.normal_state(),

                                KeyCode::Char('p') if !control_pressed => self.load_request_query_params_tab(),

                                KeyCode::Char('a') if control_pressed => self.modify_request_auth(),
                                KeyCode::Char('a') => self.load_request_auth_param_tab(),

                                KeyCode::Char('b') if control_pressed => self.modify_request_content_type(),
                                KeyCode::Char('b') => self.load_request_body_param_tab(),

                                KeyCode::Char('u') => self.edit_request_url_state(),
                                KeyCode::Char('m') => self.modify_request_method(),

                                KeyCode::PageUp => self.result_scrollbar.page_up(),
                                KeyCode::PageDown => self.result_scrollbar.page_down(),

                                KeyCode::BackTab if shift_pressed => self.next_request_view(),
                                KeyCode::Tab if control_pressed => self.next_request_result_tab(),
                                KeyCode::Tab => self.next_request_param_tab(),

                                KeyCode::Enter if control_pressed => self.send_request().await,

                                _ => miss_input = true
                            }
                        },

                        AppState::EditingRequestUrl => match key.code {
                            KeyCode::Char(char) => self.url_text_input.enter_char(char),

                            KeyCode::Esc => self.select_request_state(),
                            KeyCode::Enter => self.modify_request_url(),

                            KeyCode::Delete => self.url_text_input.delete_char_forward(),
                            KeyCode::Backspace => self.url_text_input.delete_char_backward(),
                            KeyCode::Left => self.url_text_input.move_cursor_left(),
                            KeyCode::Right => self.url_text_input.move_cursor_right(),

                            _ => miss_input = true
                        },

                        AppState::EditingRequestParam => match key.code {
                            KeyCode::Char(char) => self.request_param_table.param_selection_text_input.enter_char(char),

                            KeyCode::Esc => self.select_request_state(),
                            KeyCode::Enter => self.modify_request_param(),

                            KeyCode::Delete => self.request_param_table.param_selection_text_input.delete_char_forward(),
                            KeyCode::Backspace => self.request_param_table.param_selection_text_input.delete_char_backward(),
                            KeyCode::Left => self.request_param_table.param_selection_text_input.move_cursor_left(),
                            KeyCode::Right => self.request_param_table.param_selection_text_input.move_cursor_right(),

                            _ => miss_input = true
                        }

                        AppState::EditingRequestAuthUsername => match key.code {
                            KeyCode::Char(char) => self.auth_basic_username_text_input.enter_char(char),

                            KeyCode::Esc => self.select_request_state(),
                            KeyCode::Enter => self.modify_request_auth_basic_username(),

                            KeyCode::Delete => self.auth_basic_username_text_input.delete_char_forward(),
                            KeyCode::Backspace => self.auth_basic_username_text_input.delete_char_backward(),
                            KeyCode::Left => self.auth_basic_username_text_input.move_cursor_left(),
                            KeyCode::Right => self.auth_basic_username_text_input.move_cursor_right(),

                            _ => miss_input = true
                        },

                        AppState::EditingRequestAuthPassword => match key.code {
                            KeyCode::Char(char) => self.auth_basic_password_text_input.enter_char(char),

                            KeyCode::Esc => self.select_request_state(),
                            KeyCode::Enter => self.modify_request_auth_basic_password(),

                            KeyCode::Delete => self.auth_basic_password_text_input.delete_char_forward(),
                            KeyCode::Backspace => self.auth_basic_password_text_input.delete_char_backward(),
                            KeyCode::Left => self.auth_basic_password_text_input.move_cursor_left(),
                            KeyCode::Right => self.auth_basic_password_text_input.move_cursor_right(),

                            _ => miss_input = true
                        },

                        AppState::EditingRequestAuthBearerToken => match key.code {
                            KeyCode::Char(char) => self.auth_bearer_token_text_input.enter_char(char),

                            KeyCode::Esc => self.select_request_state(),
                            KeyCode::Enter => self.modify_request_auth_bearer_token(),

                            KeyCode::Delete => self.auth_bearer_token_text_input.delete_char_forward(),
                            KeyCode::Backspace => self.auth_bearer_token_text_input.delete_char_backward(),
                            KeyCode::Left => self.auth_bearer_token_text_input.move_cursor_left(),
                            KeyCode::Right => self.auth_bearer_token_text_input.move_cursor_right(),

                            _ => miss_input = true
                        }

                        AppState::EditingRequestBody => match key.code {
                            KeyCode::Char('c') if control_pressed => self.body_text_area.copy(),
                            KeyCode::Char('v') if control_pressed => {
                                self.body_text_area.paste();
                            },
                            KeyCode::Char('z') if control_pressed => {
                                self.body_text_area.undo();
                            },
                            KeyCode::Char('y') if control_pressed => {
                                self.body_text_area.redo();
                            },
                            KeyCode::Char('s') if control_pressed => self.modify_request_body(),

                            KeyCode::Char(char) => self.body_text_area.insert_char(char),

                            KeyCode::Esc => self.quit_request_body(),
                            KeyCode::Enter => self.body_text_area.insert_newline(),

                            KeyCode::Tab => {
                                self.body_text_area.set_hard_tab_indent(true);
                                self.body_text_area.insert_tab();
                            },
                            KeyCode::Backspace => {
                                self.body_text_area.delete_char();
                            },
                            KeyCode::Delete => {
                                self.body_text_area.delete_next_char();
                            },

                            KeyCode::Right if control_pressed => self.body_text_area.move_cursor(CursorMove::WordForward),
                            KeyCode::Left if control_pressed => self.body_text_area.move_cursor(CursorMove::WordBack),

                            KeyCode::Up => self.body_text_area.move_cursor(CursorMove::Up),
                            KeyCode::Down => self.body_text_area.move_cursor(CursorMove::Bottom),
                            KeyCode::Right => self.body_text_area.move_cursor(CursorMove::Forward),
                            KeyCode::Left => self.body_text_area.move_cursor(CursorMove::Back),

                            _ => miss_input = true
                        },
                    };

                    if !miss_input {
                        self.write_to_log_file(format!("{:?}", key.modifiers), format!("{:?}", key.code), previous_app_state.to_string());
                    }
                }
            }
        }
    }
}