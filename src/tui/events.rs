use crokey::KeyCombination;
use crokey::OneToThree::One;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use tracing::{debug};
use tui_textarea::CursorMove;

use crate::app::app::App;
use crate::app::files::key_bindings::KEY_BINDINGS;
use crate::get_key_bindings;
use crate::tui::app_states::AVAILABLE_EVENTS;
use crate::tui::event_key_bindings::EventKeyBinding;
use crate::tui::events::AppEvent::*;
use crate::tui::utils::vim_emulation::{Vim, VimTransition};

get_key_bindings! {
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
        DuplicateElement(EventKeyBinding),

        MoveElementUp(EventKeyBinding),
        MoveElementDown(EventKeyBinding),

        NextEnvironment(EventKeyBinding),
        DisplayEnvEditor(EventKeyBinding),
        DisplayCookies(EventKeyBinding),
        DisplayLogs(EventKeyBinding),

        GoBackToLastState(EventKeyBinding),

        /* Env */

        EditEnvVariable(EventKeyBinding),
        EnvVariablesMoveUp(EventKeyBinding),
        EnvVariablesMoveDown(EventKeyBinding),
        EnvVariablesMoveLeft(EventKeyBinding),
        EnvVariablesMoveRight(EventKeyBinding),
        CreateEnvVariable(EventKeyBinding),
        DeleteEnvVariable(EventKeyBinding),

        ModifyEnvVariable(EventKeyBinding),
        EditingEnvVariableDeleteCharBackward(EventKeyBinding),
        EditingEnvVariableDeleteCharForward(EventKeyBinding),
        EditingEnvVariableMoveCursorLeft(EventKeyBinding),
        EditingEnvVariableMoveCursorRight(EventKeyBinding),
        EditingEnvVariableMoveCursorLineStart(EventKeyBinding),
        EditingEnvVariableMoveCursorLineEnd(EventKeyBinding),
        EditingEnvVariableCharInput(EventKeyBinding),

        /* Cookies */

        CookiesMoveUp(EventKeyBinding),
        CookiesMoveDown(EventKeyBinding),
        CookiesMoveLeft(EventKeyBinding),
        CookiesMoveRight(EventKeyBinding),
        DeleteCookie(EventKeyBinding),

        /* Logs */

        ScrollLogsUp(EventKeyBinding),
        ScrollLogsDown(EventKeyBinding),
        ScrollLogsLeft(EventKeyBinding),
        ScrollLogsRight(EventKeyBinding),

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
        DuplicateRequestQueryParam(EventKeyBinding),

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
        DuplicateRequestHeader(EventKeyBinding),

        EditRequestBody(EventKeyBinding),
        RequestBodyTableMoveUp(EventKeyBinding),
        RequestBodyTableMoveDown(EventKeyBinding),
        RequestBodyTableMoveLeft(EventKeyBinding),
        RequestBodyTableMoveRight(EventKeyBinding),
        CreateRequestBodyTableElement(EventKeyBinding),
        DeleteRequestBodyTableElement(EventKeyBinding),
        ToggleRequestBodyTableElement(EventKeyBinding),
        DuplicateRequestBodyTableElement(EventKeyBinding),

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

        /* Request export */

        ExportRequest(EventKeyBinding),
        RequestExportFormatMoveCursorLeft(EventKeyBinding),
        RequestExportFormatMoveCursorRight(EventKeyBinding),
        SelectRequestExportFormat(EventKeyBinding),

        ScrollRequestExportUp(EventKeyBinding),
        ScrollRequestExportDown(EventKeyBinding),
        ScrollRequestExportLeft(EventKeyBinding),
        ScrollRequestExportRight(EventKeyBinding),
        CopyRequestExport(EventKeyBinding),

        /* Request Text inputs */

        ModifyRequestUrl(EventKeyBinding),
        EditingRequestUrlDeleteCharBackward(EventKeyBinding),
        EditingRequestUrlDeleteCharForward(EventKeyBinding),
        EditingRequestUrlMoveCursorLeft(EventKeyBinding),
        EditingRequestUrlMoveCursorRight(EventKeyBinding),
        EditingRequestUrlMoveCursorLineStart(EventKeyBinding),
        EditingRequestUrlMoveCursorLineEnd(EventKeyBinding),
        EditingRequestUrlCharInput(EventKeyBinding),

        ModifyRequestQueryParam(EventKeyBinding),
        EditingRequestQueryParamDeleteCharBackward(EventKeyBinding),
        EditingRequestQueryParamDeleteCharForward(EventKeyBinding),
        EditingRequestQueryParamMoveCursorLeft(EventKeyBinding),
        EditingRequestQueryParamMoveCursorRight(EventKeyBinding),
        EditingRequestQueryParamMoveCursorLineStart(EventKeyBinding),
        EditingRequestQueryParamMoveCursorLineEnd(EventKeyBinding),
        EditingRequestQueryParamCharInput(EventKeyBinding),

        /* Auth */

        ModifyRequestAuthUsername(EventKeyBinding),
        EditingRequestAuthUsernameDeleteCharBackward(EventKeyBinding),
        EditingRequestAuthUsernameDeleteCharForward(EventKeyBinding),
        EditingRequestAuthUsernameMoveCursorLeft(EventKeyBinding),
        EditingRequestAuthUsernameMoveCursorRight(EventKeyBinding),
        EditingRequestAuthUsernameMoveCursorLineStart(EventKeyBinding),
        EditingRequestAuthUsernameMoveCursorLineEnd(EventKeyBinding),
        EditingRequestAuthUsernameCharInput(EventKeyBinding),

        ModifyRequestAuthPassword(EventKeyBinding),
        EditingRequestAuthPasswordDeleteCharBackward(EventKeyBinding),
        EditingRequestAuthPasswordDeleteCharForward(EventKeyBinding),
        EditingRequestAuthPasswordMoveCursorLeft(EventKeyBinding),
        EditingRequestAuthPasswordMoveCursorRight(EventKeyBinding),
        EditingRequestAuthPasswordMoveCursorLineStart(EventKeyBinding),
        EditingRequestAuthPasswordMoveCursorLineEnd(EventKeyBinding),
        EditingRequestAuthPasswordCharInput(EventKeyBinding),

        ModifyRequestAuthBearerToken(EventKeyBinding),
        EditingRequestAuthBearerTokenDeleteCharBackward(EventKeyBinding),
        EditingRequestAuthBearerTokenDeleteCharForward(EventKeyBinding),
        EditingRequestAuthBearerTokenMoveCursorLeft(EventKeyBinding),
        EditingRequestAuthBearerTokenMoveCursorRight(EventKeyBinding),
        EditingRequestAuthBearerTokenMoveCursorLineStart(EventKeyBinding),
        EditingRequestAuthBearerTokenMoveCursorLineEnd(EventKeyBinding),
        EditingRequestAuthBearerTokenCharInput(EventKeyBinding),

        /* Headers */

        ModifyRequestHeader(EventKeyBinding),
        EditingRequestHeaderDeleteCharBackward(EventKeyBinding),
        EditingRequestHeaderDeleteCharForward(EventKeyBinding),
        EditingRequestHeaderMoveCursorLeft(EventKeyBinding),
        EditingRequestHeaderMoveCursorRight(EventKeyBinding),
        EditingRequestHeaderMoveCursorLineStart(EventKeyBinding),
        EditingRequestHeaderMoveCursorLineEnd(EventKeyBinding),
        EditingRequestHeaderCharInput(EventKeyBinding),

        /* Body */

        ModifyRequestBodyTable(EventKeyBinding),
        EditingRequestBodyTableDeleteCharBackward(EventKeyBinding),
        EditingRequestBodyTableDeleteCharForward(EventKeyBinding),
        EditingRequestBodyTableMoveCursorLeft(EventKeyBinding),
        EditingRequestBodyTableMoveCursorRight(EventKeyBinding),
        EditingRequestBodyTableMoveCursorLineStart(EventKeyBinding),
        EditingRequestBodyTableMoveCursorLineEnd(EventKeyBinding),
        EditingRequestBodyTableCharInput(EventKeyBinding),

        ModifyRequestBodyFile(EventKeyBinding),
        EditingRequestBodyFileDeleteCharBackward(EventKeyBinding),
        EditingRequestBodyFileDeleteCharForward(EventKeyBinding),
        EditingRequestBodyFileMoveCursorLeft(EventKeyBinding),
        EditingRequestBodyFileMoveCursorRight(EventKeyBinding),
        EditingRequestBodyFileMoveCursorLineStart(EventKeyBinding),
        EditingRequestBodyFileMoveCursorLineEnd(EventKeyBinding),
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
        EditingRequestBodyStringMoveCursorLineStart(EventKeyBinding),
        EditingRequestBodyStringMoveCursorLineEnd(EventKeyBinding),
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
        EditingPreRequestScriptMoveCursorLineStart(EventKeyBinding),
        EditingPreRequestScriptMoveCursorLineEnd(EventKeyBinding),
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
        EditingPostRequestScriptMoveCursorLineStart(EventKeyBinding),
        EditingPostRequestScriptMoveCursorLineEnd(EventKeyBinding),
        EditingPostRequestScriptCharInput(EventKeyBinding),

        /* Settings */

        RequestSettingsMoveUp(EventKeyBinding),
        RequestSettingsMoveDown(EventKeyBinding),
        RequestSettingsToggleSettingLeft(EventKeyBinding),
        RequestSettingsToggleSettingRight(EventKeyBinding),
        ModifyRequestSettings(EventKeyBinding),

        /* Others */

        Documentation(EventKeyBinding),
    }
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

                let key = KeyCombination::from(key_event);
                let is_input_missed = self.handle_key(key).await;

                if !is_input_missed {
                    debug!("Key pressed: {}", key);
                }
            }
        }

        let received_response = *self.received_response.lock();
        if received_response {
            self.tui_highlight_response_body_and_console();
            self.tui_refresh_result_scrollbars();

            if self.config.should_save_requests_reponse() {
                let selection = self.collections_tree.state.selected().to_vec();
                if selection.len() > 0 {
                    self.save_collection_to_file(selection[0]);
                }
            }

            *self.received_response.lock() = false;
        }
    }

    async fn handle_key(&mut self, key: KeyCombination) -> bool {
        // Debug tool
        //dbg!("{}", key.to_string());

        {
            let key_bindings = KEY_BINDINGS.read();

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
            else if key == key_bindings.generic.display_help && !self.in_input() {
                self.should_display_help = true;
                self.help_popup.selection = self.state;
                return false;
            }
        }

        let mut miss_input = false;
        let mut matching_event: Option<&AppEvent> = None;

        let available_app_events = AVAILABLE_EVENTS.read();

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
                DuplicateElement(_) => self.duplicate_element(),

                MoveElementUp(_) => self.tui_move_element_up(),
                MoveElementDown(_) => self.tui_move_element_down(),

                NextEnvironment(_) => self.tui_next_environment(),
                DisplayEnvEditor(_) => self.display_env_editor_state(),
                DisplayCookies(_) => self.display_cookies_state(),
                DisplayLogs(_) => self.display_logs_state(),

                GoBackToLastState(_) => self.normal_state(),

                /* Env */

                EditEnvVariable(_) => match self.env_editor_table.is_selected() {
                    true => self.edit_env_variable_state(),
                    false => {}
                },
                EnvVariablesMoveUp(_) => self.env_editor_table.up(),
                EnvVariablesMoveDown(_) => self.env_editor_table.down(),
                EnvVariablesMoveLeft(_) | EnvVariablesMoveRight(_) => self.env_editor_table.change_y(),
                CreateEnvVariable(_) => self.tui_create_env_variable(),
                DeleteEnvVariable(_) => self.tui_delete_env_variable(),

                ModifyEnvVariable(_) => self.tui_modify_env_variable(),
                EditingEnvVariableDeleteCharBackward(_) => self.env_editor_table.selection_text_input.delete_char_forward(),
                EditingEnvVariableDeleteCharForward(_) => self.env_editor_table.selection_text_input.delete_char_backward(),
                EditingEnvVariableMoveCursorLeft(_) => self.env_editor_table.selection_text_input.move_cursor_left(),
                EditingEnvVariableMoveCursorRight(_) => self.env_editor_table.selection_text_input.move_cursor_right(),
                EditingEnvVariableMoveCursorLineStart(_) => self.env_editor_table.selection_text_input.move_cursor_line_start(),
                EditingEnvVariableMoveCursorLineEnd(_) => self.env_editor_table.selection_text_input.move_cursor_line_end(),
                EditingEnvVariableCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.env_editor_table.selection_text_input.enter_char(char),
                    _ => {}
                },

                /* Cookies */

                CookiesMoveUp(_) => self.cookies_popup.cookies_table.up(),
                CookiesMoveDown(_) => self.cookies_popup.cookies_table.down(),
                CookiesMoveLeft(_) => self.cookies_popup.cookies_table.left(),
                CookiesMoveRight(_) => self.cookies_popup.cookies_table.right(),

                DeleteCookie(_) => self.tui_delete_cookie(),

                /* Logs */
                
                ScrollLogsUp(_) => self.logs_vertical_scrollbar.page_up(),
                ScrollLogsDown(_) => self.logs_vertical_scrollbar.page_down(),
                ScrollLogsLeft(_) => self.logs_horizontal_scrollbar.page_up(),
                ScrollLogsRight(_) => self.logs_horizontal_scrollbar.page_down(),
                
                /* Collections */

                ChooseElementToCreateMoveCursorLeft(_) => self.creation_popup.previous(),
                ChooseElementToCreateMoveCursorRight(_) => self.creation_popup.next(),
                SelectElementToCreate(_) => self.new_element(),

                CreateNewCollection(_) => self.tui_new_collection(),
                CreatingCollectionDeleteCharBackward(_) => self.new_collection_input.delete_char_forward(),
                CreatingCollectionDeleteCharForward(_) => self.new_collection_input.delete_char_backward(),
                CreatingCollectionMoveCursorLeft(_) => self.new_collection_input.move_cursor_left(),
                CreatingCollectionMoveCursorRight(_) => self.new_collection_input.move_cursor_right(),
                CreatingCollectionCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.new_collection_input.enter_char(char),
                    _ => {}
                },

                CreateNewRequest(_) => self.tui_new_request(),
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
                    true => self.tui_delete_collection(),
                    false => self.normal_state(),
                },
                
                DeletingRequestMoveCursorLeft(_) => self.delete_request_popup.change_state(),
                DeletingRequestMoveCursorRight(_) => self.delete_request_popup.change_state(),
                DeleteRequest(_) => match self.delete_request_popup.state {
                    true => self.tui_delete_request(),
                    false => self.normal_state(),
                },

                RenameCollection(_) => self.tui_rename_collection(),
                RenamingCollectionDeleteCharBackward(_) => self.rename_collection_input.delete_char_forward(),
                RenamingCollectionDeleteCharForward(_) => self.rename_collection_input.delete_char_backward(),
                RenamingCollectionMoveCursorLeft(_) => self.rename_collection_input.move_cursor_left(),
                RenamingCollectionMoveCursorRight(_) => self.rename_collection_input.move_cursor_right(),
                RenamingCollectionCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.rename_collection_input.enter_char(char),
                    _ => {}
                },
                
                RenameRequest(_) => self.tui_rename_request(),
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
                EditMethod(_) => self.tui_next_request_method(),
                EditSettings(_) => self.edit_request_settings_state(),

                NextView(_) => self.next_request_view(),
                SendRequest(_) => self.tui_send_request().await,

                /* Param tabs */

                NextParamTab(_) => self.tui_next_request_param_tab(),

                ModifyRequestAuthMethod(_) => self.tui_next_request_auth(),
                ModifyRequestBodyContentType(_) => self.tui_modify_request_content_type(),

                EditRequestQueryParam(_) => match self.query_params_table.is_selected() {
                    true => self.edit_request_param_state(),
                    false => {}
                },
                RequestQueryParamsMoveUp(_) => self.query_params_table.up(),
                RequestQueryParamsMoveDown(_) => self.query_params_table.down(),
                RequestQueryParamsMoveLeft(_) | RequestQueryParamsMoveRight(_) => self.query_params_table.change_y(),
                CreateRequestQueryParam(_) => self.tui_create_new_query_param(),
                DeleteRequestQueryParam(_) => self.tui_delete_query_param(),
                ToggleRequestQueryParam(_) => self.tui_toggle_query_param(),
                DuplicateRequestQueryParam(_) => self.tui_duplicate_query_param(),

                EditRequestAuth(_) => match self.auth_text_input_selection.usable {
                    true => self.tui_select_request_auth_input_text(),
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
                CreateRequestHeader(_) => self.tui_create_new_header(),
                DeleteRequestHeader(_) => self.tui_delete_header(),
                ToggleRequestHeader(_) => self.tui_toggle_header(),
                DuplicateRequestHeader(_) => self.tui_duplicate_header(),

                EditRequestBody(_) => match self.body_form_table.is_selected() {
                    true => self.edit_request_body_table_state(),
                    false => self.edit_request_body_file_or_string_state(),
                },
                RequestBodyTableMoveUp(_) => self.body_form_table.up(),
                RequestBodyTableMoveDown(_) => self.body_form_table.down(),
                RequestBodyTableMoveLeft(_) | RequestBodyTableMoveRight(_) => self.body_form_table.change_y(),
                CreateRequestBodyTableElement(_) => self.tui_create_new_form_data(),
                DeleteRequestBodyTableElement(_) => self.tui_delete_form_data(),
                ToggleRequestBodyTableElement(_) => self.tui_toggle_form_data(),
                DuplicateRequestBodyTableElement(_) => self.tui_duplicate_form_data(),

                /* Scripts */

                EditRequestScript(_) => self.edit_request_script_state(),
                RequestScriptMove(_) => self.script_console.change_selection(),

                /* Result tabs */

                NextResultTab(_) => self.tui_next_request_result_tab(),

                ScrollResultUp(_) => self.result_vertical_scrollbar.page_up(),
                ScrollResultDown(_) => self.result_vertical_scrollbar.page_down(),
                ScrollResultLeft(_) => self.result_horizontal_scrollbar.page_up(),
                ScrollResultRight(_) => self.result_horizontal_scrollbar.page_down(),

                /* Others */

                CopyResponsePart(_) => self.copy_response_body_content_to_clipboard(),

                /* Request Export */

                ExportRequest(_) => self.choose_request_export_format_state(),

                RequestExportFormatMoveCursorLeft(_) => self.export_request.previous(),
                RequestExportFormatMoveCursorRight(_) => self.export_request.next(),

                SelectRequestExportFormat(_) => self.tui_export_request(),

                ScrollRequestExportUp(_) => self.display_request_export.vertical_scrollbar.page_up(),
                ScrollRequestExportDown(_) => self.display_request_export.vertical_scrollbar.page_down(),
                ScrollRequestExportLeft(_) => self.display_request_export.horizontal_scrollbar.page_up(),
                ScrollRequestExportRight(_) => self.display_request_export.horizontal_scrollbar.page_down(),

                CopyRequestExport(_) => self.copy_request_export_to_clipboard(),

                /* Request text inputs */

                ModifyRequestUrl(_) => self.tui_modify_request_url(),
                EditingRequestUrlDeleteCharBackward(_) => self.url_text_input.delete_char_forward(),
                EditingRequestUrlDeleteCharForward(_) => self.url_text_input.delete_char_backward(),
                EditingRequestUrlMoveCursorLeft(_) => self.url_text_input.move_cursor_left(),
                EditingRequestUrlMoveCursorRight(_) => self.url_text_input.move_cursor_right(),
                EditingRequestUrlMoveCursorLineStart(_) => self.url_text_input.move_cursor_line_start(),
                EditingRequestUrlMoveCursorLineEnd(_) => self.url_text_input.move_cursor_line_end(),
                EditingRequestUrlCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.url_text_input.enter_char(char),
                    _ => {}
                },

                ModifyRequestQueryParam(_) => self.tui_modify_request_query_param(),
                EditingRequestQueryParamDeleteCharBackward(_) => self.query_params_table.selection_text_input.delete_char_forward(),
                EditingRequestQueryParamDeleteCharForward(_) => self.query_params_table.selection_text_input.delete_char_backward(),
                EditingRequestQueryParamMoveCursorLeft(_) => self.query_params_table.selection_text_input.move_cursor_left(),
                EditingRequestQueryParamMoveCursorRight(_) => self.query_params_table.selection_text_input.move_cursor_right(),
                EditingRequestQueryParamMoveCursorLineStart(_) => self.query_params_table.selection_text_input.move_cursor_line_start(),
                EditingRequestQueryParamMoveCursorLineEnd(_) => self.query_params_table.selection_text_input.move_cursor_line_end(),
                EditingRequestQueryParamCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.query_params_table.selection_text_input.enter_char(char),
                    _ => {}
                },

                /* Auth */
                
                // self.auth_text_input_selection.usable

                ModifyRequestAuthUsername(_) => self.tui_modify_request_auth_basic_username(),
                EditingRequestAuthUsernameDeleteCharBackward(_) => self.auth_basic_username_text_input.delete_char_forward(),
                EditingRequestAuthUsernameDeleteCharForward(_) => self.auth_basic_username_text_input.delete_char_backward(),
                EditingRequestAuthUsernameMoveCursorLeft(_) => self.auth_basic_username_text_input.move_cursor_left(),
                EditingRequestAuthUsernameMoveCursorRight(_) => self.auth_basic_username_text_input.move_cursor_right(),
                EditingRequestAuthUsernameMoveCursorLineStart(_) => self.auth_basic_username_text_input.move_cursor_line_start(),
                EditingRequestAuthUsernameMoveCursorLineEnd(_) => self.auth_basic_username_text_input.move_cursor_line_end(),
                EditingRequestAuthUsernameCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.auth_basic_username_text_input.enter_char(char),
                    _ => {}
                },
                
                ModifyRequestAuthPassword(_) => self.tui_modify_request_auth_basic_password(),
                EditingRequestAuthPasswordDeleteCharBackward(_) => self.auth_basic_password_text_input.delete_char_forward(),
                EditingRequestAuthPasswordDeleteCharForward(_) => self.auth_basic_password_text_input.delete_char_backward(),
                EditingRequestAuthPasswordMoveCursorLeft(_) => self.auth_basic_password_text_input.move_cursor_left(),
                EditingRequestAuthPasswordMoveCursorRight(_) => self.auth_basic_password_text_input.move_cursor_right(),
                EditingRequestAuthPasswordMoveCursorLineStart(_) => self.auth_basic_password_text_input.move_cursor_line_start(),
                EditingRequestAuthPasswordMoveCursorLineEnd(_) => self.auth_basic_password_text_input.move_cursor_line_end(),
                EditingRequestAuthPasswordCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.auth_basic_password_text_input.enter_char(char),
                    _ => {}
                },
                
                ModifyRequestAuthBearerToken(_) => self.tui_modify_request_auth_bearer_token(),
                EditingRequestAuthBearerTokenDeleteCharBackward(_) => self.auth_bearer_token_text_input.delete_char_forward(),
                EditingRequestAuthBearerTokenDeleteCharForward(_) => self.auth_bearer_token_text_input.delete_char_backward(),
                EditingRequestAuthBearerTokenMoveCursorLeft(_) => self.auth_bearer_token_text_input.move_cursor_left(),
                EditingRequestAuthBearerTokenMoveCursorRight(_) => self.auth_bearer_token_text_input.move_cursor_right(),
                EditingRequestAuthBearerTokenMoveCursorLineStart(_) => self.auth_bearer_token_text_input.move_cursor_line_start(),
                EditingRequestAuthBearerTokenMoveCursorLineEnd(_) => self.auth_bearer_token_text_input.move_cursor_line_end(),
                EditingRequestAuthBearerTokenCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.auth_bearer_token_text_input.enter_char(char),
                    _ => {}
                },

                /* Header */

                ModifyRequestHeader(_) => self.tui_modify_request_header(),
                EditingRequestHeaderDeleteCharBackward(_) => self.headers_table.selection_text_input.delete_char_forward(),
                EditingRequestHeaderDeleteCharForward(_) => self.headers_table.selection_text_input.delete_char_backward(),
                EditingRequestHeaderMoveCursorLeft(_) => self.headers_table.selection_text_input.move_cursor_left(),
                EditingRequestHeaderMoveCursorRight(_) => self.headers_table.selection_text_input.move_cursor_right(),
                EditingRequestHeaderMoveCursorLineStart(_) => self.headers_table.selection_text_input.move_cursor_line_start(),
                EditingRequestHeaderMoveCursorLineEnd(_) => self.headers_table.selection_text_input.move_cursor_line_end(),
                EditingRequestHeaderCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.headers_table.selection_text_input.enter_char(char),
                    _ => {}
                },

                /* Body */

                ModifyRequestBodyTable(_) => self.tui_modify_request_form_data(),
                EditingRequestBodyTableDeleteCharBackward(_) => self.body_form_table.selection_text_input.delete_char_forward(),
                EditingRequestBodyTableDeleteCharForward(_) => self.body_form_table.selection_text_input.delete_char_backward(),
                EditingRequestBodyTableMoveCursorLeft(_) => self.body_form_table.selection_text_input.move_cursor_left(),
                EditingRequestBodyTableMoveCursorRight(_) => self.body_form_table.selection_text_input.move_cursor_right(),
                EditingRequestBodyTableMoveCursorLineStart(_) => self.body_form_table.selection_text_input.move_cursor_line_start(),
                EditingRequestBodyTableMoveCursorLineEnd(_) => self.body_form_table.selection_text_input.move_cursor_line_end(),
                EditingRequestBodyTableCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.body_form_table.selection_text_input.enter_char(char),
                    _ => {}
                },
                
                ModifyRequestBodyFile(_) => self.tui_next_request_body(),
                EditingRequestBodyFileDeleteCharBackward(_) => self.body_file_text_input.delete_char_forward(),
                EditingRequestBodyFileDeleteCharForward(_) => self.body_file_text_input.delete_char_backward(),
                EditingRequestBodyFileMoveCursorLeft(_) => self.body_file_text_input.move_cursor_left(),
                EditingRequestBodyFileMoveCursorRight(_) => self.body_file_text_input.move_cursor_right(),
                EditingRequestBodyFileMoveCursorLineStart(_) => self.body_file_text_input.move_cursor_line_start(),
                EditingRequestBodyFileMoveCursorLineEnd(_) => self.body_file_text_input.move_cursor_line_end(),
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
                    VimTransition::SaveAndQuit => self.tui_next_request_body(),
                },

                EditingRequestBodyStringSaveAndQuit(_) => self.tui_next_request_body(),
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
                EditingRequestBodyStringMoveCursorLineStart(_) => self.body_text_area.move_cursor(CursorMove::Head),
                EditingRequestBodyStringMoveCursorLineEnd(_) => self.body_text_area.move_cursor(CursorMove::End),
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
                EditingPreRequestScriptMoveCursorLineStart(_) => self.script_console.pre_request_text_area.move_cursor(CursorMove::Head),
                EditingPreRequestScriptMoveCursorLineEnd(_) => self.script_console.pre_request_text_area.move_cursor(CursorMove::End),
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
                EditingPostRequestScriptMoveCursorLineStart(_) => self.script_console.post_request_text_area.move_cursor(CursorMove::Head),
                EditingPostRequestScriptMoveCursorLineEnd(_) => self.script_console.post_request_text_area.move_cursor(CursorMove::End),
                EditingPostRequestScriptCharInput(_) => match key {
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.script_console.post_request_text_area.insert_char(char),
                    _ => {}
                },
                
                /* Settings */

                RequestSettingsMoveUp(_) => self.request_settings_popup.previous(),
                RequestSettingsMoveDown(_) => self.request_settings_popup.next(),
                RequestSettingsToggleSettingLeft(_) => self.request_settings_popup.toggle_setting_left(),
                RequestSettingsToggleSettingRight(_) => self.request_settings_popup.toggle_setting_right(),
                ModifyRequestSettings(_) => self.tui_modify_request_settings(),

                /* Others */

                Documentation(_) => {}
            }
        };

        return miss_input;
    }
}