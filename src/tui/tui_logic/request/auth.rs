use rayon::prelude::*;
use tracing::info;
use tui_textarea::TextArea;
use crate::app::app::App;
use crate::models::auth::auth::Auth::{NoAuth, BasicAuth, BearerToken, JwtToken};
use crate::models::auth::auth::next_auth;
use crate::models::auth::jwt::{next_jwt_algorithm, next_jwt_secret_type, previous_jwt_algorithm, previous_jwt_secret_type};

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
            BasicAuth(_) => match self.auth_text_input_selection.selected {
                0 => self.edit_request_auth_username_state(),
                1 => self.edit_request_auth_password_state(),
                _ => {}
            },
            BearerToken(_) => match self.auth_text_input_selection.selected {
                0 => self.edit_request_auth_bearer_token_state(),
                _ => {}
            },
            JwtToken(_) => match self.auth_text_input_selection.selected {
                0 => {},
                1 => {},
                2 => self.edit_request_auth_jwt_secret_state(),
                3 => self.edit_request_auth_jwt_payload_state(),
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

    pub fn tui_request_auth_move_left(&mut self) {
        match self.auth_text_input_selection.selected {
            0 => self.tui_request_auth_previous_jwt_algorithm(),
            1 => self.tui_request_auth_previous_jwt_secret_type(),
            _ => {}
        }
    }

    pub fn tui_request_auth_move_right(&mut self) {
        match self.auth_text_input_selection.selected {
            0 => self.tui_request_auth_next_jwt_algorithm(),
            1 => self.tui_request_auth_next_jwt_secret_type(),
            _ => {}
        }
    }

    pub fn refresh_auth_jwt_payload_textarea(&mut self, text: &String) {
        let lines: Vec<String> = text
            .par_lines()
            .map(|line| line.to_string())
            .collect();

        self.auth_jwt_payload_text_area = TextArea::new(lines);
    }

    pub fn tui_request_auth_previous_jwt_algorithm(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write();
            let jwt_token = selected_request.auth.get_jwt_mut();

            let new_algorithm = previous_jwt_algorithm(&jwt_token.algorithm);
            let new_secret_type = new_algorithm.default_secret_type();

            info!("Auth JWT algorithm set to \"{}\"", new_algorithm);
            info!("Auth JWT secret type set to \"{}\"", new_secret_type);

            jwt_token.algorithm = new_algorithm;
            jwt_token.secret_type = new_secret_type;
        }

        self.select_request_state();
    }

    pub fn tui_request_auth_next_jwt_algorithm(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write();
            let jwt_token = selected_request.auth.get_jwt_mut();

            let new_algorithm = next_jwt_algorithm(&jwt_token.algorithm);
            let new_secret_type = new_algorithm.default_secret_type();

            info!("Auth JWT algorithm set to \"{}\"", new_algorithm);
            info!("Auth JWT secret type set to \"{}\"", new_secret_type);

            jwt_token.algorithm = new_algorithm;
            jwt_token.secret_type = new_secret_type;
        }

        self.select_request_state();
    }

    pub fn tui_request_auth_previous_jwt_secret_type(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);
        {
            let mut selected_request = local_selected_request.write();
            let jwt_token = selected_request.auth.get_jwt_mut();

            let new_secret_type = previous_jwt_secret_type(&jwt_token.secret_type);

            info!("Auth JWT secret type set to \"{}\"", new_secret_type);

            jwt_token.secret_type = new_secret_type;
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    pub fn tui_request_auth_next_jwt_secret_type(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);
        
        {
            let mut selected_request = local_selected_request.write();
            let jwt_token = selected_request.auth.get_jwt_mut();

            let new_secret_type = next_jwt_secret_type(&jwt_token.secret_type);

            info!("Auth JWT secret type set to \"{}\"", new_secret_type);

            jwt_token.secret_type = new_secret_type;
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    pub fn tui_modify_request_auth_secret(&mut self) {
        let input_text = self.auth_jwt_secret_text_input.text.clone();
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.modify_request_auth_secret(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_modify_request_auth_payload(&mut self) {
        let payload = self.auth_jwt_payload_text_area.lines().join("\n");
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.modify_request_auth_payload(selected_request_index.0, selected_request_index.1, payload);

        self.select_request_state();
    }
}
