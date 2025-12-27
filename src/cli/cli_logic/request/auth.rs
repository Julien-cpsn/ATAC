use crate::app::app::App;
use crate::models::auth::auth::Auth;
use crate::models::auth::basic::BasicAuth;
use crate::models::auth::bearer_token::BearerToken;
use crate::models::auth::jwt::JwtToken;

impl App<'_> {
    pub fn cli_print_request_auth(&mut self, collection_index: usize, request_index: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.read();

            print!("{}", selected_request.auth);

            match &selected_request.auth {
                Auth::NoAuth => {},
                Auth::BasicAuth(BasicAuth { username, password }) => print!(" {username} {password}"),
                Auth::BearerToken(BearerToken { token: bearer_token }) => print!(" {bearer_token}"),
                Auth::JwtToken(JwtToken { algorithm, secret_type, secret, payload  }) => print!(" {algorithm} {secret_type} {secret} {payload}")
            }

            println!()
        }

        Ok(())
    }
}

