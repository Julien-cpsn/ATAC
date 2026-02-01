use std::io::Stdout;
use crokey::KeyCombination;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyEventKind};
use ratatui::prelude::{CrosstermBackend};
use ratatui::Terminal;
use tracing::{debug};

use crate::app::app::App;
use crate::app::files::key_bindings::KEY_BINDINGS;
use crate::get_key_bindings;
use crate::tui::app_states::AVAILABLE_EVENTS;
use crate::tui::event_key_bindings::EventKeyBinding;
use crate::tui::events::AppEvent::*;

get_key_bindings! {
    #[derive(Debug, Clone)]
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
        CancelModifyEnvVariable(EventKeyBinding),
        KeyEventModifyEnvVariable(EventKeyBinding),

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
        CancelCreateNewCollection(EventKeyBinding),
        KeyEventCreateNewCollection(EventKeyBinding),

        CreateNewRequest(EventKeyBinding),
        CancelCreateNewRequest(EventKeyBinding),
        CreatingRequestSelectInputUp(EventKeyBinding),
        CreatingRequestSelectInputDown(EventKeyBinding),
        CreatingRequestInputLeft(EventKeyBinding),
        CreatingRequestInputRight(EventKeyBinding),
        KeyEventCreateNewRequest(EventKeyBinding),

        DeletingCollectionMoveCursorLeft(EventKeyBinding),
        DeletingCollectionMoveCursorRight(EventKeyBinding),
        DeleteCollection(EventKeyBinding),

        DeletingRequestMoveCursorLeft(EventKeyBinding),
        DeletingRequestMoveCursorRight(EventKeyBinding),
        DeleteRequest(EventKeyBinding),

        RenameCollection(EventKeyBinding),
        CancelRenameCollection(EventKeyBinding),
        KeyEventRenameCollection(EventKeyBinding),

        RenameRequest(EventKeyBinding),
        CancelRenameRequest(EventKeyBinding),
        KeyEventRenameRequest(EventKeyBinding),

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
        ModifyRequestMessageType(EventKeyBinding),

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
        RequestAuthMoveLeft(EventKeyBinding),
        RequestAuthMoveRight(EventKeyBinding),

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

        EditRequestMessage(EventKeyBinding),

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
        CancelEditRequestUrl(EventKeyBinding),
        KeyEventEditRequestUrl(EventKeyBinding),

        ModifyRequestQueryParam(EventKeyBinding),
        CancelEditRequestQueryParam(EventKeyBinding),
        KeyEventEditRequestQueryParam(EventKeyBinding),

        /* Auth */

        ModifyRequestAuthBasicUsername(EventKeyBinding),
        CancelEditRequestAuthBasicUsername(EventKeyBinding),
        KeyEventEditRequestAuthBasicUsername(EventKeyBinding),

        ModifyRequestAuthBasicPassword(EventKeyBinding),
        CancelEditRequestAuthBasicPassword(EventKeyBinding),
        KeyEventEditRequestAuthBasicPassword(EventKeyBinding),

        ModifyRequestAuthBearerToken(EventKeyBinding),
        CancelEditRequestAuthBearerToken(EventKeyBinding),
        KeyEventEditRequestAuthBearerToken(EventKeyBinding),

        ModifyRequestAuthJwtSecret(EventKeyBinding),
        CancelEditRequestAuthJwtSecret(EventKeyBinding),
        KeyEventEditRequestAuthJwtSecret(EventKeyBinding),

        ModifyRequestAuthJwtPayload(EventKeyBinding),
        CancelEditRequestAuthJwtPayload(EventKeyBinding),
        KeyEventEditRequestAuthJwtPayload(EventKeyBinding),

        ModifyRequestAuthDigestUsername(EventKeyBinding),
        CancelEditRequestAuthDigestUsername(EventKeyBinding),
        KeyEventEditRequestAuthDigestUsername(EventKeyBinding),

        ModifyRequestAuthDigestPassword(EventKeyBinding),
        CancelEditRequestAuthDigestPassword(EventKeyBinding),
        KeyEventEditRequestAuthDigestPassword(EventKeyBinding),

        ModifyRequestAuthDigestDomains(EventKeyBinding),
        CancelEditRequestAuthDigestDomains(EventKeyBinding),
        KeyEventEditRequestAuthDigestDomains(EventKeyBinding),

        ModifyRequestAuthDigestRealm(EventKeyBinding),
        CancelEditRequestAuthDigestRealm(EventKeyBinding),
        KeyEventEditRequestAuthDigestRealm(EventKeyBinding),

        ModifyRequestAuthDigestNonce(EventKeyBinding),
        CancelEditRequestAuthDigestNonce(EventKeyBinding),
        KeyEventEditRequestAuthDigestNonce(EventKeyBinding),

        ModifyRequestAuthDigestOpaque(EventKeyBinding),
        CancelEditRequestAuthDigestOpaque(EventKeyBinding),
        KeyEventEditRequestAuthDigestOpaque(EventKeyBinding),

        /* Headers */

        ModifyRequestHeader(EventKeyBinding),
        CancelEditRequestHeader(EventKeyBinding),
        KeyEventEditRequestHeader(EventKeyBinding),

        /* Body */

        ModifyRequestBodyTable(EventKeyBinding),
        CancelEditRequestBodyTable(EventKeyBinding),
        KeyEventEditRequestBodyTable(EventKeyBinding),

        ModifyRequestBodyFile(EventKeyBinding),
        CancelEditRequestBodyFile(EventKeyBinding),
        KeyEventEditRequestBodyFile(EventKeyBinding),

        ModifyRequestBodyString(EventKeyBinding),
        CancelEditRequestBodyString(EventKeyBinding),
        KeyEventEditRequestBodyString(EventKeyBinding),

        /* Websocket */

        ModifyRequestMessage(EventKeyBinding),
        CancelEditRequestMessage(EventKeyBinding),
        KeyEventEditRequestMessage(EventKeyBinding),

        /* Scripts */

        ModifyRequestPreRequestScript(EventKeyBinding),
        CancelEditRequestPreRequestScript(EventKeyBinding),
        KeyEventEditRequestPreRequestScript(EventKeyBinding),

        ModifyRequestPostRequestScript(EventKeyBinding),
        CancelEditRequestPostRequestScript(EventKeyBinding),
        KeyEventEditRequestPostRequestScript(EventKeyBinding),

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
    pub async fn handle_events(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        // Refreshes the app every tick_rate
        if event::poll(self.tick_rate).unwrap() {
            // Block while a key is pressed
            if let Event::Key(key_event) = event::read().unwrap() {
                // We do not need
                if key_event.kind != KeyEventKind::Press {
                    return;
                }

                let key = KeyCombination::from(key_event);
                let is_input_missed = self.handle_key(key, terminal).await;

                if !is_input_missed {
                    debug!("Key pressed: {}", key);
                }
            }
        }

        let received_response = *self.received_response.lock();
        if received_response {
            self.tui_highlight_response_body_and_console();
            self.tui_refresh_result_scrollbars();

            if self.config.should_save_requests_response() {
                let selection = self.collections_tree.state.selected().to_vec();
                if selection.len() > 0 {
                    self.save_collection_to_file(selection[0]);
                }
            }

            *self.received_response.lock() = false;
        }
    }

    async fn handle_key(&mut self, key: KeyCombination, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> bool {
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

                ModifyEnvVariable(_) => match self.env_editor_table.selection_text_input.is_in_default_mode() {
                    true => self.tui_modify_env_variable(),
                    false => self.env_editor_table.selection_text_input.key_event(key, None),
                },
                CancelModifyEnvVariable(_) => match self.env_editor_table.selection_text_input.is_in_default_mode() {
                    true => self.display_env_editor_state(),
                    false => self.env_editor_table.selection_text_input.key_event(key, None),
                },
                KeyEventModifyEnvVariable(_) => self.env_editor_table.selection_text_input.key_event(key, None),

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


                CreateNewCollection(_) => match self.new_collection_input.is_in_default_mode() {
                    true => self.tui_new_collection(),
                    false => self.new_collection_input.key_event(key, None),
                },
                CancelCreateNewCollection(_) => match self.new_collection_input.is_in_default_mode() {
                    true => self.normal_state(),
                    false => self.new_collection_input.key_event(key, None),
                },
                KeyEventCreateNewCollection(_) => self.new_collection_input.key_event(key, None),


                CreateNewRequest(_) => match self.new_request_popup.text_input.is_in_default_mode() {
                    true => self.tui_new_request(),
                    false => self.new_request_popup.text_input.key_event(key, None),
                },
                CancelCreateNewRequest(_) => match self.new_request_popup.text_input.is_in_default_mode() {
                    true => self.normal_state(),
                    false => self.new_request_popup.text_input.key_event(key, None),
                },
                CreatingRequestSelectInputUp(_) => self.new_request_popup.previous_input(),
                CreatingRequestSelectInputDown(_) => self.new_request_popup.next_input(),
                CreatingRequestInputLeft(_) => self.new_request_popup.input_left(),
                CreatingRequestInputRight(_) => self.new_request_popup.input_right(),
                KeyEventCreateNewRequest(_) => self.new_request_popup.text_input.key_event(key, None),


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


                RenameCollection(_) => match self.rename_collection_input.is_in_default_mode() {
                    true => self.tui_rename_collection(),
                    false => self.rename_collection_input.key_event(key, None),
                },
                CancelRenameCollection(_) => match self.rename_collection_input.is_in_default_mode() {
                    true => self.normal_state(),
                    false => self.rename_collection_input.key_event(key, None),
                },
                KeyEventRenameCollection(_) => self.rename_collection_input.key_event(key, None),


                RenameRequest(_) => match self.rename_request_input.is_in_default_mode() {
                    true => self.tui_rename_request(),
                    false => self.rename_request_input.key_event(key, None),
                },
                CancelRenameRequest(_) => match self.rename_request_input.is_in_default_mode() {
                    true => self.normal_state(),
                    false => self.rename_request_input.key_event(key, None),
                },
                KeyEventRenameRequest(_) => self.rename_request_input.key_event(key, None),


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
                ModifyRequestBodyContentType(_) => self.tui_next_request_content_type(),
                ModifyRequestMessageType(_) => self.tui_next_request_message_type(),

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
                RequestAuthMoveLeft(_) => self.tui_request_auth_move_left(),
                RequestAuthMoveRight(_) => self.tui_request_auth_move_right(),

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

                EditRequestMessage(_) => self.edit_request_message_state(),

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

                #[cfg(feature = "clipboard")]
                CopyResponsePart(_) => self.copy_response_body_content_to_clipboard(),

                #[cfg(not(feature = "clipboard"))]
                CopyResponsePart(_) => {},

                /* Request Export */

                ExportRequest(_) => self.choose_request_export_format_state(),

                RequestExportFormatMoveCursorLeft(_) => self.export_request.previous(),
                RequestExportFormatMoveCursorRight(_) => self.export_request.next(),

                SelectRequestExportFormat(_) => self.tui_export_request(),

                ScrollRequestExportUp(_) => self.display_request_export.vertical_scrollbar.page_up(),
                ScrollRequestExportDown(_) => self.display_request_export.vertical_scrollbar.page_down(),
                ScrollRequestExportLeft(_) => self.display_request_export.horizontal_scrollbar.page_up(),
                ScrollRequestExportRight(_) => self.display_request_export.horizontal_scrollbar.page_down(),

                #[cfg(feature = "clipboard")]
                CopyRequestExport(_) => self.copy_request_export_to_clipboard(),

                #[cfg(not(feature = "clipboard"))]
                CopyRequestExport(_) => {},

                /* Url */

                ModifyRequestUrl(_) => match self.url_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_url(),
                    false => self.url_text_input.key_event(key, None),
                },
                CancelEditRequestUrl(_) => match self.url_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.url_text_input.key_event(key, None),
                },
                KeyEventEditRequestUrl(_) => self.url_text_input.key_event(key, None),

                /* Query params */

                ModifyRequestQueryParam(_) => match self.query_params_table.selection_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_query_param(),
                    false => self.query_params_table.selection_text_input.key_event(key, None),
                },
                CancelEditRequestQueryParam(_) => match self.query_params_table.selection_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.query_params_table.selection_text_input.key_event(key, None),
                },
                KeyEventEditRequestQueryParam(_) => self.query_params_table.selection_text_input.key_event(key, None),

                /* Auth */

                // self.auth_text_input_selection.usable

                ModifyRequestAuthBasicUsername(_) => match self.auth_basic_username_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_auth_basic_username(),
                    false => self.auth_basic_username_text_input.key_event(key, None),
                },
                CancelEditRequestAuthBasicUsername(_) => match self.auth_basic_password_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.auth_basic_password_text_input.key_event(key, None),
                },
                KeyEventEditRequestAuthBasicUsername(_) => self.auth_basic_password_text_input.key_event(key, None),


                ModifyRequestAuthBasicPassword(_) => match self.auth_basic_password_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_auth_basic_password(),
                    false => self.auth_basic_password_text_input.key_event(key, None),
                },
                CancelEditRequestAuthBasicPassword(_) => match self.auth_basic_password_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.auth_basic_password_text_input.key_event(key, None),
                },
                KeyEventEditRequestAuthBasicPassword(_) => self.auth_digest_nonce_text_input.key_event(key, None),


                ModifyRequestAuthBearerToken(_) => match self.auth_bearer_token_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_auth_bearer_token(),
                    false => self.auth_bearer_token_text_input.key_event(key, None),
                },
                CancelEditRequestAuthBearerToken(_) => match self.auth_bearer_token_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.auth_bearer_token_text_input.key_event(key, None),
                },
                KeyEventEditRequestAuthBearerToken(_) => self.auth_bearer_token_text_input.key_event(key, None),


                ModifyRequestAuthJwtSecret(_) => match self.auth_jwt_secret_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_auth_jwt_secret(),
                    false => self.auth_jwt_secret_text_input.key_event(key, None),
                },
                CancelEditRequestAuthJwtSecret(_) => match self.auth_jwt_secret_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.auth_jwt_secret_text_input.key_event(key, None),
                },
                KeyEventEditRequestAuthJwtSecret(_) => self.auth_jwt_secret_text_input.key_event(key, None),


                ModifyRequestAuthJwtPayload(_) => match self.auth_jwt_payload_text_area.is_in_default_mode() {
                    true => self.tui_modify_request_auth_jwt_payload(),
                    false => self.auth_jwt_payload_text_area.key_event(key, Some(terminal)),
                },
                CancelEditRequestAuthJwtPayload(_) => match self.auth_jwt_payload_text_area.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.auth_jwt_payload_text_area.key_event(key, Some(terminal)),
                },
                KeyEventEditRequestAuthJwtPayload(_) => self.auth_jwt_payload_text_area.key_event(key, Some(terminal)),


                ModifyRequestAuthDigestUsername(_) => match self.auth_digest_username_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_auth_digest_username(),
                    false => self.auth_digest_username_text_input.key_event(key, None),
                },
                CancelEditRequestAuthDigestUsername(_) => match self.auth_digest_username_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.auth_digest_username_text_input.key_event(key, None),
                },
                KeyEventEditRequestAuthDigestUsername(_) => self.auth_digest_username_text_input.key_event(key, None),


                ModifyRequestAuthDigestPassword(_) => match self.auth_digest_password_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_auth_digest_password(),
                    false => self.auth_digest_password_text_input.key_event(key, None),
                },
                CancelEditRequestAuthDigestPassword(_) => match self.auth_digest_password_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.auth_digest_password_text_input.key_event(key, None),
                },
                KeyEventEditRequestAuthDigestPassword(_) => self.auth_digest_password_text_input.key_event(key, None),


                ModifyRequestAuthDigestDomains(_) => match self.auth_digest_domains_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_auth_digest_domains(),
                    false => self.auth_digest_domains_text_input.key_event(key, None),
                },
                CancelEditRequestAuthDigestDomains(_) => match self.auth_digest_domains_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.auth_digest_domains_text_input.key_event(key, None),
                },
                KeyEventEditRequestAuthDigestDomains(_) => self.auth_digest_domains_text_input.key_event(key, None),


                ModifyRequestAuthDigestRealm(_) => match self.auth_digest_realm_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_auth_digest_realm(),
                    false => self.auth_digest_realm_text_input.key_event(key, None),
                },
                CancelEditRequestAuthDigestRealm(_) => match self.auth_digest_realm_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.auth_digest_realm_text_input.key_event(key, None),
                },
                KeyEventEditRequestAuthDigestRealm(_) => self.auth_digest_realm_text_input.key_event(key, None),


                ModifyRequestAuthDigestNonce(_) => match self.auth_digest_nonce_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_auth_digest_nonce(),
                    false => self.auth_digest_nonce_text_input.key_event(key, None),
                },
                CancelEditRequestAuthDigestNonce(_) => match self.auth_digest_nonce_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.auth_digest_nonce_text_input.key_event(key, None),
                },
                KeyEventEditRequestAuthDigestNonce(_) => self.auth_digest_nonce_text_input.key_event(key, None),


                ModifyRequestAuthDigestOpaque(_) => match self.auth_digest_opaque_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_auth_digest_opaque(),
                    false => self.auth_digest_opaque_text_input.key_event(key, None),
                },
                CancelEditRequestAuthDigestOpaque(_) => match self.auth_digest_opaque_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.auth_digest_opaque_text_input.key_event(key, None),
                },
                KeyEventEditRequestAuthDigestOpaque(_) => self.auth_digest_opaque_text_input.key_event(key, None),

                /* Header */

                ModifyRequestHeader(_) => match self.headers_table.selection_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_header(),
                    false => self.headers_table.selection_text_input.key_event(key, None),
                },
                CancelEditRequestHeader(_) => match self.headers_table.selection_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.headers_table.selection_text_input.key_event(key, None),
                },
                KeyEventEditRequestHeader(_) => self.headers_table.selection_text_input.key_event(key, None),

                /* Body */

                ModifyRequestBodyTable(_) => match self.body_form_table.selection_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_form_data(),
                    false => self.body_form_table.selection_text_input.key_event(key, None),
                },
                CancelEditRequestBodyTable(_) => match self.body_form_table.selection_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.body_form_table.selection_text_input.key_event(key, None),
                },
                KeyEventEditRequestBodyTable(_) => self.body_form_table.selection_text_input.key_event(key, None),


                ModifyRequestBodyFile(_) => match self.body_file_text_input.is_in_default_mode() {
                    true => self.tui_modify_request_body(),
                    false => self.body_file_text_input.key_event(key, None),
                },
                CancelEditRequestBodyFile(_) => match self.body_file_text_input.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.body_file_text_input.key_event(key, None),
                },
                KeyEventEditRequestBodyFile(_) => self.body_file_text_input.key_event(key, None),


                ModifyRequestBodyString(_) => match self.body_text_area.is_in_default_mode() {
                    true => self.tui_modify_request_body(),
                    false => self.body_text_area.key_event(key, Some(terminal)),
                },
                CancelEditRequestBodyString(_) => match self.body_text_area.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.body_text_area.key_event(key, Some(terminal)),
                },
                KeyEventEditRequestBodyString(_) => self.body_text_area.key_event(key, Some(terminal)),

                /* Websocket */

                ModifyRequestMessage(_) => match self.message_text_area.is_in_default_mode() {
                    true => self.tui_send_request_message().await,
                    false => self.message_text_area.key_event(key, Some(terminal)),
                },
                CancelEditRequestMessage(_) => match self.message_text_area.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.message_text_area.key_event(key, Some(terminal)),
                },
                KeyEventEditRequestMessage(_) => self.message_text_area.key_event(key, Some(terminal)),

                /* Scripts */

                ModifyRequestPreRequestScript(_) => match self.script_console.pre_request_text_area.is_in_default_mode() {
                    true => self.tui_modify_pre_request_script(),
                    false => self.script_console.pre_request_text_area.key_event(key, Some(terminal)),
                },
                CancelEditRequestPreRequestScript(_) => match self.script_console.pre_request_text_area.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.script_console.pre_request_text_area.key_event(key, Some(terminal)),
                },
                KeyEventEditRequestPreRequestScript(_) => self.script_console.pre_request_text_area.key_event(key, Some(terminal)),


                ModifyRequestPostRequestScript(_) => match self.script_console.post_request_text_area.is_in_default_mode() {
                    true => self.tui_modify_post_request_script(),
                    false => self.script_console.post_request_text_area.key_event(key, None),
                },
                CancelEditRequestPostRequestScript(_) => match self.script_console.post_request_text_area.is_in_default_mode() {
                    true => self.select_request_state(),
                    false => self.script_console.post_request_text_area.key_event(key, None),
                },
                KeyEventEditRequestPostRequestScript(_) => self.script_console.post_request_text_area.key_event(key, None),

                /* Settings */

                RequestSettingsMoveUp(_) => self.request_settings_popup.previous(),
                RequestSettingsMoveDown(_) => self.request_settings_popup.next(),
                RequestSettingsToggleSettingLeft(_) => self.request_settings_popup.toggle_setting_left(),
                RequestSettingsToggleSettingRight(_) => self.request_settings_popup.toggle_setting_right(),
                ModifyRequestSettings(_) => self.tui_modify_request_settings(),

                /* Others */

                Documentation(_) => {},
            }
        };

        return miss_input;
    }
}
