use crate::app::app::App;
use crate::models::auth::auth::Auth;
use crate::models::auth::basic::BasicAuth;
use crate::models::auth::bearer_token::BearerToken;
use crate::models::auth::digest::Digest;
use crate::models::auth::jwt::JwtToken;

impl App<'_> {
    pub fn cli_print_request_auth(&mut self, collection_index: usize, request_index: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.read();

            print!("{}\n\t", selected_request.auth);

            match &selected_request.auth {
                Auth::NoAuth => {}
                Auth::BasicAuth(BasicAuth { username, password }) => println!("username: {username}\n\tpassword: {password}"),
                Auth::BearerToken(BearerToken { token: bearer_token }) => println!("token: {bearer_token}"),
                Auth::JwtToken(JwtToken { algorithm, secret_type, secret, payload }) => println!("algorithm: {algorithm}\n\tsecret_type: {secret_type}\n\tsecret: {secret}\n\tpayload: {payload}"),
                Auth::Digest(Digest {
                    username,
                    password,
                    domains,
                    realm,
                    nonce,
                    opaque,
                    stale,
                    algorithm,
                    qop,
                    user_hash,
                    charset,
                    ..
                }) => println!("username: {username}\n\tpassword: {password}\n\tdomains: {domains}\n\trealm: {realm}\n\tnonce: {nonce}\n\topaque: {opaque}\n\tstale: {}\n\talgorithm: {algorithm}\n\tqop: {qop}\n\tuser_hash: {}\n\tcharset: {charset}", stale.to_string(), user_hash.to_string()),
            }
        }


        Ok(())
    }
}

