use crate::app::app::App;
use crate::models::auth::Auth;

impl App<'_> {
    pub fn cli_print_request_auth(&mut self, collection_index: usize, request_index: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.read();

            print!("{}", selected_request.auth);

            match &selected_request.auth {
                Auth::NoAuth => {},
                Auth::BasicAuth { username, password } => print!(" {username} {password}"),
                Auth::BearerToken { token: bearer_token } => print!(" {bearer_token}"),
                Auth::JwtToken { payload, secret, algorythm } => print!(" {algorythm} {secret} {payload}")
            }

            println!()
        }

        Ok(())
    }
}

