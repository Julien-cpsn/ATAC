use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use tui_textarea::CursorMove;
use crate::app::app::{App};
use crate::app::app_states::AppState;
use crate::app::ui::param_tabs::param_tabs::RequestParamsTabs;
use crate::app::ui::views::RequestView;

impl App<'_> {
    /// Handle events
    pub async fn handle_events(&mut self) {
        // Refreshes the app every tick_rate
        if event::poll(self.tick_rate).unwrap() {
            // Block while a key is pressed
            if let Event::Key(key) = event::read().unwrap() {
                let mut miss_input = false;

                let previous_app_state = self.state;
                let control_pressed: bool = key.modifiers == KeyModifiers::CONTROL;

                // Debug tool
                //println!("{:?} {:?}", key.modifiers, key.code);

                if key.kind == KeyEventKind::Press {
                    match self.state {
                        AppState::Normal => match key.code {
                            KeyCode::Char('c') if control_pressed => self.should_quit = true,
                            KeyCode::Char('q') => self.should_quit = true,

                            KeyCode::Up if control_pressed => self.move_request_up(),
                            KeyCode::Down if control_pressed => self.move_request_down(),

                            KeyCode::Up => self.collections_tree.up(),
                            KeyCode::Down => self.collections_tree.down(),
                            KeyCode::Left => self.unselect_request(),
                            KeyCode::Right => {
                                self.collections_tree.state.toggle_selected();
                            },
                            KeyCode::Enter => self.select_request_or_expand_collection(),

                            KeyCode::Char('e') => self.next_environment(),

                            KeyCode::Char('c') => self.display_cookies_state(),
                            
                            KeyCode::Char('n') => self.choose_element_to_create_state(),
                            KeyCode::Char('d') => self.delete_element(),
                            KeyCode::Char('r') => self.rename_element(),
                            
                            KeyCode::Char('h') => self.display_full_help = !self.display_full_help,

                            _ => miss_input = true
                        },
                        
                        /* Cookies */
                        
                        AppState::DisplayingCookies => match key.code {
                            KeyCode::Esc => self.normal_state(),
                            //KeyCode::Enter if !control_pressed && self.cookies_popup.cookies_table.is_selected() => self.edit_cookie_state(),

                            KeyCode::Up => self.cookies_popup.cookies_table.up(),
                            KeyCode::Down => self.cookies_popup.cookies_table.down(),
                            KeyCode::Left => self.cookies_popup.cookies_table.left(),
                            KeyCode::Right => self.cookies_popup.cookies_table.right(),

                            //KeyCode::Char('n') => self.create_new_cookie(),
                            KeyCode::Char('d') => self.delete_cookie(),

                            _ => {}
                        },

                        AppState::EditingCookies => match key.code {
                            KeyCode::Char(char) => self.cookies_popup.cookies_table.selection_text_input.enter_char(char),

                            KeyCode::Esc => self.display_cookies_state(),
                            KeyCode::Enter => self.modify_cookie(),

                            KeyCode::Delete => self.cookies_popup.cookies_table.selection_text_input.delete_char_forward(),
                            KeyCode::Backspace => self.cookies_popup.cookies_table.selection_text_input.delete_char_backward(),
                            KeyCode::Left => self.cookies_popup.cookies_table.selection_text_input.move_cursor_left(),
                            KeyCode::Right => self.cookies_popup.cookies_table.selection_text_input.move_cursor_right(),

                            _ => miss_input = true
                        },

                        /* Collections */

                        AppState::ChoosingElementToCreate => match key.code {
                            KeyCode::Esc => self.normal_state(),

                            KeyCode::Enter => self.new_element(),

                            KeyCode::Left => self.creation_popup.previous(),
                            KeyCode::Right => self.creation_popup.next(),

                            _ => miss_input = true
                        },

                        AppState::CreatingNewCollection => match key.code {
                            KeyCode::Char(char) => self.new_collection_input.enter_char(char),

                            KeyCode::Esc => self.normal_state(),
                            KeyCode::Enter => self.new_collection(),

                            KeyCode::Delete => self.new_collection_input.delete_char_forward(),
                            KeyCode::Backspace => self.new_collection_input.delete_char_backward(),
                            KeyCode::Left => self.new_collection_input.move_cursor_left(),
                            KeyCode::Right => self.new_collection_input.move_cursor_right(),

                            _ => miss_input = true
                        },

                        AppState::CreatingNewRequest => match key.code {
                            KeyCode::Char(char) => self.new_request_popup.text_input.enter_char(char),

                            KeyCode::Esc => self.normal_state(),
                            KeyCode::Enter => self.new_request(),

                            KeyCode::Delete => self.new_request_popup.text_input.delete_char_forward(),
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

                        AppState::RenamingCollection => match key.code {
                            KeyCode::Char(char) => self.rename_collection_input.enter_char(char),

                            KeyCode::Esc => self.normal_state(),
                            KeyCode::Enter => self.rename_collection(),

                            KeyCode::Delete => self.rename_collection_input.delete_char_forward(),
                            KeyCode::Backspace => self.rename_collection_input.delete_char_backward(),
                            KeyCode::Left => self.rename_collection_input.move_cursor_left(),
                            KeyCode::Right => self.rename_collection_input.move_cursor_right(),

                            _ => miss_input = true
                        },
                        
                        AppState::RenamingRequest => match key.code {
                            KeyCode::Char(char) => self.rename_request_input.enter_char(char),

                            KeyCode::Esc => self.normal_state(),
                            KeyCode::Enter => self.rename_request(),

                            KeyCode::Delete => self.rename_request_input.delete_char_forward(),
                            KeyCode::Backspace => self.rename_request_input.delete_char_backward(),
                            KeyCode::Left => self.rename_request_input.move_cursor_left(),
                            KeyCode::Right => self.rename_request_input.move_cursor_right(),

                            _ => miss_input = true
                        },

                        /* Request */
                        /* /!\ Below, consider that a request has been selected /!\ */

                        AppState::SelectedRequest => {
                            // Depending on the current request view, some keys may need to be deactivated
                            let (params_events_allowed, result_events_allowed) = match self.request_view {
                                RequestView::Normal => (true, true),
                                RequestView::OnlyResult => (false, true),
                                RequestView::OnlyParams => (true, false)
                            };
                            
                            // Param tabs
                            if params_events_allowed {
                                match self.request_param_tab {
                                    RequestParamsTabs::QueryParams => match key.code {
                                        KeyCode::Enter if !control_pressed && self.query_params_table.is_selected() => self.edit_request_param_state(),

                                        KeyCode::Up if !control_pressed => self.query_params_table.up(),
                                        KeyCode::Down if !control_pressed => self.query_params_table.down(),
                                        KeyCode::Left | KeyCode::Right if !control_pressed => self.query_params_table.change_y(),

                                        KeyCode::Char('n') => self.create_new_query_param(),
                                        KeyCode::Char('d') => self.delete_query_param(),
                                        KeyCode::Char('t') => self.toggle_query_param(),

                                        _ => {}
                                    },
                                    RequestParamsTabs::Auth if params_events_allowed && self.auth_text_input_selection.usable => match key.code {
                                        KeyCode::Enter if !control_pressed => self.select_request_auth_input_text(),

                                        KeyCode::Up if !control_pressed => self.auth_text_input_selection.previous(),
                                        KeyCode::Down if !control_pressed => self.auth_text_input_selection.next(),

                                        _ => {}
                                    }
                                    RequestParamsTabs::Headers => match key.code {
                                        KeyCode::Enter if !control_pressed && self.headers_table.is_selected() => self.edit_request_header_state(),

                                        KeyCode::Up if !control_pressed => self.headers_table.up(),
                                        KeyCode::Down if !control_pressed => self.headers_table.down(),
                                        KeyCode::Left | KeyCode::Right if !control_pressed => self.headers_table.change_y(),

                                        KeyCode::Char('n') => self.create_new_header(),
                                        KeyCode::Char('d') => self.delete_header(),
                                        KeyCode::Char('t') => self.toggle_header(),

                                        _ => {}
                                    },
                                    RequestParamsTabs::Body => match key.code {
                                        KeyCode::Enter if !control_pressed && self.body_form_table.is_selected() => self.edit_request_body_table_state(),
                                        KeyCode::Enter if !control_pressed => self.edit_request_body_file_or_string_state(),

                                        KeyCode::Up if !control_pressed => self.body_form_table.up(),
                                        KeyCode::Down if !control_pressed => self.body_form_table.down(),
                                        KeyCode::Left | KeyCode::Right if !control_pressed => self.body_form_table.change_y(),

                                        KeyCode::Char('n') => self.create_new_form_data(),
                                        KeyCode::Char('d') => self.delete_form_data(),
                                        KeyCode::Char('t') => self.toggle_form_data(),

                                        _ => {}
                                    },
                                    _ => {}
                                }
                            }
                            
                            if params_events_allowed {
                                match key.code {
                                    //KeyCode::Char('p') => self.load_request_query_params_tab(),

                                    KeyCode::Char('a') if control_pressed => self.modify_request_auth(),
                                    //KeyCode::Char('a') => self.load_request_auth_param_tab(),

                                    //KeyCode::Char('h') => self.load_request_headers_tab(),

                                    KeyCode::Char('b') if control_pressed => self.modify_request_content_type(),
                                    //KeyCode::Char('b') => self.load_request_body_param_tab(),

                                    KeyCode::Tab => self.next_request_param_tab(),

                                    _ => {}
                                }
                            }

                            if result_events_allowed {
                                match key.code {
                                    KeyCode::Up if control_pressed => self.result_vertical_scrollbar.page_up(),
                                    KeyCode::Down if control_pressed => self.result_vertical_scrollbar.page_down(),
                                    KeyCode::Left if control_pressed => self.result_horizontal_scrollbar.page_up(),
                                    KeyCode::Right if control_pressed => self.result_horizontal_scrollbar.page_down(),

                                    KeyCode::Char('y') => self.copy_to_clipboard(),
                                    KeyCode::BackTab if params_events_allowed => self.next_request_result_tab(),
                                    KeyCode::Tab if !params_events_allowed => self.next_request_result_tab(),

                                    _ => {}
                                }
                            }

                            match key.code {
                                KeyCode::Esc => self.normal_state(),

                                KeyCode::Char('c') => self.display_cookies_state(),
                                KeyCode::Char('e') => self.next_environment(),

                                KeyCode::Char('h') => self.display_full_help = !self.display_full_help,

                                KeyCode::Char('u') => self.edit_request_url_state(),
                                KeyCode::Char('m') => self.modify_request_method(),

                                KeyCode::Char('s') => self.edit_request_settings_state(),

                                KeyCode::Char('v') => self.next_request_view(),

                                // Used to be ctrl + enter, but it doesn't register right on many platforms
                                // https://github.com/crossterm-rs/crossterm/issues/685
                                KeyCode::Char(' ') => self.send_request().await,
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
                            KeyCode::Char(char) => self.query_params_table.selection_text_input.enter_char(char),

                            KeyCode::Esc => self.select_request_state(),
                            KeyCode::Enter => self.modify_request_query_param(),

                            KeyCode::Delete => self.query_params_table.selection_text_input.delete_char_forward(),
                            KeyCode::Backspace => self.query_params_table.selection_text_input.delete_char_backward(),
                            KeyCode::Left => self.query_params_table.selection_text_input.move_cursor_left(),
                            KeyCode::Right => self.query_params_table.selection_text_input.move_cursor_right(),

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

                        AppState::EditingRequestHeader => match key.code {
                            KeyCode::Char(char) => self.headers_table.selection_text_input.enter_char(char),

                            KeyCode::Esc => self.select_request_state(),
                            KeyCode::Enter => self.modify_request_header(),

                            KeyCode::Delete => self.headers_table.selection_text_input.delete_char_forward(),
                            KeyCode::Backspace => self.headers_table.selection_text_input.delete_char_backward(),
                            KeyCode::Left => self.headers_table.selection_text_input.move_cursor_left(),
                            KeyCode::Right => self.headers_table.selection_text_input.move_cursor_right(),

                            _ => miss_input = true
                        }
                        
                        AppState::EditingRequestBodyTable => match key.code {
                            KeyCode::Char(char) => self.body_form_table.selection_text_input.enter_char(char),

                            KeyCode::Esc => self.select_request_state(),
                            KeyCode::Enter => self.modify_request_form_data(),

                            KeyCode::Delete => self.body_form_table.selection_text_input.delete_char_forward(),
                            KeyCode::Backspace => self.body_form_table.selection_text_input.delete_char_backward(),
                            KeyCode::Left => self.body_form_table.selection_text_input.move_cursor_left(),
                            KeyCode::Right => self.body_form_table.selection_text_input.move_cursor_right(),

                            _ => miss_input = true
                        }

                        AppState::EditingRequestBodyFile => match key.code {
                            KeyCode::Char(char) => self.body_file_text_input.enter_char(char),

                            KeyCode::Esc => self.select_request_state(),
                            KeyCode::Enter => self.modify_request_body(),

                            KeyCode::Delete => self.body_file_text_input.delete_char_forward(),
                            KeyCode::Backspace => self.body_file_text_input.delete_char_backward(),
                            KeyCode::Left => self.body_file_text_input.move_cursor_left(),
                            KeyCode::Right => self.body_file_text_input.move_cursor_right(),

                            _ => miss_input = true
                        }

                        AppState::EditingRequestBodyString => match key.code {
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

                        AppState::EditingRequestSettings => match key.code {
                            KeyCode::Esc => self.select_request_state(),

                            KeyCode::Enter => self.modify_request_settings(),

                            KeyCode::Up => self.request_settings_popup.previous(),
                            KeyCode::Down => self.request_settings_popup.next(),
                            KeyCode::Left => self.request_settings_popup.toggle_setting(),
                            KeyCode::Right => self.request_settings_popup.toggle_setting(),

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
