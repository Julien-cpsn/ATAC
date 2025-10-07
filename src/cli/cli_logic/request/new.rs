use anyhow::anyhow;
use tokio_util::sync::CancellationToken;
use crate::app::app::App;
use crate::cli::commands::request_commands::new::{AuthArgs, BodyArgs, NewRequestCommand};
use crate::models::auth::Auth;
use crate::models::protocol::http::body::ContentType;
use crate::models::protocol::http::method::Method;
use crate::models::protocol::protocol::Protocol;
use crate::models::request::{ConsoleOutput, KeyValue, Request, DEFAULT_HEADERS};
use crate::models::response::RequestResponse;
use crate::models::scripts::RequestScripts;
use crate::models::settings::{RequestSettings, Setting};

impl App<'_> {
    pub fn cli_new_request(&mut self, collection_slash_request: (String, String), new_request_command: NewRequestCommand) -> anyhow::Result<()> {
        let collection_index = self.find_collection(&collection_slash_request.0)?;
        let new_request = create_request_from_new_request_command(collection_slash_request.1.trim().to_string(), new_request_command)?;
        
        self.new_request(collection_index, new_request)?;
        
        Ok(())
    }
}

pub fn create_request_from_new_request_command(request_name: String, new_request_command: NewRequestCommand) -> anyhow::Result<Request> {
    let params = string_array_to_key_value_array(new_request_command.add_param);
    let auth = get_auth_from_auth_args(new_request_command.auth);
    let headers = string_array_to_key_value_array(new_request_command.add_header);
    let body = get_content_type_from_body_args(new_request_command.body);

    let base_headers = match new_request_command.no_base_headers {
        true => vec![],
        false => DEFAULT_HEADERS.clone()
    };

    let mut protocol = new_request_command.protocol.clone();

    match &mut protocol {
        Protocol::HttpRequest(http_request) => {
            http_request.method = new_request_command.method;
            http_request.body = body;
        }
        Protocol::WsRequest(_) => {
            match new_request_command.method {
                Method::GET => {}
                _ => return Err(anyhow!("Setting a method with a websocket request is incompatible"))
            }

            match body {
                ContentType::NoBody => {}
                _ => return Err(anyhow!("Setting a body with a websocket request body is incompatible"))
            }
        }
    };

    let mut request = Request {
        name: request_name,
        url: String::new(),
        protocol,
        params,
        auth,
        headers: vec![base_headers, headers].concat(),
        scripts: RequestScripts {
            pre_request_script: new_request_command.pre_request_script,
            post_request_script: new_request_command.post_request_script,
        },
        settings: RequestSettings {
            use_config_proxy: Setting::Bool(!new_request_command.no_proxy),
            allow_redirects: Setting::Bool(!new_request_command.no_redirects),
            timeout: Setting::U32(new_request_command.timeout),
            store_received_cookies: Setting::Bool(!new_request_command.no_cookies),
            pretty_print_response_content: Setting::Bool(!new_request_command.no_pretty),
            accept_invalid_certs: Setting::Bool(new_request_command.accept_invalid_certs),
            accept_invalid_hostnames: Setting::Bool(new_request_command.accept_invalid_hostnames),
        },
        response: RequestResponse::default(),
        console_output: ConsoleOutput::default(),
        is_pending: false,
        cancellation_token: CancellationToken::new(),
    };

    request.update_url_and_params(new_request_command.url);

    Ok(request)
}

fn string_array_to_key_value_array(string_array: Vec<String>) -> Vec<KeyValue> {
    let mut key_value_array: Vec<KeyValue> = vec![];

    for i in (0..string_array.len()).step_by(2) {
        key_value_array.push(KeyValue {
            enabled: true,
            data: (string_array[i].clone(), string_array[i + 1].clone()),
        })
    }

    return key_value_array
}

fn get_auth_from_auth_args(auth_args: AuthArgs) -> Auth {
    if !auth_args.auth_basic.is_empty() {
        return Auth::BasicAuth {
            username: auth_args.auth_basic[0].clone(),
            password: auth_args.auth_basic[1].clone()
        };
    }
    else if !auth_args.auth_bearer_token.is_empty() {
        return Auth::BearerToken {
            token: auth_args.auth_bearer_token[0].clone()
        };
    }
    else if !auth_args.auth_jwt_token.is_empty() {
        return Auth::JwtToken {
            algorythm: auth_args.auth_jwt_token[0].clone(),
            secret: auth_args.auth_jwt_token[1].clone(),
            payload: auth_args.auth_jwt_token[2].clone(),
        };
    }
    else {
        return Auth::NoAuth;
    }
}

fn get_content_type_from_body_args(body_args: BodyArgs) -> ContentType {
    if let Some(file_path) = &body_args.body_file {
        return ContentType::File(file_path.clone());
    }
    else if !body_args.add_body_multipart.is_empty() {
        let multipart_key_values = string_array_to_key_value_array(body_args.add_body_multipart);
        return ContentType::Multipart(multipart_key_values);
    }
    else if !body_args.add_body_form.is_empty() {
        let form_key_values = string_array_to_key_value_array(body_args.add_body_multipart);
        return ContentType::Form(form_key_values);
    }
    else if let Some(raw) = &body_args.body_raw {
        return ContentType::Raw(raw.clone());
    }
    else if let Some(json) = &body_args.body_json {
        return ContentType::Json(json.clone());
    }
    else if let Some(xml) = &body_args.body_xml {
        return ContentType::Xml(xml.clone());
    }
    else if let Some(html) = &body_args.body_html {
        return ContentType::Html(html.clone());
    }
    else if let Some(javascript) = &body_args.body_javascript {
        return ContentType::Javascript(javascript.clone());
    }
    else {
        return ContentType::NoBody;
    }
}
