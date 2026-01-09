use crate::app::app::App;
use crate::app::business_logic::key_value::print_key_value_vector;
use crate::models::auth::auth::Auth;
use crate::models::auth::basic::BasicAuth;
use crate::models::auth::bearer_token::BearerToken;
use crate::models::auth::digest::Digest;
use crate::models::auth::jwt::JwtToken;
use crate::models::protocol::http::body::ContentType;
use crate::models::protocol::protocol::Protocol;

impl App<'_> {
    pub fn cli_describe_request(&mut self, collection_index: usize, request_index: usize) -> anyhow::Result<()> {
        let local_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));
        let request = local_request.read();

        println!("name: {}", request.name);
        println!("protocol: {}", request.protocol.to_string());

        if let Protocol::HttpRequest(request) = &request.protocol {
            println!("method: {}", request.method);
        }

        println!("url: {}", request.url_with_params_to_string());

        if !request.headers.is_empty() {
            println!("headers:");
            print_key_value_vector(&request.headers, Some("\t"));
        }

        match &request.auth {
            Auth::NoAuth => {}
            Auth::BasicAuth(BasicAuth { username, password }) => println!("auth: Basic\n\tusername: {username}\n\tpassword: {password}"),
            Auth::BearerToken(BearerToken { token: bearer_token }) => println!("auth: Bearer token\n\ttoken: {bearer_token}"),
            Auth::JwtToken(JwtToken { algorithm, secret_type, secret, payload }) => println!("auth: JWT\n\talgorithm: {algorithm}\n\tsecret_type: {secret_type}\n\tsecret: {secret}\n\tpayload: {payload}"),
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
             }) => println!("auth: Digest\n\tusername: {username}\n\tpassword: {password}\n\tdomains: {domains}\n\trealm: {realm}\n\tnonce: {nonce}\n\topaque: {opaque}\n\tstale: {}\n\talgorithm: {algorithm}\n\tqop: {qop}\n\tuser_hash: {}\n\tcharset: {charset}", stale.to_string(), user_hash.to_string()),
        }

        if let Protocol::HttpRequest(http_request) = &request.protocol {
            match &http_request.body {
                ContentType::NoBody => {}
                ContentType::File(file) => println!("body: {}\n{file}", &http_request.body.to_string()),
                ContentType::Multipart(form) | ContentType::Form(form) => {
                    println!("body: {}", &http_request.body.to_string());
                    print_key_value_vector(form, Some("\t"));
                },
                ContentType::Raw(body) | ContentType::Json(body) | ContentType::Xml(body) | ContentType::Html(body) | ContentType::Javascript(body) => {
                    println!("body: {}\n{body}", &http_request.body.to_string());
                }
            }
        }
        
        match (request.scripts.pre_request_script.is_some(), request.scripts.post_request_script.is_some()) {
            (false, false) => {}
            (true, false) => println!("scripts:\n\tpre-request"),
            (false, true) => println!("scripts:\n\tpost-request"),
            (true, true) => println!("scripts:\n\tpre and post-request"),
        }

        Ok(())
    }
}
