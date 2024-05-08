use crate::app::app::App;
use crate::models::auth::Auth::{BasicAuth, BearerToken};
use crate::models::auth::Auth;

impl App<'_> {
    pub fn modify_request_auth(&mut self, auth: Auth, collection_index: usize, request_index: usize) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            selected_request.auth = auth;
        }

        self.save_collection_to_file(collection_index);
    }

    pub fn modify_request_auth_basic_username(&mut self, basic_auth_username: String, collection_index: usize, request_index: usize) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            match &selected_request.auth {
                BasicAuth(_, password) => {
                    selected_request.auth = BasicAuth(basic_auth_username, password.to_string());
                }
                _ => {}
            }
        }

        self.save_collection_to_file(collection_index);
    }

    pub fn modify_request_auth_basic_password(&mut self, basic_auth_password: String, collection_index: usize, request_index: usize) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            match &selected_request.auth {
                BasicAuth(username, _) => {
                    selected_request.auth = BasicAuth(username.to_string(), basic_auth_password);
                }
                _ => {}
            }
        }

        self.save_collection_to_file(collection_index);
    }

    pub fn modify_request_auth_bearer_token(&mut self, bearer_token: String, collection_index: usize, request_index: usize) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            match &selected_request.auth {
                BearerToken(_) => {
                    selected_request.auth = BearerToken(bearer_token);
                }
                _ => {}
            }
        }

        self.save_collection_to_file(collection_index);
    }
}