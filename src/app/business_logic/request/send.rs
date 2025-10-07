use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::RwLock;
use reqwest::multipart::Part;
use reqwest::{ClientBuilder, Proxy, Url};
use reqwest::header::HeaderMap;
use reqwest::redirect::Policy;
use reqwest_middleware::Extension;
use reqwest_tracing::{DisableOtelPropagation, OtelName, TracingMiddleware};
use thiserror::Error;
use tracing_log::log::trace;
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use serde_json::{Value, Map};
use crate::app::app::App;
use crate::app::business_logic::request::scripts::{execute_post_request_script, execute_pre_request_script};
use crate::app::business_logic::request::send::RequestResponseError::PostRequestScript;
use crate::app::files::environment::save_environment_to_file;
use crate::models::auth::Auth::{NoAuth, BasicAuth, BearerToken, JwtToken};
use crate::models::protocol::http::body::ContentType::{NoBody, File, Form, Html, Javascript, Json, Multipart, Raw, Xml};
use crate::models::environment::Environment;
use crate::models::protocol::protocol::Protocol;
use crate::models::request::Request;
use crate::models::response::RequestResponse;
use crate::panic_error;

#[derive(Error, Debug)]
pub enum PrepareRequestError {
    #[error("(CONSOLE) PRE-REQUEST SCRIPT ERROR")]
    PreRequestScript,
    #[error("INVALID URL")]
    InvalidUrl,
    #[error("COULD NOT OPEN FILE")]
    CouldNotOpenFile
}

#[derive(Error, Debug)]
pub enum RequestResponseError {
    #[error("(CONSOLE) POST-SCRIPT ERROR")]
    PostRequestScript,
    #[error("COULD NOT DECODE RESPONSE TEXT OR BYTES")]
    CouldNotDecodeResponse,
    #[error(transparent)]
    WebsocketError(#[from] anyhow::Error),
}

impl App<'_> {
    #[allow(deprecated)]
    pub async fn prepare_request(&self, request: &mut Request) -> Result<reqwest_middleware::RequestBuilder, PrepareRequestError> {
        trace!("Preparing request");

        let env = self.get_selected_env_as_local();

        let mut client_builder = ClientBuilder::new()
            .default_headers(HeaderMap::new())
            .referer(false);

        /* REDIRECTS */

        if !request.settings.allow_redirects.as_bool() {
            client_builder = client_builder.redirect(Policy::none());
        }

        /* STORE COOKIES */

        let should_store_cookies = request.settings.store_received_cookies.as_bool();

        client_builder = client_builder.cookie_store(should_store_cookies);

        /* PROXY */

        if request.settings.use_config_proxy.as_bool() {
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

        /* PRE-REQUEST SCRIPT */

        let modified_request = self.handle_pre_request_script(request, env)?;

        /* INVALID CERTS */

        if request.settings.accept_invalid_certs.as_bool() {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }

        /* INVALID HOSTNAMES */

        if request.settings.accept_invalid_hostnames.as_bool() {
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

        let method = match &modified_request.protocol {
            Protocol::HttpRequest(http_request) => http_request.method.to_reqwest(),
            Protocol::WsRequest(_) => reqwest::Method::GET,
        };

        let mut request_builder = client.request(
            method,
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
                let username = self.replace_env_keys_by_value(username);
                let password = self.replace_env_keys_by_value(password);

                request_builder = request_builder.basic_auth(username, Some(password));
            }
            BearerToken { token: bearer_token } => {
                let bearer_token = self.replace_env_keys_by_value(bearer_token);

                request_builder = request_builder.bearer_auth(bearer_token);
            }
            JwtToken {algorithm, secret, payload } => {
                let algorithm = self.replace_env_keys_by_value(algorithm);
                let secret = self.replace_env_keys_by_value(secret);
                let payload = self.replace_env_keys_by_value(payload);

                let token = do_jaat(algorithm, secret, payload);
                let bearer_token = format!("Authorization: Bearer {}", token);
                request_builder = request_builder.bearer_auth(bearer_token);
            }
        }

        /* BODY */

        if let Protocol::HttpRequest(http_request) = &modified_request.protocol {
            match &http_request.body {
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
        }

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

        Ok(request_builder)
    }

    pub fn handle_pre_request_script(&self, request: &mut Request, env: Option<Arc<RwLock<Environment>>>) -> anyhow::Result<Request, PrepareRequestError> {
        match &request.scripts.pre_request_script {
            None => {
                request.console_output.pre_request_output = None;
                Ok(request.clone())
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

                request.console_output.pre_request_output = Some(console_output);

                match result_request {
                    None => {
                        return Err(PrepareRequestError::PreRequestScript);
                    }
                    Some(request) => Ok(request)
                }
            }
        }
    }

    pub fn handle_post_request_script(request: &Request, response: RequestResponse, env: &Option<Arc<RwLock<Environment>>>) -> anyhow::Result<(RequestResponse, Option<String>), RequestResponseError> {
        match &request.scripts.post_request_script {
            None => {
                Ok((response, None))
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
                    None => Err(PostRequestScript),
                    Some(result_response) => Ok((result_response, Some(result_console_output)))
                }
            }
        }
    }
}

pub fn get_file_content_with_name(path: PathBuf) -> std::io::Result<(Vec<u8>, String)> {
    let mut buffer: Vec<u8> = vec![];
    let mut file = std::fs::File::open(path.clone())?;

    file.read_to_end(&mut buffer)?;

    let file_name = path.file_name().unwrap().to_str().unwrap();

    return Ok((buffer, file_name.to_string()));
}


pub fn algorithm_from_str(alg: &str) -> Option<Algorithm> {
    match alg {
        "HS256" => Some(Algorithm::HS256),
        "HS384" => Some(Algorithm::HS384),
        "HS512" => Some(Algorithm::HS512),
        "RS256" => Some(Algorithm::RS256),
        "RS384" => Some(Algorithm::RS384),
        "RS512" => Some(Algorithm::RS512),
        "ES256" => Some(Algorithm::ES256),
        "ES384" => Some(Algorithm::ES384),
        "PS256" => Some(Algorithm::PS256),
        "PS384" => Some(Algorithm::PS384),
        "PS512" => Some(Algorithm::PS512),
        _ => None,
    }
}

pub fn do_jaat(algorithm: String, secret: String, payload: String) ->  String {
    let claims: Map<String, Value> = serde_json::from_str(payload.as_ref()).expect("Invalid JSON payload");
    let alg = algorithm_from_str(&algorithm).unwrap_or(Algorithm::HS512);
    let header = Header::new(alg); 
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref())).expect("JWT encoding failed")
}
