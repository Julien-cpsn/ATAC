use crokey::KeyCombination;
use crokey::OneToThree::One;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use tui_textarea::CursorMove;

use crate::app::app::App;
use crate::app::app_states::AVAILABLE_EVENTS;
use crate::app::event_key_bindings::EventKeyBinding;
use crate::app::events::AppEvent::*;
use crate::app::files::key_bindings::KEY_BINDINGS;
use crate::utils::vim_emulation::{Vim, VimTransition};

#[derive(Clone)]
pub enum AppEvent {
    /* Main Page */

    ExitApp(EventKeyBinding),

    MoveCollectionCursorUp(EventKeyBinding),
    MoveCollectionCursorDown(EventKeyBinding),

    SelectRequestOrExpandCollection(EventKeyBinding),
    ExpandCollection(EventKeyBinding),
    UnselectRequest(EventKeyBinding),

    CreateElement(EventKeyBinding),
    DeleteElement(EventKeyBinding),
    RenameElement(EventKeyBinding),

    MoveRequestUp(EventKeyBinding),
    MoveRequestDown(EventKeyBinding),

    NextEnvironment(EventKeyBinding),
    DisplayCookies(EventKeyBinding),

    GoBackToMainMenu(EventKeyBinding),

    /* Cookies */

    CookiesMoveUp(EventKeyBinding),
    CookiesMoveDown(EventKeyBinding),
    CookiesMoveLeft(EventKeyBinding),
    CookiesMoveRight(EventKeyBinding),
    DeleteCookie(EventKeyBinding),

    /* Collections */

    ChooseElementToCreateMoveCursorLeft(EventKeyBinding),
    ChooseElementToCreateMoveCursorRight(EventKeyBinding),
    SelectElementToCreate(EventKeyBinding),

    CreateNewCollection(EventKeyBinding),
    CreatingCollectionDeleteCharBackward(EventKeyBinding),
    CreatingCollectionDeleteCharForward(EventKeyBinding),
    CreatingCollectionMoveCursorLeft(EventKeyBinding),
    CreatingCollectionMoveCursorRight(EventKeyBinding),
    CreatingCollectionCharInput(EventKeyBinding),

    CreateNewRequest(EventKeyBinding),
    CreatingRequestDeleteCharBackward(EventKeyBinding),
    CreatingRequestDeleteCharForward(EventKeyBinding),
    CreatingRequestMoveCursorLeft(EventKeyBinding),
    CreatingRequestMoveCursorRight(EventKeyBinding),
    CreatingRequestSelectCollectionUp(EventKeyBinding),
    CreatingRequestSelectCollectionDown(EventKeyBinding),
    CreatingRequestCharInput(EventKeyBinding),

    DeletingCollectionMoveCursorLeft(EventKeyBinding),
    DeletingCollectionMoveCursorRight(EventKeyBinding),
    DeleteCollection(EventKeyBinding),
    
    DeletingRequestMoveCursorLeft(EventKeyBinding),
    DeletingRequestMoveCursorRight(EventKeyBinding),
    DeleteRequest(EventKeyBinding),

    RenameCollection(EventKeyBinding),
    RenamingCollectionDeleteCharBackward(EventKeyBinding),
    RenamingCollectionDeleteCharForward(EventKeyBinding),
    RenamingCollectionMoveCursorLeft(EventKeyBinding),
    RenamingCollectionMoveCursorRight(EventKeyBinding),
    RenamingCollectionCharInput(EventKeyBinding),

    RenameRequest(EventKeyBinding),
    RenamingRequestDeleteCharBackward(EventKeyBinding),
    RenamingRequestDeleteCharForward(EventKeyBinding),
    RenamingRequestMoveCursorLeft(EventKeyBinding),
    RenamingRequestMoveCursorRight(EventKeyBinding),
    RenamingRequestCharInput(EventKeyBinding),
    
    /* Request */

    GoBackToRequestMenu(EventKeyBinding),

    EditUrl(EventKeyBinding),
    EditMethod(EventKeyBinding),

    EditSettings(EventKeyBinding),

    NextView(EventKeyBinding),

    SendRequest(EventKeyBinding),

    /* Param tabs */

    NextParamTab(EventKeyBinding),
    ModifyRequestAuthMethod(EventKeyBinding),
    ModifyRequestBodyContentType(EventKeyBinding),

    EditRequestQueryParam(EventKeyBinding),
    RequestQueryParamsMoveUp(EventKeyBinding),
    RequestQueryParamsMoveDown(EventKeyBinding),
    RequestQueryParamsMoveLeft(EventKeyBinding),
    RequestQueryParamsMoveRight(EventKeyBinding),
    CreateRequestQueryParam(EventKeyBinding),
    DeleteRequestQueryParam(EventKeyBinding),
    ToggleRequestQueryParam(EventKeyBinding),

    EditRequestAuth(EventKeyBinding),
    RequestAuthMoveUp(EventKeyBinding),
    RequestAuthMoveDown(EventKeyBinding),

    EditRequestHeader(EventKeyBinding),
    RequestHeadersMoveUp(EventKeyBinding),
    RequestHeadersMoveDown(EventKeyBinding),
    RequestHeadersMoveLeft(EventKeyBinding),
    RequestHeadersMoveRight(EventKeyBinding),
    CreateRequestHeader(EventKeyBinding),
    DeleteRequestHeader(EventKeyBinding),
    ToggleRequestHeader(EventKeyBinding),

    EditRequestBody(EventKeyBinding),
    RequestBodyTableMoveUp(EventKeyBinding),
    RequestBodyTableMoveDown(EventKeyBinding),
    RequestBodyTableMoveLeft(EventKeyBinding),
    RequestBodyTableMoveRight(EventKeyBinding),
    CreateRequestBodyTableElement(EventKeyBinding),
    DeleteRequestBodyTableElement(EventKeyBinding),
    ToggleRequestBodyTableElement(EventKeyBinding),

    EditRequestScript(EventKeyBinding),
    // Move up or down
    RequestScriptMove(EventKeyBinding),

    /* Result tabs */

    NextResultTab(EventKeyBinding),

    ScrollResultUp(EventKeyBinding),
    ScrollResultDown(EventKeyBinding),
    ScrollResultLeft(EventKeyBinding),
    ScrollResultRight(EventKeyBinding),

    /* Others */

    CopyResponsePart(EventKeyBinding),

    /* Request Text inputs */

