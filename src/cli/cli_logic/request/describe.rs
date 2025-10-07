use crate::app::app::App;
use crate::app::business_logic::key_value::print_key_value_vector;
use crate::models::auth::Auth;
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
            Auth::BasicAuth { username, password } => println!("auth: Basic\n\t{username}\n\t{password}"),
            Auth::BearerToken { token: bearer_token } => println!("auth: Bearer token\n\t{bearer_token}"),
            Auth::JwtToken { algorithm, secret, payload } => println!("auth: JWT\n\t{algorithm}\n\t{secret}\n\t{payload}"),
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
                    println!("body: {}{body}", &http_request.body.to_string());
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
