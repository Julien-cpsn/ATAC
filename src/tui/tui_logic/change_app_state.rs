use crate::app::app::App;
use crate::app::log::{LOGS, SHOULD_RECORD_LOGS};
use crate::models::export::ExportFormat;
use crate::models::protocol::http::body::ContentType;
use crate::models::protocol::protocol::Protocol;
use crate::tui::app_states::AppState;
use crate::tui::ui::param_tabs::param_tabs::RequestParamsTabs;
use crate::tui::utils::stateful::cookie_table::cookie_to_row;
use edtui::actions::MoveToEndOfLine;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use strum::VariantArray;

impl App<'_> {
    fn set_app_state(&mut self, app_state: AppState) {
        match self.state {
            AppState::Normal => self.was_last_state_selected_request = false,
            AppState::SelectedRequest => self.was_last_state_selected_request = true,
            _ => {}
        }
        self.state = app_state;
    }

    pub fn normal_state(&mut self) {
        SHOULD_RECORD_LOGS.store(true, Ordering::SeqCst);

        if self.was_last_state_selected_request && self.state != AppState::SelectedRequest {
            self.state = AppState::SelectedRequest;
        }
        else {
            self.state = AppState::Normal;
        }
    }

    pub fn display_env_editor_state(&mut self) {
        if self.get_selected_env_as_local().is_none() {
            return;
        }

        self.tui_update_env_variable_table();
        self.set_app_state(AppState::DisplayingEnvEditor);
    }

    pub fn edit_env_variable_state(&mut self) {
        let local_env = self.get_selected_env_as_local();

        if let Some(local_env) = local_env {
            let env = local_env.read();

            if !env.values.is_empty() {
                let selection = self.env_editor_table.selection.unwrap();

                let pair = env.values.get_index(selection.0).unwrap();
                let text = match selection.1 {
                    0 => pair.0,
                    1 => pair.1,
                    _ => unreachable!()
                };

                self.env_editor_table.selection_text_input.reset_mode();
                self.env_editor_table.selection_text_input.clear();
                self.env_editor_table.selection_text_input.push_str(&text);
                self.env_editor_table.selection_text_input.move_cursor_line_end();
                self.set_app_state(AppState::EditingEnvVariable);
            }
        }
    }
    
    pub fn display_cookies_state(&mut self) {
        let local_cookie_store = Arc::clone(&self.cookies_popup.cookie_store);

        self.cookies_popup.cookies_table.rows = vec![];

        for cookie in local_cookie_store.read().unwrap().iter_any() {
            self.cookies_popup.cookies_table.rows.push(cookie_to_row(cookie))
        }

        self.tui_update_cookies_table_selection();
        self.set_app_state(AppState::DisplayingCookies);
    }

    #[allow(dead_code)]
    pub fn edit_cookie_state(&mut self) {
        let selection = self.cookies_popup.cookies_table.selection.unwrap();

        let input_text = self.cookies_popup.cookies_table.rows[selection.0][selection.1].clone();

        self.cookies_popup.cookies_table.selection_text_input.clear();
        self.cookies_popup.cookies_table.selection_text_input.push_str(&input_text);
        self.cookies_popup.cookies_table.selection_text_input.move_cursor_line_end();

        self.set_app_state(AppState::EditingCookies);
    }

    pub fn display_logs_state(&mut self) {
        SHOULD_RECORD_LOGS.store(false, Ordering::SeqCst);
        let logs = LOGS.lock();

        let mut max_log_width = 0;

        for log in logs.iter() {
            let width = log.0.len() + log.1.as_str().len() + log.2.len() + log.3.len();
            if width > max_log_width {
                max_log_width = width;
            }
        }

        self.logs_vertical_scrollbar.max_scroll = logs.len().saturating_sub(1) as u16;
        self.logs_horizontal_scrollbar.max_scroll = max_log_width as u16;

        self.set_app_state(AppState::DisplayingLogs);
    }

    pub fn choose_element_to_create_state(&mut self) {
        self.creation_popup.selection = 0;
        
        if self.collections.is_empty() {
            self.create_new_collection_state();
        }
        else {
            self.set_app_state(AppState::ChoosingElementToCreate);
        }
    }
    
    pub fn create_new_collection_state(&mut self) {
        self.set_app_state(AppState::CreatingNewCollection);
    }

    pub fn create_new_request_state(&mut self) {
        let collections_length = self.collections.len();

        // Cannot create a request if there is no collection
        if collections_length == 0 {
            return;
        }
        
        let selected_collection = &self.collections_tree.state.selected();

        // If a collection is already selected, automatically selects it in the popup
        let popup_selected_collection_index = if selected_collection.len() > 0 {
            selected_collection[0]
        }
        else {
            0
        };
        
        self.new_request_popup.selected_collection = popup_selected_collection_index;
        self.new_request_popup.max_collection_selection = collections_length;
        self.set_app_state(AppState::CreatingNewRequest);
    }

    pub fn delete_collection_state(&mut self) {
        self.delete_collection_popup.state = false;
        self.set_app_state(AppState::DeletingCollection);
    }

    pub fn delete_request_state(&mut self) {
        self.delete_request_popup.state = false;
        self.set_app_state(AppState::DeletingRequest);
    }

    pub fn rename_collection_state(&mut self) {
        let selected_request_index = self.collections_tree.state.selected();

        let collection_name = &self.collections[selected_request_index[0]].name;
        self.rename_collection_input.clear();
        self.rename_collection_input.push_str(collection_name);
        self.rename_collection_input.state.execute(MoveToEndOfLine());
        
        self.set_app_state(AppState::RenamingCollection);
    }

    pub fn rename_request_state(&mut self) {
        let selected_request_index = self.collections_tree.state.selected();

        {
            let selected_request = self.collections[selected_request_index[0]].requests[selected_request_index[1]].read();
            self.rename_request_input.clear();
            self.rename_request_input.push_str(&selected_request.name);
            self.rename_request_input.state.execute(MoveToEndOfLine());
        }

        self.set_app_state(AppState::RenamingRequest);
    }
    
    pub fn select_request_state(&mut self) {
        self.set_app_state(AppState::SelectedRequest);
        self.update_inputs();
        self.reset_cursors();
    }

    pub fn edit_request_url_state(&mut self) {
        self.set_app_state(AppState::EditingRequestUrl);
        self.update_inputs();
    }

    pub fn edit_request_param_state(&mut self) {
        self.set_app_state(AppState::EditingRequestParam);
        self.update_inputs();
    }

    pub fn edit_request_auth_username_state(&mut self) {
        self.set_app_state(AppState::EditingRequestAuthBasicUsername);
        self.update_inputs();
    }

    pub fn edit_request_auth_password_state(&mut self) {
        self.set_app_state(AppState::EditingRequestAuthBasicPassword);
        self.update_inputs();
    }

    pub fn edit_request_auth_bearer_token_state(&mut self) {
        self.set_app_state(AppState::EditingRequestAuthBearerToken);
        self.update_inputs();
    }

    pub fn edit_request_auth_jwt_secret_state(&mut self) {
        self.set_app_state(AppState::EditingRequestAuthJwtSecret);
        self.update_inputs();
    }

    pub fn edit_request_auth_jwt_payload_state(&mut self) {
        self.set_app_state(AppState::EditingRequestAuthJwtPayload);
        self.update_inputs();
    }

    pub fn edit_request_auth_digest_username_state(&mut self) {
        self.set_app_state(AppState::EditingRequestAuthDigestUsername);
        self.update_inputs();
    }

    pub fn edit_request_auth_digest_password_state(&mut self) {
        self.set_app_state(AppState::EditingRequestAuthDigestPassword);
        self.update_inputs();
    }

    pub fn edit_request_auth_digest_domains_state(&mut self) {
        self.set_app_state(AppState::EditingRequestAuthDigestDomains);
        self.update_inputs();
    }

    pub fn edit_request_auth_digest_realm_state(&mut self) {
        self.set_app_state(AppState::EditingRequestAuthDigestRealm);
        self.update_inputs();
    }

    pub fn edit_request_auth_digest_nonce_state(&mut self) {
        self.set_app_state(AppState::EditingRequestAuthDigestNonce);
        self.update_inputs();
    }

    pub fn edit_request_auth_digest_opaque_state(&mut self) {
        self.set_app_state(AppState::EditingRequestAuthDigestOpaque);
        self.update_inputs();
    }

    pub fn edit_request_header_state(&mut self) {
        self.set_app_state(AppState::EditingRequestHeader);
        self.update_inputs();
    }

    pub fn edit_request_body_table_state(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let selected_request = local_selected_request.read();
            let selected_http_request = selected_request.get_http_request().unwrap();

            match &selected_http_request.body {
                ContentType::Multipart(form) | ContentType::Form(form) => {
                    self.body_form_table.rows = form.clone();
                }
                _ => {
                    return;
                }
            }
        }

        self.request_param_tab = RequestParamsTabs::Body;
        self.set_app_state(AppState::EditingRequestBodyTable);
        self.update_inputs();
    }


    pub fn edit_request_body_file_or_string_state(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let selected_request = local_selected_request.read();
            let selected_http_request = selected_request.get_http_request().unwrap();

            match selected_http_request.body {
                ContentType::File(_) => {
                    self.set_app_state(AppState::EditingRequestBodyFile);
                }
                ContentType::Raw(_) | ContentType::Json(_) | ContentType::Xml(_) | ContentType::Html(_) | ContentType::Javascript(_) => {
                    self.set_app_state(AppState::EditingRequestBodyString);
                }
                _ => {
                    return;
                }
            }
        }

        self.request_param_tab = RequestParamsTabs::Body;
        self.update_inputs();
    }

    pub fn edit_request_message_state(&mut self) {
        self.set_app_state(AppState::EditingRequestMessage);
    }

    pub fn edit_request_script_state(&mut self) {
        self.request_param_tab = RequestParamsTabs::Scripts;
        
        match self.script_console.script_selection {
            0 => self.set_app_state(AppState::EditingPreRequestScript),
            1 => self.set_app_state(AppState::EditingPostRequestScript),
            _ => {}
        }
        
        self.update_inputs();
    }

    pub fn edit_request_settings_state(&mut self) {
        self.request_settings_popup.selection = 0;

        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        self.request_settings_popup.settings = selected_request.settings.to_vec();

        self.set_app_state(AppState::EditingRequestSettings);
    }

    pub fn choose_request_export_format_state(&mut self) {
        self.export_request.selection = 0;

        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        self.export_request.choices = match selected_request.protocol {
            Protocol::HttpRequest(_) => ExportFormat::VARIANTS.to_vec(),
            Protocol::WsRequest(_) => vec![ExportFormat::RustReqwest]
        };

        self.set_app_state(AppState::ChoosingRequestExportFormat);
    }

    pub fn display_request_export_state(&mut self) {
        self.set_app_state(AppState::DisplayingRequestExport);
    }
}
