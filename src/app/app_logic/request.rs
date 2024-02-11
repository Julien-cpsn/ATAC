use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use tui_textarea::TextArea;
use crate::app::app::App;
use crate::request::auth::{Auth, next_auth};
use crate::request::auth::Auth::BasicAuth;
use crate::request::body::{ContentType, next_content_type};
use crate::request::method::next_method;

impl App<'_> {
    /* URL */
    pub fn modify_request_url(&mut self) {
        let input_text = self.url_text_input.text.clone();
        let selected_request_index = self.collection.selected.unwrap();

        self.collection.items[selected_request_index].url = input_text.leak();

        self.update_inputs();
        self.select_request_state();
    }

    /* METHOD */

    pub fn modify_request_method(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let next_method = next_method(&self.collection.items[selected_request_index].method);

        self.collection.items[selected_request_index].method = next_method;
    }

    /* AUTH */

    pub fn modify_request_auth(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        self.collection.items[selected_request_index].auth = next_auth(&selected_request.auth);

        self.load_request_auth_param_tab();
    }

    pub fn select_request_auth_input_text(&mut self) {
        match self.auth_text_input_selection.selected {
            0 => self.edit_request_auth_username_state(),
            1 => self.edit_request_auth_password_state(),
            _ => {}
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

        self.update_inputs();
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

        self.update_inputs();
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

        self.update_inputs();
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

        let client = Client::new();

        let mut request = client.request(
            selected_request.method.clone(),
            selected_request.url
        );

        match &selected_request.auth {
            Auth::NoAuth => {}
            BasicAuth(username, password) => {
                request = request.basic_auth(username, Some(password));
            }
            Auth::BearerToken(_token) => {}
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