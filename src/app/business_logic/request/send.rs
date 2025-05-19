use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use parking_lot::RwLock;

use ratatui::prelude::Line;
use rayon::prelude::*;
use reqwest::{ClientBuilder, Proxy, Url};
use reqwest::header::{CONTENT_TYPE, HeaderMap};
use reqwest::multipart::Part;
use reqwest::redirect::Policy;
use reqwest_tracing::{TracingMiddleware, OtelName, DisableOtelPropagation};
use reqwest_middleware::Extension;
use thiserror::Error;
use tokio_util::sync::CancellationToken;
use tracing::{info, trace};

use crate::app::app::App;
use crate::app::business_logic::request::scripts::{execute_post_request_script, execute_pre_request_script};
use crate::app::business_logic::request::send::RequestResponseError::PostRequestScript;
use crate::app::files::environment::save_environment_to_file;
use crate::models::auth::Auth::{BasicAuth, BearerToken, NoAuth};
use crate::models::body::ContentType::{File, Form, Html, Javascript, Json, Multipart, NoBody, Raw, Xml};
use crate::models::body::find_file_format_in_content_type;
use crate::models::environment::Environment;
use crate::models::request::Request;
use crate::models::response::{ImageResponse, RequestResponse, ResponseContent};
use crate::panic_error;
use crate::tui::utils::syntax_highlighting::highlight;

#[derive(Error, Debug)]
pub enum PrepareRequestError {
    #[error("(CONSOLE) PRE-REQUEST SCRIPT ERROR")]
    PreRequestScript,
    #[error("INVALID URL")]
    InvalidUrl,
    #[error("COULD NOT OPEN FILE")]
    CouldNotOpenFile
}

