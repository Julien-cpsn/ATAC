use crokey::{key, KeyCombination};
use crokey::OneToThree::One;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use tui_textarea::CursorMove;

use crate::app::app::App;
use crate::app::app_states::AppState;
use crate::app::files::key_bindings::{KEY_BINDINGS, TextAreaMode};
use crate::app::ui::param_tabs::param_tabs::RequestParamsTabs;
use crate::app::ui::views::RequestView;
use crate::utils::vim_emulation::{Vim, VimTransition};

impl App<'_> {
    /// Handle events
    pub async fn handle_events(&mut self) {
        // Refreshes the app every tick_rate
        if event::poll(self.tick_rate).unwrap() {
            // Block while a key is pressed
            if let Event::Key(key_event) = event::read().unwrap() {
                // We do not need
                if key_event.kind != KeyEventKind::Press {
                    return;
                }

                let key_bindings = KEY_BINDINGS.read().unwrap();
                let key = KeyCombination::from(key_event);

                // Debug tool
                //dbg!("{}", key.to_string());

                let mut miss_input = false;
                let previous_app_state = self.state;

                match self.state {
                    AppState::Normal => match key {
                        key!(ctrl-c)  => self.should_quit = true,
                        key if key == key_bindings.main_menu.quit => self.should_quit = true,

                        key if key == key_bindings.generic.navigation.move_cursor_up => self.collections_tree.up(),
                        key if key == key_bindings.generic.navigation.move_cursor_down => self.collections_tree.down(),
                        key if key == key_bindings.generic.navigation.select => self.select_request_or_expand_collection(),

                        key if key == key_bindings.main_menu.collections_expand => {self.collections_tree.state.toggle_selected();},
                        key if key == key_bindings.main_menu.unselect_request => self.unselect_request(),

                        key if key == key_bindings.main_menu.move_request_up => self.move_request_up(),
                        key if key == key_bindings.main_menu.move_request_down => self.move_request_down(),

                        key if key == key_bindings.main_menu.next_environment => self.next_environment(),

                        key if key == key_bindings.main_menu.display_cookies => self.display_cookies_state(),

                        key if key == key_bindings.generic.list_and_table_actions.create_element => self.choose_element_to_create_state(),
                        key if key == key_bindings.generic.list_and_table_actions.delete_element => self.delete_element(),
                        key if key == key_bindings.generic.list_and_table_actions.rename_element => self.rename_element(),

                        key if key == key_bindings.main_menu.display_help => {}, // TODO

                        _ => {}
                    },

                    /* Cookies */

                    AppState::DisplayingCookies => match key {
                        key if key == key_bindings.generic.navigation.go_back => self.normal_state(),

                        key if key == key_bindings.generic.navigation.move_cursor_up => self.cookies_popup.cookies_table.up(),
                        key if key == key_bindings.generic.navigation.move_cursor_down => self.cookies_popup.cookies_table.down(),
                        key if key == key_bindings.generic.navigation.move_cursor_left => self.cookies_popup.cookies_table.left(),
                        key if key == key_bindings.generic.navigation.move_cursor_right => self.cookies_popup.cookies_table.right(),

                        key if key == key_bindings.generic.list_and_table_actions.delete_element => self.delete_cookie(),

                        //KeyCode::Enter if !control_pressed && self.cookies_popup.cookies_table.is_selected() => self.edit_cookie_state(),
                        //KeyCode::Char('n') => self.create_new_cookie(),

                        _ => {}
                    },

                    /*
                    AppState::EditingCookies => match key {
                        KeyCode::Char(char) => self.cookies_popup.cookies_table.selection_text_input.enter_char(char),

                        KeyCode::Esc => self.display_cookies_state(),
                        KeyCode::Enter => self.modify_cookie(),

                        KeyCode::Delete => self.cookies_popup.cookies_table.selection_text_input.delete_char_forward(),
                        KeyCode::Backspace => self.cookies_popup.cookies_table.selection_text_input.delete_char_backward(),
                        KeyCode::Left => self.cookies_popup.cookies_table.selection_text_input.move_cursor_left(),
                        KeyCode::Right => self.cookies_popup.cookies_table.selection_text_input.move_cursor_right(),

                        _ => miss_input = true
                    },*/

                    /* Collections */

                    AppState::ChoosingElementToCreate => match key {
                        key if key == key_bindings.generic.navigation.go_back => self.normal_state(),

                        key if key == key_bindings.generic.navigation.move_cursor_left => self.creation_popup.previous(),
                        key if key == key_bindings.generic.navigation.move_cursor_right => self.creation_popup.next(),

                        key if key == key_bindings.generic.navigation.select => self.new_element(),

                        _ => miss_input = true
                    },

                    AppState::CreatingNewCollection => match key {
                        key if key == key_bindings.generic.text_inputs.text_input.cancel => self.normal_state(),
                        key if key == key_bindings.generic.text_inputs.text_input.validate => self.new_collection(),

                        key if key == key_bindings.generic.text_inputs.text_input.delete_backward => self.new_collection_input.delete_char_backward(),
                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.new_collection_input.delete_char_forward(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_left => self.new_collection_input.move_cursor_left(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_right => self.new_collection_input.move_cursor_right(),

                        KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.new_collection_input.enter_char(char),

                        _ => miss_input = true
                    },

                    AppState::CreatingNewRequest => match key {
                        key if key == key_bindings.generic.text_inputs.text_input.cancel => self.normal_state(),
                        key if key == key_bindings.generic.text_inputs.text_input.validate => self.new_request(),

                        key if key == key_bindings.generic.text_inputs.text_input.delete_backward => self.new_request_popup.text_input.delete_char_backward(),
                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.new_request_popup.text_input.delete_char_forward(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_left => self.new_request_popup.text_input.move_cursor_left(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_right => self.new_request_popup.text_input.move_cursor_right(),

                        key if key == key_bindings.generic.navigation.move_cursor_up => self.new_request_popup.previous_collection(),
                        key if key == key_bindings.generic.navigation.move_cursor_down => self.new_request_popup.next_collection(),

                        KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.new_request_popup.text_input.enter_char(char),

                        _ => miss_input = true
                    },

                    AppState::DeletingCollection => match key {
                        key if key == key_bindings.generic.navigation.go_back => self.normal_state(),

                        key if key == key_bindings.generic.navigation.select && self.delete_collection_popup.state => self.delete_collection(),
                        key if key == key_bindings.generic.navigation.select && !self.delete_collection_popup.state => self.normal_state(),

                        key if key == key_bindings.generic.navigation.move_cursor_left => self.delete_collection_popup.change_state(),
                        key if key == key_bindings.generic.navigation.move_cursor_right => self.delete_collection_popup.change_state(),

                        _ => miss_input = true
                    },

                    AppState::DeletingRequest => match key {
                        key if key == key_bindings.generic.navigation.go_back => self.normal_state(),

                        key if key == key_bindings.generic.navigation.select && self.delete_request_popup.state => self.delete_request(),
                        key if key == key_bindings.generic.navigation.select && !self.delete_request_popup.state => self.normal_state(),

                        key if key == key_bindings.generic.navigation.move_cursor_left => self.delete_request_popup.change_state(),
                        key if key == key_bindings.generic.navigation.move_cursor_right => self.delete_request_popup.change_state(),

                        _ => miss_input = true
                    },

                    AppState::RenamingCollection => match key {
                        key if key == key_bindings.generic.text_inputs.text_input.cancel => self.normal_state(),
                        key if key == key_bindings.generic.text_inputs.text_input.validate => self.rename_collection(),

                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.rename_collection_input.delete_char_backward(),
                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.rename_collection_input.delete_char_forward(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_left => self.rename_collection_input.move_cursor_left(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_right => self.rename_collection_input.move_cursor_right(),

                        KeyCombination { codes: One(KeyCode::Char(char)), modifiers: KeyModifiers::NONE } => self.rename_collection_input.enter_char(char),

                        _ => miss_input = true
                    },

                    AppState::RenamingRequest => match key {
                        key if key == key_bindings.generic.text_inputs.text_input.cancel => self.normal_state(),
                        key if key == key_bindings.generic.text_inputs.text_input.validate => self.rename_request(),

                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.rename_request_input.delete_char_backward(),
                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.rename_request_input.delete_char_forward(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_left => self.rename_request_input.move_cursor_left(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_right => self.rename_request_input.move_cursor_right(),

                        KeyCombination { codes: One(KeyCode::Char(char)), modifiers: KeyModifiers::NONE } => self.rename_request_input.enter_char(char),

                        _ => miss_input = true
                    },

                    /* Request */
                    /* /!\ Below, consider that a request has been selected /!\ */

                    AppState::SelectedRequest  => {
                        // Depending on the current request view, some keys may need to be deactivated
                        let (params_events_allowed, result_events_allowed) = match self.request_view {
                            RequestView::Normal => (true, true),
                            RequestView::OnlyResult => (false, true),
                            RequestView::OnlyParams => (true, false)
                        };

                        // Param tabs
                        if params_events_allowed {
                            match self.request_param_tab {
                                RequestParamsTabs::QueryParams => match key {
                                    key if key == key_bindings.generic.list_and_table_actions.edit_element && self.query_params_table.is_selected() => self.edit_request_param_state(),

                                    key if key == key_bindings.generic.navigation.move_cursor_up => self.query_params_table.up(),
                                    key if key == key_bindings.generic.navigation.move_cursor_down => self.query_params_table.down(),
                                    key if key == key_bindings.generic.navigation.move_cursor_left || key == key_bindings.generic.navigation.move_cursor_right => self.query_params_table.change_y(),

                                    key if key == key_bindings.generic.list_and_table_actions.create_element => self.create_new_query_param(),
                                    key if key == key_bindings.generic.list_and_table_actions.delete_element => self.delete_query_param(),
                                    key if key == key_bindings.generic.list_and_table_actions.toggle_element => self.toggle_query_param(),

                                    _ => {}
                                },
                                RequestParamsTabs::Auth if self.auth_text_input_selection.usable => match key {
                                    key if key == key_bindings.generic.list_and_table_actions.edit_element => self.select_request_auth_input_text(),

                                    key if key == key_bindings.generic.navigation.move_cursor_up => self.auth_text_input_selection.previous(),
                                    key if key == key_bindings.generic.navigation.move_cursor_down => self.auth_text_input_selection.next(),

                                    _ => {}
                                },
                                RequestParamsTabs::Headers => match key {
                                    key if key == key_bindings.generic.list_and_table_actions.edit_element && self.headers_table.is_selected() => self.edit_request_header_state(),

                                    key if key == key_bindings.generic.navigation.move_cursor_up => self.headers_table.up(),
                                    key if key == key_bindings.generic.navigation.move_cursor_down => self.headers_table.down(),
                                    key if key == key_bindings.generic.navigation.move_cursor_left || key == key_bindings.generic.navigation.move_cursor_right => self.headers_table.change_y(),

                                    key if key == key_bindings.generic.list_and_table_actions.create_element => self.create_new_header(),
                                    key if key == key_bindings.generic.list_and_table_actions.delete_element => self.delete_header(),
                                    key if key == key_bindings.generic.list_and_table_actions.toggle_element => self.toggle_header(),

                                    _ => {}
                                },
                                RequestParamsTabs::Body => match key {
                                    key if key == key_bindings.generic.list_and_table_actions.edit_element && self.body_form_table.is_selected() => self.edit_request_body_table_state(),
                                    key if key == key_bindings.generic.list_and_table_actions.edit_element => self.edit_request_body_file_or_string_state(),

                                    key if key == key_bindings.generic.navigation.move_cursor_up => self.body_form_table.up(),
                                    key if key == key_bindings.generic.navigation.move_cursor_down=> self.body_form_table.down(),
                                    key if key == key_bindings.generic.navigation.move_cursor_left || key == key_bindings.generic.navigation.move_cursor_right => self.body_form_table.change_y(),

                                    key if key == key_bindings.generic.list_and_table_actions.create_element => self.create_new_form_data(),
                                    key if key == key_bindings.generic.list_and_table_actions.delete_element => self.delete_form_data(),
                                    key if key == key_bindings.generic.list_and_table_actions.toggle_element => self.toggle_form_data(),

                                    _ => {}
                                },
                                _ => {}
                            }

                            match key {
                                //KeyCode::Char('p') => self.load_request_query_params_tab(),

                                key if key == key_bindings.request_selected.param_tabs.change_auth_method => self.modify_request_auth(),
                                //KeyCode::Char('a') => self.load_request_auth_param_tab(),

                                //KeyCode::Char('h') => self.load_request_headers_tab(),

                                key if key == key_bindings.request_selected.param_tabs.change_body_content_type => self.modify_request_content_type(),
                                //KeyCode::Char('b') => self.load_request_body_param_tab(),

                                key if key == key_bindings.request_selected.next_tab => self.next_request_param_tab(),

                                _ => {}
                            }
                        }

                        if result_events_allowed {
                            match key {
                                key if key == key_bindings.request_selected.result_tabs.scroll_up => self.result_vertical_scrollbar.page_up(),
                                key if key == key_bindings.request_selected.result_tabs.scroll_down => self.result_vertical_scrollbar.page_down(),
                                key if key == key_bindings.request_selected.result_tabs.scroll_left => self.result_horizontal_scrollbar.page_up(),
                                key if key == key_bindings.request_selected.result_tabs.scroll_right => self.result_horizontal_scrollbar.page_down(),

                                key if key == key_bindings.request_selected.next_tab && !params_events_allowed => self.next_request_result_tab(),
                                key if key == key_bindings.request_selected.result_tabs.secondary_next_tab && params_events_allowed => self.next_request_result_tab(),

                                _ => {}
                            }
                        }

                        match key {
                            key if key == key_bindings.generic.navigation.go_back => self.normal_state(),

                            key if key == key_bindings.main_menu.display_cookies => self.display_cookies_state(),
                            key if key == key_bindings.main_menu.next_environment => self.next_environment(),

                            key if key == key_bindings.main_menu.display_help => {}, // TODO

                            key if key == key_bindings.request_selected.change_url => self.edit_request_url_state(),
                            key if key == key_bindings.request_selected.change_method => self.modify_request_method(),

                            key if key == key_bindings.request_selected.request_settings => self.edit_request_settings_state(),

                            key if key == key_bindings.request_selected.next_view => self.next_request_view(),

                            key if key == key_bindings.request_selected.send_request => self.send_request().await,
                            key if key == key_bindings.request_selected.secondary_send_request => self.send_request().await,

                            _ => {}
                        }
                    },

                    AppState::EditingRequestUrl => match key {
                        key if key == key_bindings.generic.text_inputs.text_input.cancel => self.select_request_state(),
                        key if key == key_bindings.generic.text_inputs.text_input.validate => self.modify_request_url(),

                        key if key == key_bindings.generic.text_inputs.text_input.delete_backward => self.url_text_input.delete_char_backward(),
                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.url_text_input.delete_char_forward(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_left => self.url_text_input.move_cursor_left(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_right => self.url_text_input.move_cursor_right(),

                        KeyCombination { codes: One(KeyCode::Char(char)), modifiers: KeyModifiers::NONE } => self.url_text_input.enter_char(char),

                        _ => miss_input = true
                    },

                    AppState::EditingRequestParam => match key {
                        key if key == key_bindings.generic.text_inputs.text_input.cancel => self.select_request_state(),
                        key if key == key_bindings.generic.text_inputs.text_input.validate => self.modify_request_query_param(),

                        key if key == key_bindings.generic.text_inputs.text_input.delete_backward => self.query_params_table.selection_text_input.delete_char_backward(),
                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.query_params_table.selection_text_input.delete_char_forward(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_left => self.query_params_table.selection_text_input.move_cursor_left(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_right => self.query_params_table.selection_text_input.move_cursor_right(),

                        KeyCombination { codes: One(KeyCode::Char(char)), modifiers: KeyModifiers::NONE } => self.query_params_table.selection_text_input.enter_char(char),

                        _ => miss_input = true
                    }

                    AppState::EditingRequestAuthUsername => match key {
                        key if key == key_bindings.generic.text_inputs.text_input.cancel => self.select_request_state(),
                        key if key == key_bindings.generic.text_inputs.text_input.validate => self.modify_request_auth_basic_username(),

                        key if key == key_bindings.generic.text_inputs.text_input.delete_backward => self.auth_basic_username_text_input.delete_char_backward(),
                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.auth_basic_username_text_input.delete_char_forward(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_left => self.auth_basic_username_text_input.move_cursor_left(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_right => self.auth_basic_username_text_input.move_cursor_right(),

                        KeyCombination { codes: One(KeyCode::Char(char)), modifiers: KeyModifiers::NONE } => self.auth_basic_username_text_input.enter_char(char),

                        _ => miss_input = true
                    },

                    AppState::EditingRequestAuthPassword => match key {
                        key if key == key_bindings.generic.text_inputs.text_input.cancel => self.select_request_state(),
                        key if key == key_bindings.generic.text_inputs.text_input.validate => self.modify_request_auth_basic_password(),

                        key if key == key_bindings.generic.text_inputs.text_input.delete_backward => self.auth_basic_password_text_input.delete_char_backward(),
                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.auth_basic_password_text_input.delete_char_forward(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_left => self.auth_basic_password_text_input.move_cursor_left(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_right => self.auth_basic_password_text_input.move_cursor_right(),

                        KeyCombination { codes: One(KeyCode::Char(char)), modifiers: KeyModifiers::NONE } => self.auth_basic_password_text_input.enter_char(char),

                        _ => miss_input = true
                    },

                    AppState::EditingRequestAuthBearerToken => match key {
                        key if key == key_bindings.generic.text_inputs.text_input.cancel => self.select_request_state(),
                        key if key == key_bindings.generic.text_inputs.text_input.validate => self.modify_request_auth_bearer_token(),

                        key if key == key_bindings.generic.text_inputs.text_input.delete_backward => self.auth_bearer_token_text_input.delete_char_backward(),
                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.auth_bearer_token_text_input.delete_char_forward(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_left => self.auth_bearer_token_text_input.move_cursor_left(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_right => self.auth_bearer_token_text_input.move_cursor_right(),

                        KeyCombination { codes: One(KeyCode::Char(char)), modifiers: KeyModifiers::NONE } => self.auth_bearer_token_text_input.enter_char(char),

                        _ => miss_input = true
                    }

                    AppState::EditingRequestHeader => match key {
                        key if key == key_bindings.generic.text_inputs.text_input.cancel => self.select_request_state(),
                        key if key == key_bindings.generic.text_inputs.text_input.validate => self.modify_request_header(),

                        key if key == key_bindings.generic.text_inputs.text_input.delete_backward => self.headers_table.selection_text_input.delete_char_backward(),
                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.headers_table.selection_text_input.delete_char_forward(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_left => self.headers_table.selection_text_input.move_cursor_left(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_right => self.headers_table.selection_text_input.move_cursor_right(),

                        KeyCombination { codes: One(KeyCode::Char(char)), modifiers: KeyModifiers::NONE } => self.headers_table.selection_text_input.enter_char(char),

                        _ => miss_input = true
                    }

                    AppState::EditingRequestBodyTable => match key {
                        key if key == key_bindings.generic.text_inputs.text_input.cancel => self.select_request_state(),
                        key if key == key_bindings.generic.text_inputs.text_input.validate => self.modify_request_form_data(),

                        key if key == key_bindings.generic.text_inputs.text_input.delete_backward => self.body_form_table.selection_text_input.delete_char_backward(),
                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.body_form_table.selection_text_input.delete_char_forward(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_left => self.body_form_table.selection_text_input.move_cursor_left(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_right => self.body_form_table.selection_text_input.move_cursor_right(),

                        KeyCombination { codes: One(KeyCode::Char(char)), modifiers: KeyModifiers::NONE } => self.body_form_table.selection_text_input.enter_char(char),

                        _ => miss_input = true
                    }

                    AppState::EditingRequestBodyFile => match key {
                        key if key == key_bindings.generic.text_inputs.text_input.cancel => self.select_request_state(),
                        key if key == key_bindings.generic.text_inputs.text_input.validate => self.modify_request_body(),

                        key if key == key_bindings.generic.text_inputs.text_input.delete_backward => self.body_file_text_input.delete_char_backward(),
                        key if key == key_bindings.generic.text_inputs.text_input.delete_forward => self.body_file_text_input.delete_char_forward(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_left => self.body_file_text_input.move_cursor_left(),
                        key if key == key_bindings.generic.text_inputs.text_input.move_cursor_right => self.body_file_text_input.move_cursor_right(),

                        KeyCombination { codes: One(KeyCode::Char(char)), modifiers: KeyModifiers::NONE } => self.body_file_text_input.enter_char(char),

                        _ => miss_input = true
                    }

                    AppState::EditingRequestBodyString => match key_bindings.generic.text_inputs.text_area_mode {
                        // Vim Emulation mode
                        TextAreaMode::VimEmulation => match self.body_text_area_vim_emulation.transition(key, &mut self.body_text_area) {
                            VimTransition::Mode(mode) if self.body_text_area_vim_emulation.mode != mode => {
                                self.body_text_area.set_block(mode.block());
                                self.body_text_area.set_cursor_style(mode.cursor_style());
                                self.body_text_area_vim_emulation = Vim::new(mode);
                            }
                            VimTransition::Nop | VimTransition::Mode(_) => {
                                self.body_text_area_vim_emulation = self.body_text_area_vim_emulation.clone();
                            },
                            VimTransition::Pending(input) => {
                                self.body_text_area_vim_emulation = self.body_text_area_vim_emulation.clone().with_pending(input);
                            },
                            VimTransition::Quit => self.quit_request_body(),
                            VimTransition::SaveAndQuit => self.modify_request_body(),
                        },
                        // Custom text area key bindings
                        TextAreaMode::Custom(text_area_bindings) => match key {
                            key if key == text_area_bindings.copy => self.body_text_area.copy(),
                            key if key == text_area_bindings.paste => {self.body_text_area.paste();},

                            key if key == text_area_bindings.undo => {self.body_text_area.undo();},
                            key if key == text_area_bindings.redo => {self.body_text_area.redo();},

                            key if key == text_area_bindings.save_and_quit => self.modify_request_body(),

                            key if key == text_area_bindings.quit_without_saving => self.quit_request_body(),
                            key if key == text_area_bindings.new_line => self.body_text_area.insert_newline(),

                            key if key == text_area_bindings.indent => {
                                self.body_text_area.set_hard_tab_indent(true);
                                self.body_text_area.insert_tab();
                            },
                            key if key == text_area_bindings.backspace => {self.body_text_area.delete_char();},
                            key if key == text_area_bindings.delete => {self.body_text_area.delete_next_char();},

                            key if key == text_area_bindings.skip_word_cursor_right => self.body_text_area.move_cursor(CursorMove::WordForward),
                            key if key == text_area_bindings.skip_word_cursor_left => self.body_text_area.move_cursor(CursorMove::WordBack),

                            key if key == text_area_bindings.move_cursor_up => self.body_text_area.move_cursor(CursorMove::Up),
                            key if key == text_area_bindings.move_cursor_down => self.body_text_area.move_cursor(CursorMove::Bottom),
                            key if key == text_area_bindings.move_cursor_left => self.body_text_area.move_cursor(CursorMove::Back),
                            key if key == text_area_bindings.move_cursor_right => self.body_text_area.move_cursor(CursorMove::Forward),

                            KeyCombination { codes: One(KeyCode::Char(char)), modifiers: KeyModifiers::NONE } => self.body_text_area.insert_char(char),

                            _ => miss_input = true
                        }
                    }

                    AppState::EditingRequestSettings => match key {
                        key if key == key_bindings.generic.navigation.go_back => self.select_request_state(),

                        key if key == key_bindings.generic.navigation.select => self.modify_request_settings(),

                        key if key == key_bindings.generic.navigation.move_cursor_up => self.request_settings_popup.previous(),
                        key if key == key_bindings.generic.navigation.move_cursor_down => self.request_settings_popup.next(),
                        key if key == key_bindings.generic.navigation.move_cursor_left => self.request_settings_popup.toggle_setting(),
                        key if key == key_bindings.generic.navigation.move_cursor_right => self.request_settings_popup.toggle_setting(),

                        _ => miss_input = true
                    },
                    _ => {}
                }

                if !miss_input {
                    self.write_to_log_file(key.to_string(), previous_app_state.to_string());
                }
            }
        }
    }
}