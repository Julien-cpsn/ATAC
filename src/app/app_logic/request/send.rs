use std::time::{Duration, Instant};
use reqwest::multipart::Form;
use reqwest::{ClientBuilder, Proxy, Url};
use reqwest::redirect::Policy;
use tokio::task;
use crate::app::app::App;
use crate::request::auth::Auth::{NoAuth, BasicAuth, BearerToken};
use crate::request::body::ContentType;

impl App<'_> {
    pub async fn send_request(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write().unwrap();

            // Avoid creating more than one thread
            if selected_request.is_pending {
                return;
            }

            let mut client_builder = ClientBuilder::new();

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
                                let proxy = Proxy::http(http_proxy_str).expect("Could not parse HTTP proxy");
                                client_builder = client_builder.proxy(proxy);
                            }
                        }

                        match &proxy.https_proxy {
                            None => {}
                            Some(https_proxy_str) => {
                                let proxy = Proxy::https(https_proxy_str).expect("Could not parse HTTPS proxy");
                                client_builder = client_builder.proxy(proxy);
                            }
                        }
                    }
                }
            }

            /* CLIENT */

            let client = client_builder.build().expect("Could not build HTTP client");

            /* PARAMS */

            let params = self.key_value_vec_to_tuple_vec(&selected_request.params);

            /* URL */

            let url = self.replace_env_keys_by_value(&selected_request.url);

            let url = match Url::parse_with_params(&url, params) {
                Ok(url) => url,
                Err(_) => {
                    selected_request.result.status_code = Some(String::from("INVALID URL"));
                    return;
                }
            };

            /* REQUEST */

            let mut request = client.request(
                selected_request.method.to_reqwest(),
                url
            );

            /* AUTH */

            match &selected_request.auth {
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

            match &selected_request.body {
                ContentType::NoBody => {},
                ContentType::Multipart(form_data) => {
                    let mut multipart = Form::new();

                    for form_data in form_data {
                        let key = self.replace_env_keys_by_value(&form_data.data.0);
                        let value = self.replace_env_keys_by_value(&form_data.data.1);

                        multipart = multipart.text(key, value);
                    }

                    request = request.multipart(multipart);
                },
                ContentType::Form(form_data) => {
                    let form = self.key_value_vec_to_tuple_vec(form_data);

                    request = request.form(&form);
                },
                ContentType::Raw(body) | ContentType::Json(body) | ContentType::Xml(body) | ContentType::Html(body) => {
                    request = request.body(body.to_string());
                }
            };

            /* HEADERS */

            for header in &selected_request.headers {
                if !header.enabled {
                    continue;
                }

                let header_name = self.replace_env_keys_by_value(&header.data.0);
                let header_value = self.replace_env_keys_by_value(&header.data.1);

                request = request.header(header_name, header_value);
            }

            let local_selected_request = self.get_selected_request_as_local();

            /* SEND REQUEST */

            task::spawn(async move {
                local_selected_request.write().unwrap().is_pending = true;

                let request_start = Instant::now();
                let elapsed_time: Duration;

                match request.send().await {
                    Ok(response) => {
                        let status_code = response.status().to_string();

                        let headers: Vec<(String, String)> = response.headers().clone()
                            .iter()
                            .map(|(header_name, header_value)| {
                                let value = header_value.to_str().unwrap_or("").to_string();

                                (header_name.to_string(), value)
                            })
                            .collect();

                        let cookies = response.cookies()
                            .map(|cookie| {
                                format!("{}: {}", cookie.name(), cookie.value())
                            })
                            .collect::<Vec<String>>()
                            .join("\n");

                        let result_body = response.text().await.unwrap();

                        local_selected_request.write().unwrap().result.status_code = Some(status_code);
                        local_selected_request.write().unwrap().result.body = Some(result_body);
                        local_selected_request.write().unwrap().result.cookies = Some(cookies);
                        local_selected_request.write().unwrap().result.headers = headers;
                    },
                    Err(error) => {
                        let response_status_code;

                        if let Some(status_code) = error.status() {
                            response_status_code = Some(status_code.to_string());
                        } else {
                            response_status_code = None;
                        }
                        let result_body = error.to_string();


                        local_selected_request.write().unwrap().result.status_code = response_status_code;
                        local_selected_request.write().unwrap().result.body = Some(result_body);
                        local_selected_request.write().unwrap().result.cookies = None;
                        local_selected_request.write().unwrap().result.headers = vec![];
                    }
                };

                elapsed_time = request_start.elapsed();
                local_selected_request.write().unwrap().result.duration = Some(format!("{:?}", elapsed_time));

                local_selected_request.write().unwrap().is_pending = false;
            });
        }

        self.refresh_result_scrollbar();
    }
}