impl App<'_> {
    pub async fn prepare_request(&self, request: &Request) -> Result<(reqwest_middleware::RequestBuilder, String), PrepareRequestError> {
        trace!("Preparing request");
        
        let env = self.get_selected_env_as_local();

        let mut client_builder = ClientBuilder::new()
            .default_headers(HeaderMap::new())
            .referer(false);

        /* REDIRECTS */

        if !request.settings.allow_redirects {
            client_builder = client_builder.redirect(Policy::none());
        }

        /* STORE COOKIES */

        let should_store_cookies = request.settings.store_received_cookies;

        client_builder = client_builder.cookie_store(should_store_cookies);

        /* PROXY */

        if request.settings.use_config_proxy {
            match &self.config.proxy {
                None => {}
                Some(proxy) => {
                    match &proxy.http_proxy {
                        None => {}
                        Some(http_proxy_str) => {
                            let proxy = match Proxy::http(http_proxy_str) {
                                Ok(proxy) => proxy,
                                Err(e) => panic_error(format!("Could not parse HTTP proxy\n\t{e}"))
                            };
                            client_builder = client_builder.proxy(proxy);
                        }
                    }

                    match &proxy.https_proxy {
                        None => {}
                        Some(https_proxy_str) => {
                            let proxy = match Proxy::https(https_proxy_str) {
                                Ok(proxy) => proxy,
                                Err(e) => panic_error(format!("Could not parse HTTPS proxy\n\t{e}"))
                            };
                            client_builder = client_builder.proxy(proxy);
                        }
                    }
                }
            }
        }

        /* COOKIES */

        let local_cookie_store = Arc::clone(&self.cookies_popup.cookie_store);
        client_builder = client_builder.cookie_provider(local_cookie_store);

        let (modified_request, console_output): (Request, String) = match &request.scripts.pre_request_script {
            None => {
                (request.clone(), String::new())
            },
            Some(pre_request_script) => {

                let env_values = match &env {
                    None => None,
                    Some(local_env) => {
                        let env = local_env.read();
                        Some(env.values.clone())
                    }
                };

                let (result_request, env_variables, console_output) = execute_pre_request_script(pre_request_script, &request, env_values);

                match &env {
                    None => {},
                    Some(local_env) => match env_variables {
                        None => {},
                        Some(env_variables) => {
                            let mut env = local_env.write();
                            env.values = env_variables;
                            save_environment_to_file(&*env);
                        }
                    }
                }

                match result_request {
                    None => {
                        return Err(PrepareRequestError::PreRequestScript);
                    }
                    Some(request) => (request, console_output)
                }
            }
        };

        /* INVALID CERTS */
        
        if request.settings.accept_invalid_certs {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }

        /* INVALID HOSTNAMES */

        if request.settings.accept_invalid_hostnames {
            client_builder = client_builder.danger_accept_invalid_hostnames(true);
        }
        
        /* CLIENT */

        let untraced_client = client_builder.build().expect("Could not build HTTP client");
        let client = reqwest_middleware::ClientBuilder::new(untraced_client)
            .with(TracingMiddleware::default())
            .with_init(Extension(OtelName(modified_request.name.into())))
            .with_init(Extension(DisableOtelPropagation))
            .build();

        /* PARAMS */

        let params = self.key_value_vec_to_tuple_vec(&modified_request.params);
        let query_params = params.iter().filter(|(key, _)| !(key.starts_with("{") && key.ends_with("}")));
        let path_params = params.iter().filter(|(key, _)| key.starts_with("{") && key.ends_with("}"));

        /* URL */

        let mut url = self.replace_env_keys_by_value(&modified_request.url);

        for (key, value) in path_params {
            url = url.replace(key, value);
        }

        let url = if params.is_empty() {
            Url::parse(&url)
        } else {
            // this adds extra "?" when params is empty
            Url::parse_with_params(&url, query_params)
        };

        let url = match url {
            Ok(url) => url,
            Err(_) => {
                return Err(PrepareRequestError::InvalidUrl);
            }
        };

        /* REQUEST */

        let mut request_builder = client.request(
            modified_request.method.to_reqwest(),
            url
        );

        /* CORS */

        if self.config.is_cors_disabled() {
            request_builder = request_builder.fetch_mode_no_cors();
        }

        /* AUTH */

        match &modified_request.auth {
            NoAuth => {}
            BasicAuth { username, password} => {
                let username = self.replace_env_keys_by_value(&username);
                let password = self.replace_env_keys_by_value(&password);

                request_builder = request_builder.basic_auth(username, Some(password));
            }
            BearerToken { token: bearer_token } => {
                let bearer_token = self.replace_env_keys_by_value(&bearer_token);

                request_builder = request_builder.bearer_auth(bearer_token);
            }
        }

        /* BODY */

        match &modified_request.body {
            NoBody => {},
            Multipart(form_data) => {
                let mut multipart = reqwest::multipart::Form::new();

                for form_data in form_data {
                    let key = self.replace_env_keys_by_value(&form_data.data.0);
                    let value = self.replace_env_keys_by_value(&form_data.data.1);

                    // If the value starts with !!, then it is supposed to be a file
                    if value.starts_with("!!") {
                        let path = PathBuf::from(&value[2..]);

                        match get_file_content_with_name(path) {
                            Ok((file_content, file_name)) => {
                                let part = Part::bytes(file_content).file_name(file_name);
                                multipart = multipart.part(key, part);
                            }
                            Err(_) => {
                                return Err(PrepareRequestError::CouldNotOpenFile);
                            }
                        }
                    }
                    else {
                        multipart = multipart.text(key, value);
                    }
                }

                request_builder = request_builder.multipart(multipart);
            },
            Form(form_data) => {
                let form = self.key_value_vec_to_tuple_vec(&form_data);

                request_builder = request_builder.form(&form);
            },
            File(file_path) => {
                let file_path_with_env_values = self.replace_env_keys_by_value(&file_path);
                let path = PathBuf::from(file_path_with_env_values);

                match tokio::fs::File::open(path).await {
                    Ok(file) => {
                        request_builder = request_builder.body(file);
                    }
                    Err(_) => {
                        return Err(PrepareRequestError::CouldNotOpenFile);
                    }
                }
            },
            Raw(body) | Json(body) | Xml(body) | Html(body) | Javascript(body) => {
                let body_with_env_values = self.replace_env_keys_by_value(body);
                request_builder = request_builder.body(body_with_env_values);
            }
        };

        /* HEADERS */

        for header in &modified_request.headers {
            if !header.enabled {
                continue;
            }

            let header_name = self.replace_env_keys_by_value(&header.data.0);
            let header_value = self.replace_env_keys_by_value(&header.data.1);

            request_builder = request_builder.header(header_name, header_value);
        }

        trace!("Request prepared");

        Ok((request_builder, console_output))
    }
}

#[derive(Error, Debug)]
pub enum RequestResponseError {
    #[error("(CONSOLE) POST-SCRIPT ERROR")]
    PostRequestScript,
}

