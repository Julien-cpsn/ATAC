use tracing::{info};
use crate::app::app::App;
use crate::models::auth::auth::Auth;
use crate::models::auth::basic::BasicAuth;
use crate::models::auth::bearer_token::BearerToken;

impl App<'_> {
    pub fn modify_request_auth(&mut self, collection_index: usize, request_index: usize, auth: Auth) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            info!("Auth method set to \"{}\"", auth);

            selected_request.auth = auth;
        }

        self.save_collection_to_file(collection_index);

        Ok(())
    }
    
    pub fn modify_request_auth_basic_username(&mut self, collection_index: usize, request_index: usize, basic_auth_username: String) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            info!("Auth basic username set to \"{}\"", basic_auth_username);

            match &selected_request.auth {
                Auth::BasicAuth(BasicAuth { password, .. }) => {
                    selected_request.auth = Auth::BasicAuth(BasicAuth {
                        username: basic_auth_username,
                        password: password.to_string()
                    });
                }
                _ => {}
            }
        }

        self.save_collection_to_file(collection_index);
    }

    pub fn modify_request_auth_basic_password(&mut self, collection_index: usize, request_index: usize, basic_auth_password: String) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            info!("Auth basic password set to \"{}\"", basic_auth_password);
            
            match &selected_request.auth {
                Auth::BasicAuth(BasicAuth { username, .. }) => {
                    selected_request.auth = Auth::BasicAuth(BasicAuth {
                        username: username.to_string(),
                        password: basic_auth_password
                    });
                }
                _ => {}
            }
        }

        self.save_collection_to_file(collection_index);
    }

    pub fn modify_request_auth_bearer_token(&mut self, collection_index: usize, request_index: usize, bearer_token: String) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();
            
            info!("Auth bearer token set to \"{}\"", bearer_token);
            
            match &selected_request.auth {
                Auth::BearerToken(BearerToken { .. }) => {
                    selected_request.auth = Auth::BearerToken(BearerToken { token: bearer_token });
                }
                _ => {}
            }
        }

        self.save_collection_to_file(collection_index);
    }

    pub fn modify_request_auth_secret(&mut self, collection_index: usize, request_index: usize, secret: String) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();
            let jwt_token = selected_request.auth.get_jwt_mut();

            info!("Auth JWT secret set to \"{}\"", secret);

            jwt_token.secret = secret.to_string();
        }

        self.save_collection_to_file(collection_index);
    }

    pub fn modify_request_auth_payload(&mut self, collection_index: usize, request_index: usize, payload: String) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();
            let jwt_token = selected_request.auth.get_jwt_mut();

            info!("Auth JWT payload set to \"{}\"", payload);
            
            jwt_token.payload = payload;
        }

        self.save_collection_to_file(collection_index);
    }

}
