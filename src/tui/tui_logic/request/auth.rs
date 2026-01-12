use tracing::info;
use crate::app::app::App;
use crate::models::auth::auth::Auth::{NoAuth, BasicAuth, BearerToken, JwtToken, Digest};
use crate::models::auth::auth::next_auth;
use crate::models::auth::digest::{next_digest_algorithm, next_digest_qop, previous_digest_algorithm, previous_digest_qop, toggle_digest_charset};
use crate::models::auth::jwt::{next_jwt_algorithm, next_jwt_secret_type, previous_jwt_algorithm, previous_jwt_secret_type};
use crate::tui::ui::views::RequestView;

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
            Digest(_) => match self.auth_text_input_selection.selected {
                0 => self.edit_request_auth_digest_username_state(),
                1 => self.edit_request_auth_digest_password_state(),
                2 => self.edit_request_auth_digest_domains_state(),
                3 => self.edit_request_auth_digest_realm_state(),
                4 => self.edit_request_auth_digest_nonce_state(),
                5 => self.edit_request_auth_digest_opaque_state(),
                _ => {}
            }
        }
    }

    pub fn tui_modify_request_auth_basic_username(&mut self) {
        let input_text = self.auth_basic_username_text_input.to_string();
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.modify_request_auth_basic_username(selected_request_index.0, selected_request_index.1, input_text);
            
        self.select_request_state();
    }

    pub fn tui_modify_request_auth_basic_password(&mut self) {
        let input_text = self.auth_basic_password_text_input.to_string();
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.modify_request_auth_basic_password(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_modify_request_auth_bearer_token(&mut self) {
        let input_text = self.auth_bearer_token_text_input.to_string();
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.modify_request_auth_bearer_token(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_request_auth_move_left(&mut self) {
        if matches!(self.request_view, RequestView::OnlyResult) {
            return;
        }

        let request_auth = {
            let local_selected_request = self.get_selected_request_as_local();
            let selected_request = local_selected_request.read();
            selected_request.auth.clone()
        };

        match request_auth {
            NoAuth => {}
            BasicAuth(_) => {}
            BearerToken(_) => {}
            JwtToken(_) => match self.auth_text_input_selection.selected {
                0 => self.tui_request_auth_previous_jwt_algorithm(),
                1 => self.tui_request_auth_previous_jwt_secret_type(),
                _ => {}
            },
            Digest(_) => match self.auth_text_input_selection.selected {
                6 => self.tui_request_auth_toggle_digest_stale(),
                7 => self.tui_request_auth_previous_digest_algorithm(),
                8 => self.tui_request_auth_previous_digest_qop(),
                9 => self.tui_request_auth_toggle_digest_user_hash(),
                10 => self.tui_request_auth_toggle_digest_charset(),
                _ => {}
            }
        }
    }

    pub fn tui_request_auth_move_right(&mut self) {
        if matches!(self.request_view, RequestView::OnlyResult) {
            return;
        }

        let request_auth = {
            let local_selected_request = self.get_selected_request_as_local();
            let selected_request = local_selected_request.read();
            selected_request.auth.clone()
        };

        match request_auth {
            NoAuth => {}
            BasicAuth(_) => {}
            BearerToken(_) => {}
            JwtToken(_) => match self.auth_text_input_selection.selected {
                0 => self.tui_request_auth_next_jwt_algorithm(),
                1 => self.tui_request_auth_next_jwt_secret_type(),
                _ => {}
            },
            Digest(_) => match self.auth_text_input_selection.selected {
                6 => self.tui_request_auth_toggle_digest_stale(),
                7 => self.tui_request_auth_next_digest_algorithm(),
                8 => self.tui_request_auth_next_digest_qop(),
                9 => self.tui_request_auth_toggle_digest_user_hash(),
                10 => self.tui_request_auth_toggle_digest_charset(),
                _ => {}
            }
        }
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

    pub fn tui_modify_request_auth_jwt_secret(&mut self) {
        let input_text = self.auth_jwt_secret_text_input.to_string();
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.modify_request_auth_jwt_secret(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_modify_request_auth_jwt_payload(&mut self) {
        let payload = self.auth_jwt_payload_text_area.to_string();
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.modify_request_auth_jwt_payload(selected_request_index.0, selected_request_index.1, payload);

        self.select_request_state();
    }

    pub fn tui_modify_request_auth_digest_username(&mut self) {
        let input_text = self.auth_digest_username_text_input.to_string();
        let selected_request_index = &self.collections_tree.selected.unwrap();

        self.modify_request_auth_digest_username(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_modify_request_auth_digest_password(&mut self) {
        let input_text = self.auth_digest_password_text_input.to_string();
        let selected_request_index = &self.collections_tree.selected.unwrap();

        self.modify_request_auth_digest_password(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_modify_request_auth_digest_domains(&mut self) {
        let input_text = self.auth_digest_domains_text_input.to_string();
        let selected_request_index = &self.collections_tree.selected.unwrap();

        self.modify_request_auth_digest_domains(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_modify_request_auth_digest_realm(&mut self) {
        let input_text = self.auth_digest_realm_text_input.to_string();
        let selected_request_index = &self.collections_tree.selected.unwrap();

        self.modify_request_auth_digest_realm(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_modify_request_auth_digest_nonce(&mut self) {
        let input_text = self.auth_digest_nonce_text_input.to_string();
        let selected_request_index = &self.collections_tree.selected.unwrap();

        self.modify_request_auth_digest_nonce(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_modify_request_auth_digest_opaque(&mut self) {
        let input_text = self.auth_digest_opaque_text_input.to_string();
        let selected_request_index = &self.collections_tree.selected.unwrap();

        self.modify_request_auth_digest_opaque(selected_request_index.0, selected_request_index.1, input_text);

        self.select_request_state();
    }

    pub fn tui_request_auth_toggle_digest_stale(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write();
            let digest = selected_request.auth.get_digest_mut();

            let new_stale = !digest.stale;

            info!("Auth digest stale set to \"{}\"", new_stale);

            digest.stale = new_stale;
        }

        self.select_request_state();
    }

    pub fn tui_request_auth_previous_digest_algorithm(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write();
            let digest = selected_request.auth.get_digest_mut();

            let previous_algorithm = previous_digest_algorithm(&digest.algorithm);

            info!("Auth digest algorithm set to \"{}\"", previous_algorithm);

            digest.algorithm = previous_algorithm;
        }

        self.select_request_state();
    }

    pub fn tui_request_auth_next_digest_algorithm(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write();
            let digest = selected_request.auth.get_digest_mut();

            let new_algorithm = next_digest_algorithm(&digest.algorithm);

            info!("Auth digest algorithm set to \"{}\"", new_algorithm);

            digest.algorithm = new_algorithm;
        }

        self.select_request_state();
    }

    pub fn tui_request_auth_previous_digest_qop(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write();
            let digest = selected_request.auth.get_digest_mut();

            let previous_qop = previous_digest_qop(&digest.qop);

            info!("Auth digest QOP set to \"{}\"", previous_qop);

            digest.qop = previous_qop;
        }

        self.select_request_state();
    }

    pub fn tui_request_auth_next_digest_qop(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write();
            let digest = selected_request.auth.get_digest_mut();

            let new_qop = next_digest_qop(&digest.qop);

            info!("Auth digest QOP set to \"{}\"", new_qop);

            digest.qop = new_qop;
        }

        self.select_request_state();
    }

    pub fn tui_request_auth_toggle_digest_user_hash(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write();
            let digest = selected_request.auth.get_digest_mut();

            let new_user_hash = !digest.user_hash;

            info!("Auth digest user hash set to \"{}\"", new_user_hash);

            digest.user_hash = new_user_hash;
        }

        self.select_request_state();
    }

    pub fn tui_request_auth_toggle_digest_charset(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write();
            let digest = selected_request.auth.get_digest_mut();

            let previous_charset = toggle_digest_charset(&digest.charset);

            info!("Auth digest charset set to \"{}\"", previous_charset);

            digest.charset = previous_charset;
        }

        self.select_request_state();
    }
}
