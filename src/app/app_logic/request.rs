use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use regex::Regex;
use reqwest::{ClientBuilder, Proxy, Url};
use reqwest::header::CONTENT_TYPE;
use tokio::task;
use tui_textarea::TextArea;
use crate::app::app::App;
use crate::request::auth::{next_auth};
use crate::request::auth::Auth::*;
use crate::request::body::{ContentType, next_content_type};
use crate::request::method::next_method;
use crate::request::request::{KeyValue, Request};

impl App<'_> {
    pub fn get_selected_request_as_local(&self) -> Arc<RwLock<Request>> {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        self.collections[selected_request_index.0].requests[selected_request_index.1].clone()
    }

    pub fn get_request_as_local_from_indexes(&self, selected_request_index: &(usize, usize)) -> Arc<RwLock<Request>> {
        self.collections[selected_request_index.0].requests[selected_request_index.1].clone()
    }

    /* URL */
    pub fn modify_request_url(&mut self) {
        let input_text = self.url_text_input.text.clone();

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            let url_parts = input_text.split_once("?");

            let final_url: String;
            let query_params: &str;

            if let Some((url, found_query_params)) = url_parts {
                final_url = url.to_string();
                query_params = found_query_params;
            } else {
                final_url = input_text;
                query_params = "";
            }


            let mut new_params_to_add: Vec<KeyValue> = vec![];
            let mut existing_params_found_indexes: Vec<usize> = vec![];

            let query_params_pattern = Regex::new(r"(&?([^=]+)=([^&]+))").unwrap();

            for (_, [_, param_name, value]) in query_params_pattern.captures_iter(query_params).map(|c| c.extract()) {
                let mut url_param_found = false;

                for (index, existing_param) in selected_request.params.iter_mut().enumerate() {
                    if param_name == existing_param.data.0 && existing_param.enabled {
                        existing_param.data.1 = value.to_string();
                        url_param_found = true;
                        existing_params_found_indexes.push(index);
                    }
                }

                if !url_param_found {
                    let new_param = KeyValue {
                        enabled: true,
                        data: (param_name.to_string(), value.to_string()),
                    };

                    new_params_to_add.push(new_param);
                }
            }

            let param_indexes = selected_request.params.len();

            for param_index in 0..param_indexes {
                if !existing_params_found_indexes.contains(&param_index) {
                    selected_request.params.remove(param_index);
                }
            }

            for new_param in new_params_to_add {
                selected_request.params.push(new_param);
            }

            selected_request.url = final_url;
        }

        // In case new params were inputted or deleted
        self.update_query_params_selection();

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    /* METHOD */

    pub fn modify_request_method(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            let next_method = next_method(&selected_request.method);
            selected_request.method = next_method;
        }

        self.save_collection_to_file(selected_request_index.0);
    }

    /* PARAMS */

    /// Reset selection of if params are provided, either set it to none
    pub fn update_query_params_selection(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read().unwrap();

        match !selected_request.params.is_empty() {
            true => {
                self.query_params_table.selection = Some((0, 0));
                self.query_params_table.left_state.select(Some(0));
                self.query_params_table.right_state.select(Some(0));
            },
            false => {
                self.query_params_table.selection = None;
                self.query_params_table.left_state.select(None);
                self.query_params_table.right_state.select(None);
            }
        }
    }

    pub fn toggle_query_param(&mut self) {
        if self.query_params_table.rows.is_empty() {
            return;
        }

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            let row = self.query_params_table.selection.unwrap().0;
            selected_request.params[row].enabled = !selected_request.params[row].enabled;
        }

        self.save_collection_to_file(selected_request_index.0);
        self.update_inputs();
    }

    pub fn modify_request_query_param(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            let selection = self.query_params_table.selection.unwrap();
            let input_text = &self.query_params_table.selection_text_input.text;

            match selection {
                (_, 0) => selected_request.params[selection.0].data.0 = input_text.clone(),
                (_, 1) => selected_request.params[selection.0].data.1 = input_text.clone(),
                (_, _) => {}
            };
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }


    pub fn create_new_query_param(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write().unwrap();

            selected_request.params.push(KeyValue {
                enabled: true,
                data: (String::from("param"), String::from("value"))
            });
        }

        self.update_query_params_selection();
        self.update_inputs();
    }

    pub fn delete_query_param(&mut self) {
        if self.query_params_table.selection.is_none() {
            return;
        }

        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write().unwrap();

            let selection = self.query_params_table.selection.unwrap();
            selected_request.params.remove(selection.0);
        }

        self.update_query_params_selection();
        self.update_inputs();
    }

    /* AUTH */

    pub fn modify_request_auth(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            selected_request.auth = next_auth(&selected_request.auth);
        }

        self.save_collection_to_file(selected_request_index.0);
        self.load_request_auth_param_tab();
    }

    pub fn select_request_auth_input_text(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read().unwrap();

        match selected_request.auth {
            NoAuth => {}
            BasicAuth(_, _) => match self.auth_text_input_selection.selected {
                0 => self.edit_request_auth_username_state(),
                1 => self.edit_request_auth_password_state(),
                _ => {}
            },
            BearerToken(_) => match self.auth_text_input_selection.selected {
                0 => self.edit_request_auth_bearer_token_state(),
                _ => {}
            }
        }
    }

    pub fn modify_request_auth_basic_username(&mut self) {
        let input_text = self.auth_basic_username_text_input.text.clone();

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            match &selected_request.auth {
                BasicAuth(_, password) => {
                    selected_request.auth = BasicAuth(input_text, password.to_string());
                }
                _ => {}
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    pub fn modify_request_auth_basic_password(&mut self) {
        let input_text = self.auth_basic_password_text_input.text.clone();

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            match &selected_request.auth {
                BasicAuth(username, _) => {
                    selected_request.auth = BasicAuth(username.to_string(), input_text);
                }
                _ => {}
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    pub fn modify_request_auth_bearer_token(&mut self) {
        let input_text = self.auth_bearer_token_text_input.text.clone();

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            match &selected_request.auth {
                BearerToken(_) => {
                    selected_request.auth = BearerToken(input_text);
                }
                _ => {}
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    /* HEADERS */

    /// Reset selection of if headers are provided, either set it to none
    pub fn update_headers_selection(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read().unwrap();

        match !selected_request.headers.is_empty() {
            true => {
                self.headers_table.selection = Some((0, 0));
                self.headers_table.left_state.select(Some(0));
                self.headers_table.right_state.select(Some(0));
            },
            false => {
                self.headers_table.selection = None;
                self.headers_table.left_state.select(None);
                self.headers_table.right_state.select(None);
            }
        }
    }

    pub fn toggle_header(&mut self) {
        if self.headers_table.rows.is_empty() {
            return;
        }

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            let row = self.headers_table.selection.unwrap().0;
            selected_request.headers[row].enabled = !selected_request.headers[row].enabled;
        }

        self.save_collection_to_file(selected_request_index.0);
        self.update_inputs();
    }

    pub fn modify_request_header(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            let selection = self.headers_table.selection.unwrap();
            let input_text = &self.headers_table.selection_text_input.text;

            match selection {
                (_, 0) => selected_request.headers[selection.0].data.0 = input_text.clone(),
                (_, 1) => selected_request.headers[selection.0].data.1 = input_text.clone(),
                (_, _) => {}
            };
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    pub fn create_new_header(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write().unwrap();

            selected_request.headers.push(KeyValue {
                enabled: true,
                data: (String::from("header"), String::from("value"))
            });
        }

        self.update_headers_selection();
        self.update_inputs();
    }

    pub fn delete_header(&mut self) {
        if self.headers_table.selection.is_none() {
            return;
        }

        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write().unwrap();

            let selection = self.headers_table.selection.unwrap();
            selected_request.headers.remove(selection.0);
        }

        self.update_headers_selection();
        self.update_inputs();
    }

    /* BODY */

    pub fn refresh_body_textarea(&mut self, text: String) {
        let lines: Vec<String> = text
            .lines()
            .map(|line| line.to_string())
            .collect();

        self.body_text_area = TextArea::new(lines);
    }

    pub fn modify_request_body(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            let body = self.body_text_area.lines().join("\n");

            let new_body = match selected_request.body {
                ContentType::NoBody => ContentType::NoBody,
                ContentType::Raw(_) => ContentType::Raw(body.clone()),
                ContentType::Json(_) => ContentType::Json(body.clone()),
                ContentType::Xml(_) => ContentType::Xml(body.clone()),
                ContentType::Html(_) => ContentType::Html(body.clone())
            };

            selected_request.body = new_body;
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    pub fn modify_request_content_type(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write().unwrap();

            selected_request.body = next_content_type(&selected_request.body);

            match &selected_request.body {
                // Removes Content-Type header if there is no more body
                ContentType::NoBody => {
                    selected_request.find_and_delete_header(CONTENT_TYPE.as_str())
                }
                // Create or replace Content-Type header with new body content type
                ContentType::Raw(_) | ContentType::Json(_) | ContentType::Xml(_) | ContentType::Html(_) => {
                    let content_type = &selected_request.body.to_content_type();
                    selected_request.modify_or_create_header(CONTENT_TYPE.as_str(), content_type)
                }
            }
        }

        self.load_request_body_param_tab();
    }

    pub fn quit_request_body(&mut self) {
        self.update_inputs();
        self.select_request_state();
    }

    /* REQUEST */

    pub async fn send_request(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let mut selected_request = local_selected_request.write().unwrap();

            // Avoid creating more than one thread
            if selected_request.is_pending {
                return;
            }

            let mut client_builder = ClientBuilder::new();

            /* PROXY */

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

            /* CLIENT */

            let client = client_builder.build().expect("Could not build HTTP client");

            /* PARAMS */

            let params: Vec<(String, String)> = selected_request.params
                .iter()
                .filter_map(|param| {
                    if param.enabled {
                        let key = self.replace_env_keys_by_value(&param.data.0);
                        let value = self.replace_env_keys_by_value(&param.data.1);

                        Some((key, value))
                    } else {
                        None
                    }
                })
                .collect();

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

                        let headers = response.headers().clone()
                            .iter()
                            .map(|(header_name, header_value)| {
                                format!("{}: {:?}", header_name.to_string(), header_value)
                            })
                            .collect::<Vec<String>>()
                            .join("\n");

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
                        local_selected_request.write().unwrap().result.headers = Some(headers);
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
                        local_selected_request.write().unwrap().result.headers = None;
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