    ModifyRequestUrl(EventKeyBinding),
    EditingRequestUrlDeleteCharBackward(EventKeyBinding),
    EditingRequestUrlDeleteCharForward(EventKeyBinding),
    EditingRequestUrlMoveCursorLeft(EventKeyBinding),
    EditingRequestUrlMoveCursorRight(EventKeyBinding),
    EditingRequestUrlCharInput(EventKeyBinding),

    ModifyRequestQueryParam(EventKeyBinding),
    EditingRequestQueryParamDeleteCharBackward(EventKeyBinding),
    EditingRequestQueryParamDeleteCharForward(EventKeyBinding),
    EditingRequestQueryParamMoveCursorLeft(EventKeyBinding),
    EditingRequestQueryParamMoveCursorRight(EventKeyBinding),
    EditingRequestQueryParamCharInput(EventKeyBinding),

    /* Auth */

    ModifyRequestAuthUsername(EventKeyBinding),
    EditingRequestAuthUsernameDeleteCharBackward(EventKeyBinding),
    EditingRequestAuthUsernameDeleteCharForward(EventKeyBinding),
    EditingRequestAuthUsernameMoveCursorLeft(EventKeyBinding),
    EditingRequestAuthUsernameMoveCursorRight(EventKeyBinding),
    EditingRequestAuthUsernameCharInput(EventKeyBinding),

    ModifyRequestAuthPassword(EventKeyBinding),
    EditingRequestAuthPasswordDeleteCharBackward(EventKeyBinding),
    EditingRequestAuthPasswordDeleteCharForward(EventKeyBinding),
    EditingRequestAuthPasswordMoveCursorLeft(EventKeyBinding),
    EditingRequestAuthPasswordMoveCursorRight(EventKeyBinding),
    EditingRequestAuthPasswordCharInput(EventKeyBinding),

    ModifyRequestAuthBearerToken(EventKeyBinding),
    EditingRequestAuthBearerTokenDeleteCharBackward(EventKeyBinding),
    EditingRequestAuthBearerTokenDeleteCharForward(EventKeyBinding),
    EditingRequestAuthBearerTokenMoveCursorLeft(EventKeyBinding),
    EditingRequestAuthBearerTokenMoveCursorRight(EventKeyBinding),
    EditingRequestAuthBearerTokenCharInput(EventKeyBinding),

    /* Headers */

    ModifyRequestHeader(EventKeyBinding),
    EditingRequestHeaderDeleteCharBackward(EventKeyBinding),
    EditingRequestHeaderDeleteCharForward(EventKeyBinding),
    EditingRequestHeaderMoveCursorLeft(EventKeyBinding),
    EditingRequestHeaderMoveCursorRight(EventKeyBinding),
    EditingRequestHeaderCharInput(EventKeyBinding),

    /* Body */

    ModifyRequestBodyTable(EventKeyBinding),
    EditingRequestBodyTableDeleteCharBackward(EventKeyBinding),
    EditingRequestBodyTableDeleteCharForward(EventKeyBinding),
    EditingRequestBodyTableMoveCursorLeft(EventKeyBinding),
    EditingRequestBodyTableMoveCursorRight(EventKeyBinding),
    EditingRequestBodyTableCharInput(EventKeyBinding),

    ModifyRequestBodyFile(EventKeyBinding),
    EditingRequestBodyFileDeleteCharBackward(EventKeyBinding),
    EditingRequestBodyFileDeleteCharForward(EventKeyBinding),
    EditingRequestBodyFileMoveCursorLeft(EventKeyBinding),
    EditingRequestBodyFileMoveCursorRight(EventKeyBinding),
    EditingRequestBodyFileCharInput(EventKeyBinding),

    EditingRequestBodyStringVimInput(EventKeyBinding),

    EditingRequestBodyStringSaveAndQuit(EventKeyBinding),
    EditingRequestBodyStringCopy(EventKeyBinding),
    EditingRequestBodyStringPaste(EventKeyBinding),
    EditingRequestBodyStringUndo(EventKeyBinding),
    EditingRequestBodyStringRedo(EventKeyBinding),
    EditingRequestBodyStringNewLine(EventKeyBinding),
    EditingRequestBodyStringIndent(EventKeyBinding),
    EditingRequestBodyStringDeleteCharBackward(EventKeyBinding),
    EditingRequestBodyStringDeleteCharForward(EventKeyBinding),
    EditingRequestBodyStringSkipWordLeft(EventKeyBinding),
    EditingRequestBodyStringSkipWordRight(EventKeyBinding),
    EditingRequestBodyStringMoveCursorUp(EventKeyBinding),
    EditingRequestBodyStringMoveCursorDown(EventKeyBinding),
    EditingRequestBodyStringMoveCursorLeft(EventKeyBinding),
    EditingRequestBodyStringMoveCursorRight(EventKeyBinding),
    EditingRequestBodyStringCharInput(EventKeyBinding),

    /* Scripts */

    EditingPreRequestScriptVimInput(EventKeyBinding),

    EditingPreRequestScriptSaveAndQuit(EventKeyBinding),
    EditingPreRequestScriptCopy(EventKeyBinding),
    EditingPreRequestScriptPaste(EventKeyBinding),
    EditingPreRequestScriptUndo(EventKeyBinding),
    EditingPreRequestScriptRedo(EventKeyBinding),
    EditingPreRequestScriptNewLine(EventKeyBinding),
    EditingPreRequestScriptIndent(EventKeyBinding),
    EditingPreRequestScriptDeleteCharBackward(EventKeyBinding),
    EditingPreRequestScriptDeleteCharForward(EventKeyBinding),
    EditingPreRequestScriptSkipWordLeft(EventKeyBinding),
    EditingPreRequestScriptSkipWordRight(EventKeyBinding),
    EditingPreRequestScriptMoveCursorUp(EventKeyBinding),
    EditingPreRequestScriptMoveCursorDown(EventKeyBinding),
    EditingPreRequestScriptMoveCursorLeft(EventKeyBinding),
    EditingPreRequestScriptMoveCursorRight(EventKeyBinding),
    EditingPreRequestScriptCharInput(EventKeyBinding),

    EditingPostRequestScriptVimInput(EventKeyBinding),

