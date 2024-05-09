use ratatui::style::Stylize;
use crate::app::app::App;
use crate::models::auth::Auth;
use crate::models::body::ContentType;
use crate::models::request::KeyValue;

impl App<'_> {
    pub fn describe_request(&mut self, collection_index: usize, request_index: usize) -> anyhow::Result<()> {
        let local_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));
        let request = local_request.read();

        println!("name: {}", request.name);
        println!("method: {}", request.method);
        println!("url: {}", request.url_with_params_to_string());

        if !request.headers.is_empty() {
            println!("headers:");
            print_key_value_vector(&request.headers);
        }

        match &request.auth {
            Auth::NoAuth => {}
            Auth::BasicAuth(username, password) => println!("auth: Basic\n\t{username}\n\t{password}"),
            Auth::BearerToken(bearer_token) => println!("auth: Bearer token\n\t{bearer_token}"),
        }

        match &request.body {
            ContentType::NoBody => {}
            ContentType::File(file) => println!("body: {}\n{file}", &request.body.to_string()),
            ContentType::Multipart(form) | ContentType::Form(form) => {
                println!("body: {}\n", &request.body.to_string());
                print_key_value_vector(form);
            },
            ContentType::Raw(body) | ContentType::Json(body) | ContentType::Xml(body) | ContentType::Html(body) | ContentType::Javascript(body) => {
                println!("body: {}\n{body}", &request.body.to_string());
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

fn print_key_value_vector(vector: &Vec<KeyValue>) {
    for key_value in vector {
        let header_text = format!("\t{}: {}", key_value.data.0, key_value.data.1);

        if key_value.enabled {
            println!("{}", header_text);
        }
        else {
            println!("{}", header_text.dark_gray());
        }
    }
}