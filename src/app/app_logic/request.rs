use reqwest::Client;
use tui_textarea::TextArea;
use crate::app::app::App;
use crate::request::method::next_method;

impl App<'_> {
    pub fn modify_request_url(&mut self) {
        let input_text = self.url_text_input.text.clone();
        let selected_request_index = self.collection.selected.unwrap();

        self.collection.items[selected_request_index].url = input_text.leak();

        self.select_request_state();
    }

    pub fn modify_request_method(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let next_method = next_method(&self.collection.items[selected_request_index].method);
        self.collection.items[selected_request_index].method = next_method;
    }

    pub fn refresh_body_textarea(&mut self, text: String) {
        let lines: Vec<String> = text
            .lines()
            .map(|line| line.to_string())
            .collect();

        self.body_text_area = TextArea::new(lines);
    }

    pub fn modify_request_body(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();

        let body: String = self.body_text_area.lines().join("\n");

        self.collection.items[selected_request_index].body = Some(body.clone());

        self.refresh_body_textarea(body);
        self.select_request_state();
    }

    pub fn toggle_request_body(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        let body = String::new();

        match selected_request.body {
            None => {
                self.collection.items[selected_request_index].body = Some(body.clone());
                self.edit_request_body_state();
            }
            Some(_) => {
                self.collection.items[selected_request_index].body = None;
                self.select_request_state();
            }
        }

        self.refresh_body_textarea(body);
    }

    pub fn quit_request_body(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &mut self.collection.items[selected_request_index];

        let body = selected_request.body.clone().unwrap_or(String::new());

        self.refresh_body_textarea(body);
        self.select_request_state();
    }

    pub async fn send_request(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &mut self.collection.items[selected_request_index];

        let client = Client::new();

        let mut request = client.request(
            selected_request.method.clone(),
            selected_request.url
        );

        if let Some(body) = selected_request.body.clone() {
            request = request.body(body);
        }

        match request.send().await {
            Ok(response) => {
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

                selected_request.result.body = Some(result_body);
                selected_request.result.cookies = Some(cookies);
                selected_request.result.headers = Some(headers);
            },
            Err(error) => {
                let result_body = error.to_string();

                selected_request.result.body = Some(result_body);
                selected_request.result.cookies = None;
                selected_request.result.headers = None;
            }
        };

        self.refresh_result_scrollbar();
    }
}