    EditingPostRequestScriptSaveAndQuit(EventKeyBinding),
    EditingPostRequestScriptCopy(EventKeyBinding),
    EditingPostRequestScriptPaste(EventKeyBinding),
    EditingPostRequestScriptUndo(EventKeyBinding),
    EditingPostRequestScriptRedo(EventKeyBinding),
    EditingPostRequestScriptNewLine(EventKeyBinding),
    EditingPostRequestScriptIndent(EventKeyBinding),
    EditingPostRequestScriptDeleteCharBackward(EventKeyBinding),
    EditingPostRequestScriptDeleteCharForward(EventKeyBinding),
    EditingPostRequestScriptSkipWordLeft(EventKeyBinding),
    EditingPostRequestScriptSkipWordRight(EventKeyBinding),
    EditingPostRequestScriptMoveCursorUp(EventKeyBinding),
    EditingPostRequestScriptMoveCursorDown(EventKeyBinding),
    EditingPostRequestScriptMoveCursorLeft(EventKeyBinding),
    EditingPostRequestScriptMoveCursorRight(EventKeyBinding),
    EditingPostRequestScriptCharInput(EventKeyBinding),
    
    /* Settings */

    RequestSettingsMoveUp(EventKeyBinding),
    RequestSettingsMoveDown(EventKeyBinding),
    RequestSettingsToggleSetting(EventKeyBinding),
    ModifyRequestSettings(EventKeyBinding),

    /* Others */

