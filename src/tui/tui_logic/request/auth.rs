use crate::app::app::App;
use crate::models::auth::Auth::{NoAuth, BasicAuth, BearerToken, JwtToken};
use crate::models::auth::next_auth;

impl App<'_> {
    pub fn tui_next_request_auth(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            selected_request.auth = next_auth(&selected_request.auth);
        }
        
        self.save_collection_to_file(selected_request_index.0);
        self.tui_load_request_auth_param_tab();
    }

    pub fn tui_select_request_auth_input_text(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        match selected_request.auth {
            NoAuth => {}
            BasicAuth { .. } => match self.auth_text_input_selection.selected {
                0 => self.edit_request_auth_username_state(),
                1 => self.edit_request_auth_password_state(),
                _ => {}
            },
            BearerToken { .. } => match self.auth_text_input_selection.selected {
                0 => self.edit_request_auth_bearer_token_state(),
                _ => {}
            },
            JwtToken { .. } => match self.auth_text_input_selection.selected {
                0 => self.edit_request_auth_jwt_algorythm_state(),
                1 => self.edit_request_auth_jwt_secret_state(),
                2 => self.edit_request_auth_jwt_payload_state(),
                _ => {}
            }
        }
    }

    pub fn tui_modify_request_auth_basic_username(&mut self) {
        let input_text = self.auth_basic_username_text_input.text.clone();
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.modify_request_auth_basic_username(selected_request_index.0, selected_request_index.1, input_text);
            
        self.select_request_state();
    }

    pub fn tui_modify_request_auth_basic_password(&mut self) {
        let input_text = self.auth_basic_password_text_input.text.clone();
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.modify_request_auth_basic_password(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_modify_request_auth_bearer_token(&mut self) {
        let input_text = self.auth_bearer_token_text_input.text.clone();
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.modify_request_auth_bearer_token(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_modify_request_auth_algorythm(&mut self) {
        let input_text = self.auth_jwt_algorythm_text_input.text.clone();
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.modify_request_auth_algorythm(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_modify_request_auth_secret(&mut self) {
        let input_text = self.auth_jwt_secret_text_input.text.clone();
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.modify_request_auth_secret(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_modify_request_auth_payload(&mut self) {
        let input_text = self.auth_jwt_payload_text_input.text.clone();
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.modify_request_auth_payload(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }
}
