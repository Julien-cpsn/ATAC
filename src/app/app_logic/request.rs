use regex::Regex;
use reqwest::{Client, Url};
use reqwest::header::CONTENT_TYPE;
use tui_textarea::TextArea;
use crate::app::app::App;
use crate::request::auth::{next_auth};
use crate::request::auth::Auth::*;
use crate::request::body::{ContentType, next_content_type};
use crate::request::method::next_method;
use crate::utils::stateful_custom_table::Param;

impl App<'_> {
    /* URL */
    pub fn modify_request_url(&mut self) {
        let input_text = self.url_text_input.text.clone();
        let selected_request_index = self.collection.selected.unwrap();

        let url_parts = input_text.split_once("?");

        let final_url: String;
        let query_params: &str;

        if let Some((url, found_query_params)) = url_parts {
            final_url = url.to_string();
            query_params = found_query_params;
        }
        else {
            final_url = input_text;
            query_params = "";
        }


        let mut new_params_to_add: Vec<Param> = vec![];
        let mut existing_params_found_indexes: Vec<usize> = vec![];

        let query_params_pattern = Regex::new(r"(&?([^=]+)=([^&]+))").unwrap();

        for (_, [_, param_name, value]) in query_params_pattern.captures_iter(query_params).map(|c| c.extract()) {
            let mut url_param_found = false;

            for (index, existing_param) in self.collection.items[selected_request_index].params.iter_mut().enumerate() {
                if param_name == existing_param.data.0 && existing_param.enabled {
                    existing_param.data.1 = value.to_string();
                    url_param_found = true;
                    existing_params_found_indexes.push(index);
                }
            }

            if !url_param_found {
                let new_param = Param {
                    enabled: true,
                    data: (param_name.to_string(), value.to_string()),
                };

                new_params_to_add.push(new_param);
            }
        }

        let param_indexes = self.collection.items[selected_request_index].params.len();

        for param_index in 0..param_indexes {
            if !existing_params_found_indexes.contains(&param_index) {
                self.collection.items[selected_request_index].params.remove(param_index);
            }
        }

        for new_param in new_params_to_add {
            self.collection.items[selected_request_index].params.push(new_param);
        }

        // In case new params were inputted or deleted
        self.update_params_selection();

        self.collection.items[selected_request_index].url = final_url;

        self.save_collection_to_file();
        self.select_request_state();
    }

    /* METHOD */

    pub fn modify_request_method(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let next_method = next_method(&self.collection.items[selected_request_index].method);

        self.collection.items[selected_request_index].method = next_method;

        self.save_collection_to_file();
    }

    /* PARAMS */
    /// Reset selection of if params are provided, either set it to none
    pub fn update_params_selection(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        match !selected_request.params.is_empty() {
            true => {
                self.request_param_table.selection = Some((0, 0));
                self.request_param_table.left_state.select(Some(0));
                self.request_param_table.right_state.select(Some(0));
            },
            false => {
                self.request_param_table.selection = None;
                self.request_param_table.left_state.select(None);
                self.request_param_table.right_state.select(None);
            }
        }
    }

    pub fn toggle_params_table_row(&mut self) {
        if self.request_param_table.rows.is_empty() {
            return;
        }

        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &mut self.collection.items[selected_request_index];

        let row = self.request_param_table.selection.unwrap().0;

        selected_request.params[row].enabled = !selected_request.params[row].enabled;

        self.save_collection_to_file();
        self.update_inputs();
    }

    pub fn modify_request_param(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();

        let selection = self.request_param_table.selection.unwrap();
        let input_text = &self.request_param_table.param_selection_text_input.text;

        match selection {
            (_, 0) => self.collection.items[selected_request_index].params[selection.0].data.0 = input_text.clone(),
            (_, 1) =>self.collection.items[selected_request_index].params[selection.0].data.1 = input_text.clone(),
            (_, _) => {}
        };

        self.save_collection_to_file();
        self.select_request_state();
    }

    /* AUTH */

    pub fn modify_request_auth(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        self.collection.items[selected_request_index].auth = next_auth(&selected_request.auth);

        self.save_collection_to_file();
        self.load_request_auth_param_tab();
    }

    pub fn select_request_auth_input_text(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

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

        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        match &selected_request.auth {
            BasicAuth(_, password) => {
                self.collection.items[selected_request_index].auth = BasicAuth(input_text, password.to_string());
            }
            _ => {}
        }

        self.save_collection_to_file();
        self.select_request_state();
    }

    pub fn modify_request_auth_basic_password(&mut self) {
        let input_text = self.auth_basic_password_text_input.text.clone();

        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        match &selected_request.auth {
            BasicAuth(username, _) => {
                self.collection.items[selected_request_index].auth = BasicAuth(username.to_string(), input_text);
            }
            _ => {}
        }

        self.save_collection_to_file();
        self.select_request_state();
    }

    pub fn modify_request_auth_bearer_token(&mut self) {
        let input_text = self.auth_bearer_token_text_input.text.clone();

        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        match &selected_request.auth {
            BearerToken(_) => {
                self.collection.items[selected_request_index].auth = BearerToken(input_text);
            }
            _ => {}
        }

        self.save_collection_to_file();
        self.select_request_state();
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
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        let body = self.body_text_area.lines().join("\n");

        let new_body = match selected_request.body {
            ContentType::NoBody => ContentType::NoBody,
            ContentType::Raw(_) => ContentType::Raw(body.clone()),
            ContentType::JSON(_) => ContentType::JSON(body.clone()),
            ContentType::XML(_) => ContentType::XML(body.clone()),
            ContentType::HTML(_) => ContentType::HTML(body.clone())
        };

        self.collection.items[selected_request_index].body = new_body;

        self.save_collection_to_file();
        self.select_request_state();
    }

    pub fn modify_request_content_type(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        self.collection.items[selected_request_index].body = next_content_type(&selected_request.body);

        self.load_request_body_param_tab();
    }

    pub fn quit_request_body(&mut self) {
        self.update_inputs();
        self.select_request_state();
    }

    /* REQUEST */

    pub async fn send_request(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &mut self.collection.items[selected_request_index];

        let params: Vec<(String, String)> = selected_request.params
            .iter()
            .filter_map(|param| {
                if param.enabled {
                    Some(param.data.clone())
                }
                else {
                    None
                }
            })
            .collect();

        let client = Client::new();

        let url = Url::parse_with_params(&selected_request.url, params).unwrap();

        let mut request = client.request(
            selected_request.method.as_reqwest(),
            url
        );

        match &selected_request.auth {
            NoAuth => {}
            BasicAuth(username, password) => {
                request = request.basic_auth(username, Some(password));
            }
            BearerToken(bearer_token) => {
                request = request.bearer_auth(bearer_token);
            }
        }

        match &selected_request.body {
            ContentType::NoBody => {},
            ContentType::Raw(body) | ContentType::JSON(body) | ContentType::XML(body) | ContentType::HTML(body) => {
                request = request
                    .header(CONTENT_TYPE, selected_request.body.to_content_type())
                    .body(body.to_string());
            }
        };

        match request.send().await {
            Ok(response) => {
                let status_code = response.status().as_u16();

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

                selected_request.result.status_code = Some(status_code);
                selected_request.result.body = Some(result_body);
                selected_request.result.cookies = Some(cookies);
                selected_request.result.headers = Some(headers);
            },
            Err(error) => {
                let response_status_code;

                if let Some(status_code) = error.status() {
                    response_status_code = Some(status_code.as_u16());
                }
                else {
                    response_status_code = None;
                }
                let result_body = error.to_string();

                selected_request.result.status_code = response_status_code;
                selected_request.result.body = Some(result_body);
                selected_request.result.cookies = None;
                selected_request.result.headers = None;
            }
        };

        self.refresh_result_scrollbar();
    }
}