    Documentation(EventKeyBinding)
}

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

                let previous_app_state = self.state;
                let key = KeyCombination::from(key_event);

                let is_input_missed = self.handle_key(key).await;

                if !is_input_missed {
                    self.write_to_log_file(key.to_string(), previous_app_state.to_string());
                }
            }
        }
    }

    async fn handle_key(&mut self, key: KeyCombination) -> bool {
        // Debug tool
        //dbg!("{}", key.to_string());

        {
            let key_bindings = KEY_BINDINGS.read().unwrap();

            // Help is being displayed
            if self.should_display_help {
                match key {
                    key if key == key_bindings.generic.navigation.go_back => self.should_display_help = false,

                    key if key == key_bindings.generic.navigation.move_cursor_left => self.help_popup.previous_page(),
                    key if key == key_bindings.generic.navigation.move_cursor_right => self.help_popup.next_page(),

                    _ => {}
                }

                // Avoid triggering other keys
                return false;
            }
            // Help is not being displayed
            else if key == key_bindings.generic.display_help {
                self.should_display_help = true;
                self.help_popup.selection = self.state;
                return false;
            }
        }

        let mut miss_input = false;
        let mut matching_event: Option<&AppEvent> = None;

        let available_app_events = AVAILABLE_EVENTS.read().unwrap();

        for possible_event in available_app_events.iter() {
            let event_key_bindings = possible_event.get_event_key_bindings();

            // Either the key is contained in the trigger condition list OR if the list is empty and no modifiers has been pressed, means 'any char'
            if event_key_bindings.keys.contains(&key) || event_key_bindings.keys.is_empty() {
                matching_event = Some(possible_event);
                break;
            }
        }

        match matching_event {
            None => miss_input = true,
            Some(event) => match event {
                /* Main menu */

                ExitApp(_) => self.should_quit = true,

                MoveCollectionCursorUp(_) => self.collections_tree.up(),
                MoveCollectionCursorDown(_) => self.collections_tree.down(),

                SelectRequestOrExpandCollection(_) => self.select_request_or_expand_collection(),
                ExpandCollection(_) => { self.collections_tree.state.toggle_selected(); },
                UnselectRequest(_) => self.unselect_request(),

                CreateElement(_) => self.choose_element_to_create_state(),
                DeleteElement(_) => self.delete_element(),
                RenameElement(_) => self.rename_element(),

                MoveRequestUp(_) => self.move_request_up(),
                MoveRequestDown(_) => self.move_request_down(),

                NextEnvironment(_) => self.next_environment(),
                DisplayCookies(_) => self.display_cookies_state(),

                GoBackToMainMenu(_) => self.normal_state(),

                /* Cookies */

                CookiesMoveUp(_) => self.cookies_popup.cookies_table.up(),
                CookiesMoveDown(_) => self.cookies_popup.cookies_table.down(),
                CookiesMoveLeft(_) => self.cookies_popup.cookies_table.left(),
                CookiesMoveRight(_) => self.cookies_popup.cookies_table.right(),

                DeleteCookie(_) => self.delete_cookie(),

                /* Collections */

                ChooseElementToCreateMoveCursorLeft(_) => self.creation_popup.previous(),
                ChooseElementToCreateMoveCursorRight(_) => self.creation_popup.next(),
                SelectElementToCreate(_) => self.new_element(),

                CreateNewCollection(_) => self.new_collection(),
                CreatingCollectionDeleteCharBackward(_) => self.new_collection_input.delete_char_forward(),
                CreatingCollectionDeleteCharForward(_) => self.new_collection_input.delete_char_backward(),
                CreatingCollectionMoveCursorLeft(_) => self.new_collection_input.move_cursor_left(),
                CreatingCollectionMoveCursorRight(_) => self.new_collection_input.move_cursor_right(),
                CreatingCollectionCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.new_collection_input.enter_char(char),
                    _ => {}
                },

                CreateNewRequest(_) => self.new_request(),
                CreatingRequestDeleteCharBackward(_) => self.new_request_popup.text_input.delete_char_forward(),
                CreatingRequestDeleteCharForward(_) => self.new_request_popup.text_input.delete_char_backward(),
                CreatingRequestMoveCursorLeft(_) => self.new_request_popup.text_input.move_cursor_left(),
                CreatingRequestMoveCursorRight(_) => self.new_request_popup.text_input.move_cursor_right(),
                CreatingRequestSelectCollectionUp(_) => self.new_request_popup.previous_collection(),
                CreatingRequestSelectCollectionDown(_) => self.new_request_popup.next_collection(),
                CreatingRequestCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.new_request_popup.text_input.enter_char(char),
                    _ => {}
                },

                DeletingCollectionMoveCursorLeft(_) => self.delete_collection_popup.change_state(),
                DeletingCollectionMoveCursorRight(_) => self.delete_collection_popup.change_state(),
                DeleteCollection(_) => match self.delete_collection_popup.state {
                    true => self.delete_collection(),
                    false => self.normal_state(),
                },
                
                DeletingRequestMoveCursorLeft(_) => self.delete_request_popup.change_state(),
                DeletingRequestMoveCursorRight(_) => self.delete_request_popup.change_state(),
                DeleteRequest(_) => match self.delete_request_popup.state {
                    true => self.delete_request(),
                    false => self.normal_state(),
                },

                RenameCollection(_) => self.rename_collection(),
                RenamingCollectionDeleteCharBackward(_) => self.rename_collection_input.delete_char_forward(),
                RenamingCollectionDeleteCharForward(_) => self.rename_collection_input.delete_char_backward(),
                RenamingCollectionMoveCursorLeft(_) => self.rename_collection_input.move_cursor_left(),
                RenamingCollectionMoveCursorRight(_) => self.rename_collection_input.move_cursor_right(),
                RenamingCollectionCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.rename_collection_input.enter_char(char),
                    _ => {}
                },
                
                RenameRequest(_) => self.rename_request(),
                RenamingRequestDeleteCharBackward(_) => self.rename_request_input.delete_char_forward(),
                RenamingRequestDeleteCharForward(_) => self.rename_request_input.delete_char_backward(),
                RenamingRequestMoveCursorLeft(_) => self.rename_request_input.move_cursor_left(),
                RenamingRequestMoveCursorRight(_) => self.rename_request_input.move_cursor_right(),
                RenamingRequestCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.rename_request_input.enter_char(char),
                    _ => {}
                },
                
                /* Selected Request */

                GoBackToRequestMenu(_) => self.select_request_state(),
                
                EditUrl(_) => self.edit_request_url_state(),
                EditMethod(_) => self.modify_request_method(),
                EditSettings(_) => self.edit_request_settings_state(),

                NextView(_) => self.next_request_view(),
                SendRequest(_) => self.send_request().await,

                /* Param tabs */

                NextParamTab(_) => self.next_request_param_tab(),

                ModifyRequestAuthMethod(_) => self.modify_request_auth(),
                ModifyRequestBodyContentType(_) => self.modify_request_content_type(),

                EditRequestQueryParam(_) => match self.query_params_table.is_selected() {
                    true => self.edit_request_param_state(),
                    false => {}
                },
                RequestQueryParamsMoveUp(_) => self.query_params_table.up(),
                RequestQueryParamsMoveDown(_) => self.query_params_table.down(),
                RequestQueryParamsMoveLeft(_) | RequestQueryParamsMoveRight(_) => self.query_params_table.change_y(),
                CreateRequestQueryParam(_) => self.create_new_query_param(),
                DeleteRequestQueryParam(_) => self.delete_query_param(),
                ToggleRequestQueryParam(_) => self.toggle_query_param(),

                EditRequestAuth(_) => match self.auth_text_input_selection.usable {
                    true => self.select_request_auth_input_text(),
                    false => {}
                },
                RequestAuthMoveUp(_) => match self.auth_text_input_selection.usable {
                    true =>self.auth_text_input_selection.previous(),
                    false => {}
                },
                RequestAuthMoveDown(_) => match self.auth_text_input_selection.usable {
                    true => self.auth_text_input_selection.next(),
                    false => {}
                },

                EditRequestHeader(_) => match self.headers_table.is_selected() {
                    true => self.edit_request_header_state(),
                    false => {}
                },
                RequestHeadersMoveUp(_) => self.headers_table.up(),
                RequestHeadersMoveDown(_) => self.headers_table.down(),
                RequestHeadersMoveLeft(_) | RequestHeadersMoveRight(_) => self.headers_table.change_y(),
                CreateRequestHeader(_) => self.create_new_header(),
                DeleteRequestHeader(_) => self.delete_header(),
                ToggleRequestHeader(_) => self.toggle_header(),

                EditRequestBody(_) => match self.body_form_table.is_selected() {
                    true => self.edit_request_body_table_state(),
                    false => self.edit_request_body_file_or_string_state(),
                },
                RequestBodyTableMoveUp(_) => self.body_form_table.up(),
                RequestBodyTableMoveDown(_) => self.body_form_table.down(),
                RequestBodyTableMoveLeft(_) | RequestBodyTableMoveRight(_) => self.body_form_table.change_y(),
                CreateRequestBodyTableElement(_) => self.create_new_form_data(),
                DeleteRequestBodyTableElement(_) => self.delete_form_data(),
                ToggleRequestBodyTableElement(_) => self.toggle_form_data(),

                /* Scripts */

                EditRequestScript(_) => self.edit_request_script_state(),
                RequestScriptMove(_) => self.script_console.change_selection(),

                /* Result tabs */

                NextResultTab(_) => self.next_request_result_tab(),

                ScrollResultUp(_) => self.result_vertical_scrollbar.page_up(),
                ScrollResultDown(_) => self.result_vertical_scrollbar.page_down(),
                ScrollResultLeft(_) => self.result_horizontal_scrollbar.page_up(),
                ScrollResultRight(_) => self.result_horizontal_scrollbar.page_down(),

                /* Others */

                CopyResponsePart(_) => self.copy_response_body_content_to_clipboard(),

                /* Request text inputs */

                ModifyRequestUrl(_) => self.modify_request_url(),
                EditingRequestUrlDeleteCharBackward(_) => self.url_text_input.delete_char_forward(),
                EditingRequestUrlDeleteCharForward(_) => self.url_text_input.delete_char_backward(),
                EditingRequestUrlMoveCursorLeft(_) => self.url_text_input.move_cursor_left(),
                EditingRequestUrlMoveCursorRight(_) => self.url_text_input.move_cursor_right(),
                EditingRequestUrlCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.url_text_input.enter_char(char),
                    _ => {}
                },

                ModifyRequestQueryParam(_) => self.modify_request_query_param(),
                EditingRequestQueryParamDeleteCharBackward(_) => self.query_params_table.selection_text_input.delete_char_forward(),
                EditingRequestQueryParamDeleteCharForward(_) => self.query_params_table.selection_text_input.delete_char_backward(),
                EditingRequestQueryParamMoveCursorLeft(_) => self.query_params_table.selection_text_input.move_cursor_left(),
                EditingRequestQueryParamMoveCursorRight(_) => self.query_params_table.selection_text_input.move_cursor_right(),
                EditingRequestQueryParamCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.query_params_table.selection_text_input.enter_char(char),
                    _ => {}
                },

                /* Auth */
                
                // self.auth_text_input_selection.usable

                ModifyRequestAuthUsername(_) => self.modify_request_auth_basic_username(),
                EditingRequestAuthUsernameDeleteCharBackward(_) => self.auth_basic_username_text_input.delete_char_forward(),
                EditingRequestAuthUsernameDeleteCharForward(_) => self.auth_basic_username_text_input.delete_char_backward(),
                EditingRequestAuthUsernameMoveCursorLeft(_) => self.auth_basic_username_text_input.move_cursor_left(),
                EditingRequestAuthUsernameMoveCursorRight(_) => self.auth_basic_username_text_input.move_cursor_right(),
                EditingRequestAuthUsernameCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.auth_basic_username_text_input.enter_char(char),
                    _ => {}
                },
                
                ModifyRequestAuthPassword(_) => self.modify_request_auth_basic_password(),
                EditingRequestAuthPasswordDeleteCharBackward(_) => self.auth_basic_password_text_input.delete_char_forward(),
                EditingRequestAuthPasswordDeleteCharForward(_) => self.auth_basic_password_text_input.delete_char_backward(),
                EditingRequestAuthPasswordMoveCursorLeft(_) => self.auth_basic_password_text_input.move_cursor_left(),
                EditingRequestAuthPasswordMoveCursorRight(_) => self.auth_basic_password_text_input.move_cursor_right(),
                EditingRequestAuthPasswordCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.auth_basic_password_text_input.enter_char(char),
                    _ => {}
                },
                
                ModifyRequestAuthBearerToken(_) => self.modify_request_auth_bearer_token(),
                EditingRequestAuthBearerTokenDeleteCharBackward(_) => self.auth_bearer_token_text_input.delete_char_forward(),
                EditingRequestAuthBearerTokenDeleteCharForward(_) => self.auth_bearer_token_text_input.delete_char_backward(),
                EditingRequestAuthBearerTokenMoveCursorLeft(_) => self.auth_bearer_token_text_input.move_cursor_left(),
                EditingRequestAuthBearerTokenMoveCursorRight(_) => self.auth_bearer_token_text_input.move_cursor_right(),
                EditingRequestAuthBearerTokenCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.auth_bearer_token_text_input.enter_char(char),
                    _ => {}
                },

                /* Header */

                ModifyRequestHeader(_) => self.modify_request_header(),
                EditingRequestHeaderDeleteCharBackward(_) => self.headers_table.selection_text_input.delete_char_forward(),
                EditingRequestHeaderDeleteCharForward(_) => self.headers_table.selection_text_input.delete_char_backward(),
                EditingRequestHeaderMoveCursorLeft(_) => self.headers_table.selection_text_input.move_cursor_left(),
                EditingRequestHeaderMoveCursorRight(_) => self.headers_table.selection_text_input.move_cursor_right(),
                EditingRequestHeaderCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.headers_table.selection_text_input.enter_char(char),
                    _ => {}
                },

                /* Body */

                ModifyRequestBodyTable(_) => self.modify_request_form_data(),
                EditingRequestBodyTableDeleteCharBackward(_) => self.body_form_table.selection_text_input.delete_char_forward(),
                EditingRequestBodyTableDeleteCharForward(_) => self.body_form_table.selection_text_input.delete_char_backward(),
                EditingRequestBodyTableMoveCursorLeft(_) => self.body_form_table.selection_text_input.move_cursor_left(),
                EditingRequestBodyTableMoveCursorRight(_) => self.body_form_table.selection_text_input.move_cursor_right(),
                EditingRequestBodyTableCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.body_form_table.selection_text_input.enter_char(char),
                    _ => {}
                },
                
                ModifyRequestBodyFile(_) => self.modify_request_body(),
                EditingRequestBodyFileDeleteCharBackward(_) => self.body_file_text_input.delete_char_forward(),
                EditingRequestBodyFileDeleteCharForward(_) => self.body_file_text_input.delete_char_backward(),
                EditingRequestBodyFileMoveCursorLeft(_) => self.body_file_text_input.move_cursor_left(),
                EditingRequestBodyFileMoveCursorRight(_) => self.body_file_text_input.move_cursor_right(),
                EditingRequestBodyFileCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.body_file_text_input.enter_char(char),
                    _ => {}
                },

                EditingRequestBodyStringVimInput(_) => match self.body_text_area_vim_emulation.transition(key, &mut self.body_text_area) {
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
                    VimTransition::Quit => self.select_request_state(),
                    VimTransition::SaveAndQuit => self.modify_request_body(),
                },

                EditingRequestBodyStringSaveAndQuit(_) => self.modify_request_body(),
                EditingRequestBodyStringCopy(_) => self.body_text_area.copy(),
                EditingRequestBodyStringPaste(_) => {self.body_text_area.paste();},
                EditingRequestBodyStringUndo(_) => {self.body_text_area.undo();},
                EditingRequestBodyStringRedo(_) => {self.body_text_area.redo();},
                EditingRequestBodyStringNewLine(_) => self.body_text_area.insert_newline(),
                EditingRequestBodyStringIndent(_) => {
                    self.body_text_area.set_hard_tab_indent(true);
                    self.body_text_area.insert_tab();
                },
                EditingRequestBodyStringDeleteCharBackward(_) => {self.body_text_area.delete_next_char();},
                EditingRequestBodyStringDeleteCharForward(_) => {self.body_text_area.delete_char();},
                EditingRequestBodyStringSkipWordLeft(_) => self.body_text_area.move_cursor(CursorMove::WordBack),
                EditingRequestBodyStringSkipWordRight(_) => self.body_text_area.move_cursor(CursorMove::WordForward),
                EditingRequestBodyStringMoveCursorUp(_) => self.body_text_area.move_cursor(CursorMove::Up),
                EditingRequestBodyStringMoveCursorDown(_) => self.body_text_area.move_cursor(CursorMove::Bottom),
                EditingRequestBodyStringMoveCursorLeft(_) => self.body_text_area.move_cursor(CursorMove::Back),
                EditingRequestBodyStringMoveCursorRight(_) => self.body_text_area.move_cursor(CursorMove::Forward),
                EditingRequestBodyStringCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.body_text_area.insert_char(char),
                    _ => {}
                },
                
                /* Scripts */

                EditingPreRequestScriptVimInput(_) => match self.script_console.vim_emulation.transition(key, &mut self.script_console.pre_request_text_area) {
                    VimTransition::Mode(mode) if self.script_console.vim_emulation.mode != mode => {
                        self.script_console.pre_request_text_area.set_block(mode.block());
                        self.script_console.pre_request_text_area.set_cursor_style(mode.cursor_style());
                        self.script_console.vim_emulation = Vim::new(mode);
                    }
                    VimTransition::Nop | VimTransition::Mode(_) => {
                        self.script_console.vim_emulation = self.script_console.vim_emulation.clone();
                    },
                    VimTransition::Pending(input) => {
                        self.script_console.vim_emulation = self.script_console.vim_emulation.clone().with_pending(input);
                    },
                    VimTransition::Quit => self.select_request_state(),
                    VimTransition::SaveAndQuit => self.modify_pre_request_script(),
                },

                EditingPreRequestScriptSaveAndQuit(_) => self.modify_pre_request_script(),
                EditingPreRequestScriptCopy(_) => self.script_console.pre_request_text_area.copy(),
                EditingPreRequestScriptPaste(_) => {self.script_console.pre_request_text_area.paste();},
                EditingPreRequestScriptUndo(_) => {self.script_console.pre_request_text_area.undo();},
                EditingPreRequestScriptRedo(_) => {self.script_console.pre_request_text_area.redo();},
                EditingPreRequestScriptNewLine(_) => self.script_console.pre_request_text_area.insert_newline(),
                EditingPreRequestScriptIndent(_) => {
                    self.script_console.pre_request_text_area.set_hard_tab_indent(true);
                    self.script_console.pre_request_text_area.insert_tab();
                },
                EditingPreRequestScriptDeleteCharBackward(_) => {self.script_console.pre_request_text_area.delete_next_char();},
                EditingPreRequestScriptDeleteCharForward(_) => {self.script_console.pre_request_text_area.delete_char();},
                EditingPreRequestScriptSkipWordLeft(_) => self.script_console.pre_request_text_area.move_cursor(CursorMove::WordBack),
                EditingPreRequestScriptSkipWordRight(_) => self.script_console.pre_request_text_area.move_cursor(CursorMove::WordForward),
                EditingPreRequestScriptMoveCursorUp(_) => self.script_console.pre_request_text_area.move_cursor(CursorMove::Up),
                EditingPreRequestScriptMoveCursorDown(_) => self.script_console.pre_request_text_area.move_cursor(CursorMove::Bottom),
                EditingPreRequestScriptMoveCursorLeft(_) => self.script_console.pre_request_text_area.move_cursor(CursorMove::Back),
                EditingPreRequestScriptMoveCursorRight(_) => self.script_console.pre_request_text_area.move_cursor(CursorMove::Forward),
                EditingPreRequestScriptCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.script_console.pre_request_text_area.insert_char(char),
                    _ => {}
                },

                EditingPostRequestScriptVimInput(_) => match self.script_console.vim_emulation.transition(key, &mut self.script_console.post_request_text_area) {
                    VimTransition::Mode(mode) if self.script_console.vim_emulation.mode != mode => {
                        self.script_console.post_request_text_area.set_block(mode.block());
                        self.script_console.post_request_text_area.set_cursor_style(mode.cursor_style());
                        self.script_console.vim_emulation = Vim::new(mode);
                    }
                    VimTransition::Nop | VimTransition::Mode(_) => {
                        self.script_console.vim_emulation = self.script_console.vim_emulation.clone();
                    },
                    VimTransition::Pending(input) => {
                        self.script_console.vim_emulation = self.script_console.vim_emulation.clone().with_pending(input);
                    },
                    VimTransition::Quit => self.select_request_state(),
                    VimTransition::SaveAndQuit => self.modify_post_request_script(),
                },

                EditingPostRequestScriptSaveAndQuit(_) => self.modify_post_request_script(),
                EditingPostRequestScriptCopy(_) => self.script_console.post_request_text_area.copy(),
                EditingPostRequestScriptPaste(_) => {self.script_console.post_request_text_area.paste();},
                EditingPostRequestScriptUndo(_) => {self.script_console.post_request_text_area.undo();},
                EditingPostRequestScriptRedo(_) => {self.script_console.post_request_text_area.redo();},
                EditingPostRequestScriptNewLine(_) => self.script_console.post_request_text_area.insert_newline(),
                EditingPostRequestScriptIndent(_) => {
                    self.script_console.post_request_text_area.set_hard_tab_indent(true);
                    self.script_console.post_request_text_area.insert_tab();
                },
                EditingPostRequestScriptDeleteCharBackward(_) => {self.script_console.post_request_text_area.delete_next_char();},
                EditingPostRequestScriptDeleteCharForward(_) => {self.script_console.post_request_text_area.delete_char();},
                EditingPostRequestScriptSkipWordLeft(_) => self.script_console.post_request_text_area.move_cursor(CursorMove::WordBack),
                EditingPostRequestScriptSkipWordRight(_) => self.script_console.post_request_text_area.move_cursor(CursorMove::WordForward),
                EditingPostRequestScriptMoveCursorUp(_) => self.script_console.post_request_text_area.move_cursor(CursorMove::Up),
                EditingPostRequestScriptMoveCursorDown(_) => self.script_console.post_request_text_area.move_cursor(CursorMove::Bottom),
                EditingPostRequestScriptMoveCursorLeft(_) => self.script_console.post_request_text_area.move_cursor(CursorMove::Back),
                EditingPostRequestScriptMoveCursorRight(_) => self.script_console.post_request_text_area.move_cursor(CursorMove::Forward),
                EditingPostRequestScriptCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.script_console.post_request_text_area.insert_char(char),
                    _ => {}
                },
                
                /* Settings */

                RequestSettingsMoveUp(_) => self.request_settings_popup.previous(),
                RequestSettingsMoveDown(_) => self.request_settings_popup.next(),
                RequestSettingsToggleSetting(_) => self.request_settings_popup.toggle_setting(),
                ModifyRequestSettings(_) => self.modify_request_settings(),

                /* Others */

                Documentation(_) => {}
            }
        };

        return miss_input;
    }
}