pub async fn send_request(prepared_request: reqwest_middleware::RequestBuilder, local_request: Arc<RwLock<Request>>, env: &Option<Arc<RwLock<Environment>>>) -> Result<(RequestResponse, String, Option<Vec<Line<'static>>>), RequestResponseError> {
    info!("Sending request");

    local_request.write().is_pending = true;

    let request = local_request.read();

    let cancellation_token = request.cancellation_token.clone();
    let timeout = tokio::time::sleep(Duration::from_secs(30));

    let request_start = Instant::now();
    let elapsed_time: Duration;

    let mut highlighted_result_body: Option<Vec<Line>> = None;

    let mut response = tokio::select! {
        _ = cancellation_token.cancelled() => {
            elapsed_time = request_start.elapsed();
            
            RequestResponse {
                duration: None,
                status_code: Some(String::from("CANCELED")),
                content: None,
                cookies: None,
                headers: vec![],
            }
        },
        _ = timeout => {
            elapsed_time = request_start.elapsed();

            RequestResponse {
                duration: None,
                status_code: Some(String::from("TIMEOUT")),
                content: None,
                cookies: None,
                headers: vec![],
            }
        },
        response = prepared_request.send() => match response {
            Ok(response) => {
                elapsed_time = request_start.elapsed();

                let status_code = response.status().to_string();

                let mut is_image = false;

                let headers: Vec<(String, String)> = response.headers().clone()
                    .iter()
                    .map(|(header_name, header_value)| {
                        let value = header_value.to_str().unwrap_or("").to_string();

                        if header_name == CONTENT_TYPE && value.starts_with("image/") {
                            is_image = true;
                        }

                        (header_name.to_string(), value)
                    })
                    .collect();

                let cookies = response.cookies()
                    .par_bridge()
                    .map(|cookie| {
                        format!("{}: {}", cookie.name(), cookie.value())
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                let response_content = match is_image {
                    true => {
                        let content = response.bytes().await.unwrap();
                        let image = image::load_from_memory(content.as_ref());

                        ResponseContent::Image(ImageResponse {
                            data: content.to_vec(),
                            image: image.ok(),
                        })
                    },
                    false => {
                        let mut result_body = match response.text().await {
                            Ok(body) => body,
                            Err(error) => error.to_string()
                        };

                        // If a file format has been found in the content-type header
                        if let Some(file_format) = find_file_format_in_content_type(&headers) {
                            // If the request response content can be pretty printed
                            if request.settings.pretty_print_response_content {
                                // Match the file format
                                match file_format.as_str() {
                                    "json" => {
                                        result_body = jsonxf::pretty_print(&result_body).unwrap_or(result_body);
                                    },
                                    _ => {}
                                }
                            }

                            highlighted_result_body = highlight(&result_body, &file_format);
                        }

                        ResponseContent::Body(result_body)
                    }
                };

                RequestResponse {
                    duration: None,
                    status_code: Some(status_code),
                    content: Some(response_content),
                    cookies: Some(cookies),
                    headers,
                }
            },
            Err(error) => {
                elapsed_time = request_start.elapsed();

                let response_status_code;

                if let Some(status_code) = error.status() {
                    response_status_code = Some(status_code.to_string());
                } else {
                    response_status_code = None;
                }

                let result_body = ResponseContent::Body(error.to_string());

                RequestResponse {
                    duration: None,
                    status_code: response_status_code,
                    content: Some(result_body),
                    cookies: None,
                    headers: vec![],
                }
            }
        }
    };

    response.duration = Some(format!("{:?}", elapsed_time));

    trace!("Request sent");

    /* POST-REQUEST SCRIPT */

    let (modified_response, console_output): (RequestResponse, String) = match &request.scripts.post_request_script {
        None => {
            (response, String::new())
        },
        Some(post_request_script) => {
            let env_values = match &env {
                None => None,
                Some(env) => {
                    let env = env.read();
                    Some(env.values.clone())
                }
            };

            let (result_response, env_variables, result_console_output) = execute_post_request_script(post_request_script, &response, env_values);

            match env {
                None => {},
                Some(env) => match env_variables {
                    None => {},
                    Some(env_variables) => {
                        let mut env = env.write();
                        env.values = env_variables;
                        save_environment_to_file(&*env);
                    }
                }
            }

            match result_response {
                None => {
                    return Err(PostRequestScript)
                }
                Some(result_response) => (result_response, result_console_output)
            }
        }
    };


    drop(request);

    {
        let mut request = local_request.write();

        request.is_pending = false;
        request.cancellation_token = CancellationToken::new();
    }
        
    return Ok((modified_response, console_output, highlighted_result_body));
}

pub fn get_file_content_with_name(path: PathBuf) -> std::io::Result<(Vec<u8>, String)> {
    let mut buffer: Vec<u8> = vec![];
    let mut file = std::fs::File::open(path.clone())?;

    file.read_to_end(&mut buffer)?;

    let file_name = path.file_name().unwrap().to_str().unwrap();

    return Ok((buffer, file_name.to_string()));
}
