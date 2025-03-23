use std::sync::Arc;

use crate::app::app::App;
use crate::models::body::ContentType;
use crate::tui::app_states::AppState;
use crate::tui::ui::param_tabs::param_tabs::RequestParamsTabs;
use crate::tui::utils::stateful::cookie_table::cookie_to_row;

impl App<'_> {
    pub fn normal_state(&mut self) {
        self.state = AppState::Normal;
    }

    pub fn display_cookies_state(&mut self) {
        let local_cookie_store = Arc::clone(&self.cookies_popup.cookie_store);

        self.cookies_popup.cookies_table.rows = vec![];

        for cookie in local_cookie_store.read().unwrap().iter_any() {
            self.cookies_popup.cookies_table.rows.push(cookie_to_row(cookie))
        }

        self.tui_update_cookies_table_selection();
        self.state = AppState::DisplayingCookies;
    }

    #[allow(dead_code)]
    pub fn edit_cookie_state(&mut self) {
        let selection = self.cookies_popup.cookies_table.selection.unwrap();

        let input_text = self.cookies_popup.cookies_table.rows[selection.0][selection.1].clone();

        self.cookies_popup.cookies_table.selection_text_input.reset_input();
        self.cookies_popup.cookies_table.selection_text_input.enter_str(&input_text);
        self.cookies_popup.cookies_table.selection_text_input.cursor_position = input_text.len();

        self.state = AppState::EditingCookies;
    }

    pub fn choose_element_to_create_state(&mut self) {
        self.creation_popup.selection = 0;
        
        if self.collections.is_empty() {
            self.create_new_collection_state();
        }
        else {
            self.state = AppState::ChoosingElementToCreate;
        }
    }
    
    pub fn create_new_collection_state(&mut self) {
        self.state = AppState::CreatingNewCollection;
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
        self.new_request_popup.max_selection = collections_length;
        self.state = AppState::CreatingNewRequest;
    }

    pub fn delete_collection_state(&mut self) {
        self.delete_collection_popup.state = false;
        self.state = AppState::DeletingCollection;
    }

    pub fn delete_request_state(&mut self) {
        self.delete_request_popup.state = false;
        self.state = AppState::DeletingRequest;
    }

    pub fn rename_collection_state(&mut self) {
        let selected_request_index = self.collections_tree.state.selected();

        let collection_name = &self.collections[selected_request_index[0]].name;
        self.rename_collection_input.text = collection_name.clone();
        self.rename_collection_input.cursor_position = collection_name.len();
        
        self.state = AppState::RenamingCollection;
    }

    pub fn rename_request_state(&mut self) {
        let selected_request_index = self.collections_tree.state.selected();

        {
            let selected_request = self.collections[selected_request_index[0]].requests[selected_request_index[1]].read();
            self.rename_request_input.text = selected_request.name.clone();
            self.rename_request_input.cursor_position = selected_request.name.len();
        }

        self.state = AppState::RenamingRequest;
    }
    
    pub fn select_request_state(&mut self) {
        self.state = AppState::SelectedRequest;
        self.update_inputs();
        self.reset_cursors();
    }

    pub fn edit_request_url_state(&mut self) {
        self.state = AppState::EditingRequestUrl;
        self.update_inputs();
    }

    pub fn edit_request_param_state(&mut self) {
        self.state = AppState::EditingRequestParam;
        self.update_inputs();
    }

    pub fn edit_request_auth_username_state(&mut self) {
        self.state = AppState::EditingRequestAuthUsername;
        self.update_inputs();
    }

    pub fn edit_request_auth_password_state(&mut self) {
        self.state = AppState::EditingRequestAuthPassword;
        self.update_inputs();
    }

    pub fn edit_request_auth_bearer_token_state(&mut self) {
        self.state = AppState::EditingRequestAuthBearerToken;
        self.update_inputs();
    }

    pub fn edit_request_header_state(&mut self) {
        self.state = AppState::EditingRequestHeader;
        self.update_inputs();
    }

    pub fn edit_request_body_table_state(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let selected_request = local_selected_request.read();

            match selected_request.body {
                ContentType::Multipart(_) | ContentType::Form(_) => {}
                _ => {
                    return;
                }
            }
        }

        self.request_param_tab = RequestParamsTabs::Body;
        self.state = AppState::EditingRequestBodyTable;
        self.update_inputs();
    }


    pub fn edit_request_body_file_or_string_state(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let selected_request = local_selected_request.read();

            match selected_request.body {
                ContentType::File(_) => {
                    self.state = AppState::EditingRequestBodyFile;
                }
                ContentType::Raw(_) | ContentType::Json(_) | ContentType::Xml(_) | ContentType::Html(_) | ContentType::Javascript(_) => {
                    self.state = AppState::EditingRequestBodyString;
                }
                _ => {
                    return;
                }
            }
        }

        self.request_param_tab = RequestParamsTabs::Body;
        self.update_inputs();
    }

    pub fn edit_request_script_state(&mut self) {
        self.request_param_tab = RequestParamsTabs::Scripts;
        
        match self.script_console.script_selection {
            0 => self.state = AppState::EditingPreRequestScript,
            1 => self.state = AppState::EditingPostRequestScript,
            _ => {}
        }
        
        self.update_inputs();
    }

    pub fn edit_request_settings_state(&mut self) {
        self.request_settings_popup.selection = 0;

        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        self.request_settings_popup.settings = selected_request.settings.to_vec();

        self.state = AppState::EditingRequestSettings;
    }

    pub fn choose_request_export_format_state(&mut self) {
        self.export_request.selection = 0;
        self.state = AppState::ChoosingRequestExportFormat;
    }

    pub fn display_request_export_state(&mut self) {
        self.state = AppState::DisplayingRequestExport;
    }
}