impl AppEvent {
    pub fn get_event_key_bindings(&self) -> &EventKeyBinding {
        match self {
            ExitApp(event_key_bindings) |
            MoveCollectionCursorUp(event_key_bindings) |
            MoveCollectionCursorDown(event_key_bindings) |
            SelectRequestOrExpandCollection(event_key_bindings) |
            ExpandCollection(event_key_bindings) |
            UnselectRequest(event_key_bindings) |
            CreateElement(event_key_bindings) |
            DeleteElement(event_key_bindings) |
            RenameElement(event_key_bindings) |
            MoveRequestUp(event_key_bindings) |
            MoveRequestDown(event_key_bindings) |
            NextEnvironment(event_key_bindings) |
            DisplayCookies(event_key_bindings) |
            GoBackToMainMenu(event_key_bindings) |
            CookiesMoveUp(event_key_bindings) |
            CookiesMoveDown(event_key_bindings) |
            CookiesMoveLeft(event_key_bindings) |
            CookiesMoveRight(event_key_bindings) |
            DeleteCookie(event_key_bindings) |
            ChooseElementToCreateMoveCursorLeft(event_key_bindings) |
            ChooseElementToCreateMoveCursorRight(event_key_bindings) |
            SelectElementToCreate(event_key_bindings) |
            CreateNewCollection(event_key_bindings) |
            CreatingCollectionDeleteCharBackward(event_key_bindings) |
            CreatingCollectionDeleteCharForward(event_key_bindings) |
            CreatingCollectionMoveCursorLeft(event_key_bindings) |
            CreatingCollectionMoveCursorRight(event_key_bindings) |
            CreatingCollectionCharInput(event_key_bindings) |
            CreateNewRequest(event_key_bindings) |
            CreatingRequestDeleteCharBackward(event_key_bindings) |
            CreatingRequestDeleteCharForward(event_key_bindings) |
            CreatingRequestMoveCursorLeft(event_key_bindings) |
            CreatingRequestMoveCursorRight(event_key_bindings) |
            CreatingRequestSelectCollectionUp(event_key_bindings) |
            CreatingRequestSelectCollectionDown(event_key_bindings) |
            CreatingRequestCharInput(event_key_bindings) |
            DeletingCollectionMoveCursorLeft(event_key_bindings) |
            DeletingCollectionMoveCursorRight(event_key_bindings) |
            DeleteCollection(event_key_bindings) |
            DeletingRequestMoveCursorLeft(event_key_bindings) |
            DeletingRequestMoveCursorRight(event_key_bindings) |
            DeleteRequest(event_key_bindings) |
            RenameCollection(event_key_bindings) |
            RenamingCollectionDeleteCharBackward(event_key_bindings) |
            RenamingCollectionDeleteCharForward(event_key_bindings) |
            RenamingCollectionMoveCursorLeft(event_key_bindings) |
            RenamingCollectionMoveCursorRight(event_key_bindings) |
            RenamingCollectionCharInput(event_key_bindings) |
            RenameRequest(event_key_bindings) |
            RenamingRequestDeleteCharBackward(event_key_bindings) |
            RenamingRequestDeleteCharForward(event_key_bindings) |
            RenamingRequestMoveCursorLeft(event_key_bindings) |
            RenamingRequestMoveCursorRight(event_key_bindings) |
            RenamingRequestCharInput(event_key_bindings) |
            GoBackToRequestMenu(event_key_bindings) |
            EditUrl(event_key_bindings) |
            EditMethod(event_key_bindings) |
            EditSettings(event_key_bindings) |
            NextView(event_key_bindings) |
            SendRequest(event_key_bindings) |
            NextParamTab(event_key_bindings) |
            ModifyRequestAuthMethod(event_key_bindings) |
            ModifyRequestBodyContentType(event_key_bindings) |
            EditRequestQueryParam(event_key_bindings) |
            RequestQueryParamsMoveUp(event_key_bindings) |
            RequestQueryParamsMoveDown(event_key_bindings) |
            RequestQueryParamsMoveLeft(event_key_bindings) |
            RequestQueryParamsMoveRight(event_key_bindings) |
            CreateRequestQueryParam(event_key_bindings) |
            DeleteRequestQueryParam(event_key_bindings) |
            ToggleRequestQueryParam(event_key_bindings) |
            EditRequestAuth(event_key_bindings) |
            RequestAuthMoveUp(event_key_bindings) |
            RequestAuthMoveDown(event_key_bindings) |
            EditRequestHeader(event_key_bindings) |
            RequestHeadersMoveUp(event_key_bindings) |
            RequestHeadersMoveDown(event_key_bindings) |
            RequestHeadersMoveLeft(event_key_bindings) |
            RequestHeadersMoveRight(event_key_bindings) |
            CreateRequestHeader(event_key_bindings) |
            DeleteRequestHeader(event_key_bindings) |
            ToggleRequestHeader(event_key_bindings) |
            EditRequestBody(event_key_bindings) |
            RequestBodyTableMoveUp(event_key_bindings) |
            RequestBodyTableMoveDown(event_key_bindings) |
            RequestBodyTableMoveLeft(event_key_bindings) |
            RequestBodyTableMoveRight(event_key_bindings) |
            CreateRequestBodyTableElement(event_key_bindings) |
            DeleteRequestBodyTableElement(event_key_bindings) |
            ToggleRequestBodyTableElement(event_key_bindings) |
            EditRequestScript(event_key_bindings) |
            RequestScriptMove(event_key_bindings) |
            NextResultTab(event_key_bindings) |
            ScrollResultUp(event_key_bindings) |
            ScrollResultDown(event_key_bindings) |
            ScrollResultLeft(event_key_bindings) |
            ScrollResultRight(event_key_bindings) |
            CopyResponsePart(event_key_bindings) |
            ModifyRequestUrl(event_key_bindings) |
            EditingRequestUrlDeleteCharBackward(event_key_bindings) |
            EditingRequestUrlDeleteCharForward(event_key_bindings) |
            EditingRequestUrlMoveCursorLeft(event_key_bindings) |
            EditingRequestUrlMoveCursorRight(event_key_bindings) |
            EditingRequestUrlCharInput(event_key_bindings) |
            ModifyRequestQueryParam(event_key_bindings) |
            EditingRequestQueryParamDeleteCharBackward(event_key_bindings) |
            EditingRequestQueryParamDeleteCharForward(event_key_bindings) |
            EditingRequestQueryParamMoveCursorLeft(event_key_bindings) |
            EditingRequestQueryParamMoveCursorRight(event_key_bindings) |
            EditingRequestQueryParamCharInput(event_key_bindings) |
            ModifyRequestAuthUsername(event_key_bindings) |
            EditingRequestAuthUsernameDeleteCharBackward(event_key_bindings) |
            EditingRequestAuthUsernameDeleteCharForward(event_key_bindings) |
            EditingRequestAuthUsernameMoveCursorLeft(event_key_bindings) |
            EditingRequestAuthUsernameMoveCursorRight(event_key_bindings) |
            EditingRequestAuthUsernameCharInput(event_key_bindings) |
            ModifyRequestAuthPassword(event_key_bindings) |
            EditingRequestAuthPasswordDeleteCharBackward(event_key_bindings) |
            EditingRequestAuthPasswordDeleteCharForward(event_key_bindings) |
            EditingRequestAuthPasswordMoveCursorLeft(event_key_bindings) |
            EditingRequestAuthPasswordMoveCursorRight(event_key_bindings) |
            EditingRequestAuthPasswordCharInput(event_key_bindings) |
            ModifyRequestAuthBearerToken(event_key_bindings) |
            EditingRequestAuthBearerTokenDeleteCharBackward(event_key_bindings) |
            EditingRequestAuthBearerTokenDeleteCharForward(event_key_bindings) |
            EditingRequestAuthBearerTokenMoveCursorLeft(event_key_bindings) |
            EditingRequestAuthBearerTokenMoveCursorRight(event_key_bindings) |
            EditingRequestAuthBearerTokenCharInput(event_key_bindings) |
            ModifyRequestHeader(event_key_bindings) |
            EditingRequestHeaderDeleteCharBackward(event_key_bindings) |
            EditingRequestHeaderDeleteCharForward(event_key_bindings) |
            EditingRequestHeaderMoveCursorLeft(event_key_bindings) |
            EditingRequestHeaderMoveCursorRight(event_key_bindings) |
            EditingRequestHeaderCharInput(event_key_bindings) |
            ModifyRequestBodyTable(event_key_bindings) |
            EditingRequestBodyTableDeleteCharBackward(event_key_bindings) |
            EditingRequestBodyTableDeleteCharForward(event_key_bindings) |
            EditingRequestBodyTableMoveCursorLeft(event_key_bindings) |
            EditingRequestBodyTableMoveCursorRight(event_key_bindings) |
            EditingRequestBodyTableCharInput(event_key_bindings) |
            ModifyRequestBodyFile(event_key_bindings) |
            EditingRequestBodyFileDeleteCharBackward(event_key_bindings) |
            EditingRequestBodyFileDeleteCharForward(event_key_bindings) |
            EditingRequestBodyFileMoveCursorLeft(event_key_bindings) |
            EditingRequestBodyFileMoveCursorRight(event_key_bindings) |
            EditingRequestBodyFileCharInput(event_key_bindings) |
            EditingRequestBodyStringVimInput(event_key_bindings) |
            EditingRequestBodyStringCopy(event_key_bindings) |
            EditingRequestBodyStringPaste(event_key_bindings) |
            EditingRequestBodyStringUndo(event_key_bindings) |
            EditingRequestBodyStringRedo(event_key_bindings) |
            EditingRequestBodyStringSaveAndQuit(event_key_bindings) |
            EditingRequestBodyStringNewLine(event_key_bindings) |
            EditingRequestBodyStringIndent(event_key_bindings) |
            EditingRequestBodyStringDeleteCharBackward(event_key_bindings) |
            EditingRequestBodyStringDeleteCharForward(event_key_bindings) |
            EditingRequestBodyStringSkipWordLeft(event_key_bindings) |
            EditingRequestBodyStringSkipWordRight(event_key_bindings) |
            EditingRequestBodyStringMoveCursorUp(event_key_bindings) |
            EditingRequestBodyStringMoveCursorDown(event_key_bindings) |
            EditingRequestBodyStringMoveCursorLeft(event_key_bindings) |
            EditingRequestBodyStringMoveCursorRight(event_key_bindings) |
            EditingRequestBodyStringCharInput(event_key_bindings) |
            EditingPreRequestScriptVimInput(event_key_bindings) |
            EditingPreRequestScriptCopy(event_key_bindings) |
            EditingPreRequestScriptPaste(event_key_bindings) |
            EditingPreRequestScriptUndo(event_key_bindings) |
            EditingPreRequestScriptRedo(event_key_bindings) |
            EditingPreRequestScriptSaveAndQuit(event_key_bindings) |
            EditingPreRequestScriptNewLine(event_key_bindings) |
            EditingPreRequestScriptIndent(event_key_bindings) |
            EditingPreRequestScriptDeleteCharBackward(event_key_bindings) |
            EditingPreRequestScriptDeleteCharForward(event_key_bindings) |
            EditingPreRequestScriptSkipWordLeft(event_key_bindings) |
            EditingPreRequestScriptSkipWordRight(event_key_bindings) |
            EditingPreRequestScriptMoveCursorUp(event_key_bindings) |
            EditingPreRequestScriptMoveCursorDown(event_key_bindings) |
            EditingPreRequestScriptMoveCursorLeft(event_key_bindings) |
            EditingPreRequestScriptMoveCursorRight(event_key_bindings) |
            EditingPreRequestScriptCharInput(event_key_bindings) |
            EditingPostRequestScriptVimInput(event_key_bindings) |
            EditingPostRequestScriptCopy(event_key_bindings) |
            EditingPostRequestScriptPaste(event_key_bindings) |
            EditingPostRequestScriptUndo(event_key_bindings) |
            EditingPostRequestScriptRedo(event_key_bindings) |
            EditingPostRequestScriptSaveAndQuit(event_key_bindings) |
            EditingPostRequestScriptNewLine(event_key_bindings) |
            EditingPostRequestScriptIndent(event_key_bindings) |
            EditingPostRequestScriptDeleteCharBackward(event_key_bindings) |
            EditingPostRequestScriptDeleteCharForward(event_key_bindings) |
            EditingPostRequestScriptSkipWordLeft(event_key_bindings) |
            EditingPostRequestScriptSkipWordRight(event_key_bindings) |
            EditingPostRequestScriptMoveCursorUp(event_key_bindings) |
            EditingPostRequestScriptMoveCursorDown(event_key_bindings) |
            EditingPostRequestScriptMoveCursorLeft(event_key_bindings) |
            EditingPostRequestScriptMoveCursorRight(event_key_bindings) |
            EditingPostRequestScriptCharInput(event_key_bindings) |
            RequestSettingsMoveUp(event_key_bindings) |
            RequestSettingsMoveDown(event_key_bindings) |
            RequestSettingsToggleSetting(event_key_bindings) |
            ModifyRequestSettings(event_key_bindings) |
            Documentation(event_key_bindings)
            => event_key_bindings,
        }
    }
}