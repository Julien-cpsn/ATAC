use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use ratatui::style::Stylize;
use ratatui::text::Line;
use reqwest::{ClientBuilder, Proxy, Url};
use reqwest::header::{CONTENT_TYPE, HeaderMap};
use reqwest::multipart::{Form, Part};
use reqwest::redirect::Policy;
use tokio::task;

use crate::app::app::App;
use crate::app::app_logic::request::scripts::{execute_post_request_script, execute_pre_request_script};
use crate::app::files::environment::save_environment_to_file;
use crate::panic_error;
use crate::request::auth::Auth::{BasicAuth, BearerToken, NoAuth};
use crate::request::body::{ContentType, find_file_format_in_content_type};
use crate::request::request::Request;
use crate::request::response::{ImageResponse, RequestResponse, ResponseContent};
use crate::utils::syntax_highlighting::highlight;

impl App<'_> {
    pub async fn send_request(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write();

            // Avoid creating more than one thread
            if selected_request.is_pending {
                return;
            }

            let mut client_builder = ClientBuilder::new()
                .danger_accept_invalid_certs(selected_request.settings.accept_invalid_certs)
                .default_headers(HeaderMap::new())
                .referer(false);

            /* REDIRECTS */

            if !selected_request.settings.allow_redirects {
                client_builder = client_builder.redirect(Policy::none());
            }

            /* STORE COOKIES */

            let should_store_cookies = selected_request.settings.store_received_cookies;

            client_builder = client_builder.cookie_store(should_store_cookies);

            /* PROXY */

            if selected_request.settings.use_config_proxy {
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
            let mut local_console_output = self.script_console.console_output.write();
            let mut local_highlighted_console_output = self.syntax_highlighting.highlighted_console_output.write();

            // Resets the data
            *local_console_output = None;
            *local_highlighted_console_output = vec![];

            let modified_request: Request = match &selected_request.scripts.pre_request_script {
                None => {
                    selected_request.clone()
                },
                Some(pre_request_script) => {
                    let local_env = self.get_selected_env_as_local();

                    let env_values = match &local_env {
                        None => None,
                        Some(local_env) => {
                            let env = local_env.read();
                            Some(env.values.clone())
                        }
                    };

                    let (result_request, env_variables, console_output) = execute_pre_request_script(pre_request_script, &*selected_request, env_values);

                    match &local_env {
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

                    let mut highlighted_console_output = highlight(&console_output, "json").unwrap();

                    highlighted_console_output.insert(0, Line::default());
                    highlighted_console_output.insert(1, Line::raw("----- Pre-request script start -----").dark_gray().centered());
                    highlighted_console_output.push(Line::raw("----- Pre-request script end -----").dark_gray().centered());

                    *local_highlighted_console_output = highlighted_console_output;

                    *local_console_output = Some(console_output);

                    match result_request {
                        None => {
                            selected_request.response.status_code = Some(String::from("(CONSOLE) PRE-SCRIPT ERROR"));
                            return;
                        }
                        Some(request) => request
                    }
                }
            };

            // Drops the write mutex
            drop(local_console_output);
            drop(local_highlighted_console_output);

            /* CLIENT */

            let client = client_builder.build().expect("Could not build HTTP client");

            /* PARAMS */

            let params = self.key_value_vec_to_tuple_vec(&modified_request.params);

            /* URL */

            let url = self.replace_env_keys_by_value(&modified_request.url);

            let url = match Url::parse_with_params(&url, params) {
                Ok(url) => url,
                Err(_) => {
                    selected_request.response.status_code = Some(String::from("INVALID URL"));
                    return;
                }
            };

            /* REQUEST */

            let mut request = client.request(
                modified_request.method.to_reqwest(),
                url
            );

            /* CORS */

            if self.config.is_cors_disabled() {
                request = request.fetch_mode_no_cors();
            }

            /* AUTH */

            match &modified_request.auth {
                NoAuth => {}
                BasicAuth(username, password) => {
                    let username = self.replace_env_keys_by_value(username);
                    let password = self.replace_env_keys_by_value(password);

                    request = request.basic_auth(username, Some(password));
                }
                BearerToken(bearer_token) => {
                    let bearer_token = self.replace_env_keys_by_value(bearer_token);

                    request = request.bearer_auth(bearer_token);
                }
            }

            /* BODY */

            match &modified_request.body {
                ContentType::NoBody => {},
                ContentType::Multipart(form_data) => {
                    let mut multipart = Form::new();

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
                                    selected_request.response.status_code = Some(String::from("COULD NOT OPEN FILE"));
                                    return;
                                }
                            }
                        }
                        else {
                            multipart = multipart.text(key, value);
                        }
                    }

                    request = request.multipart(multipart);
                },
                ContentType::Form(form_data) => {
                    let form = self.key_value_vec_to_tuple_vec(form_data);

                    request = request.form(&form);
                },
                ContentType::File(file_path) => {
                    let file_path_with_env_values = self.replace_env_keys_by_value(file_path);
                    let path = PathBuf::from(file_path_with_env_values);

                    match tokio::fs::File::open(path).await {
                        Ok(file) => {
                            request = request.body(file);
                        }
                        Err(_) => {
                            selected_request.response.status_code = Some(String::from("COULD NOT OPEN FILE"));
                            return;
                        }
                    }
                },
                ContentType::Raw(body) | ContentType::Json(body) | ContentType::Xml(body) | ContentType::Html(body) | ContentType::Javascript(body) => {
                    let body_with_envs = self.replace_env_keys_by_value(body);
                    request = request.body(body_with_envs);
                }
            };

            /* HEADERS */

            for header in &modified_request.headers {
                if !header.enabled {
                    continue;
                }

                let header_name = self.replace_env_keys_by_value(&header.data.0);
                let header_value = self.replace_env_keys_by_value(&header.data.1);

                request = request.header(header_name, header_value);
            }

            let local_selected_request = self.get_selected_request_as_local();
            let local_env = self.get_selected_env_as_local();
            let local_console_output = Arc::clone(&self.script_console.console_output);
            let local_highlighted_body = Arc::clone(&self.syntax_highlighting.highlighted_body);
            let local_highlighted_console_output = Arc::clone(&self.syntax_highlighting.highlighted_console_output);

            /* SEND REQUEST */

            task::spawn(async move {
                local_selected_request.write().is_pending = true;

                let request_start = Instant::now();
                let elapsed_time: Duration;

                let mut response = match request.send().await {
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
                                let mut result_body = response.text().await.unwrap();

                                // If a file format has been found in the content-type header
                                if let Some(file_format) = find_file_format_in_content_type(&headers) {
                                    // If the request response content can be pretty printed
                                    if local_selected_request.read().settings.pretty_print_response_content {

                                        // Match the file format
                                        match file_format.as_str() {
                                            "json" => {
                                                result_body = jsonxf::pretty_print(&result_body).unwrap_or(result_body);
                                            },
                                            _ => {}
                                        }
                                    }

                                    let highlighted_result_body = highlight(&result_body, &file_format);
                                    *local_highlighted_body.write() = highlighted_result_body;
                                } else {
                                    *local_highlighted_body.write() = None;
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
                };

                response.duration = Some(format!("{:?}", elapsed_time));

                /* POST-REQUEST SCRIPT */

                let mut selected_request = local_selected_request.write();
                let mut console_output = local_console_output.write();

                let modified_response: RequestResponse = match &selected_request.scripts.post_request_script {
                    None => {
                        response
                    },
                    Some(post_request_script) => {
                        let env_values = match &local_env {
                            None => None,
                            Some(local_env) => {
                                let env = local_env.read();
                                Some(env.values.clone())
                            }
                        };

                        let (result_response, env_variables, result_console_output) = execute_post_request_script(post_request_script, &response, env_values);

                        match &local_env {
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

                        let mut highlighted_console_output = highlight(&result_console_output, "json").unwrap();

                        highlighted_console_output.insert(0, Line::default());
                        highlighted_console_output.insert(1, Line::raw("----- Post-request script start -----").dark_gray().centered());
                        highlighted_console_output.push(Line::raw("----- Post-request script end -----").dark_gray().centered());

                        let mut local_highlighted_console_output = local_highlighted_console_output.write();

                        local_highlighted_console_output.extend(highlighted_console_output);

                        *console_output = match console_output.as_ref() {
                            None => Some(result_console_output),
                            Some(console_output) => Some(format!("{console_output}\n{result_console_output}"))
                        };

                        match result_response {
                            None => {
                                response.status_code = Some(String::from("(CONSOLE) POST-SCRIPT ERROR"));
                                response
                            }
                            Some(result_response) => result_response
                        }
                    }
                };

                selected_request.response = modified_response;
                selected_request.is_pending = false;
            });
        }
    }
}

pub fn get_file_content_with_name(path: PathBuf) -> std::io::Result<(Vec<u8>, String)> {
    let mut buffer: Vec<u8> = vec![];
    let mut file = File::open(path.clone())?;

    file.read_to_end(&mut buffer)?;

    let file_name = path.file_name().unwrap().to_str().unwrap();

    return Ok((buffer, file_name.to_string()));
}
