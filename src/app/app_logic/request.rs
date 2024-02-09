use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use tui_textarea::TextArea;
use crate::app::app::App;
use crate::request::body::{ContentType, next_content_type};
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
        let selected_request = &self.collection.items[selected_request_index];

        let body: String = self.body_text_area.lines().join("\n");

        let new_body = match selected_request.body {
            ContentType::NoBody => ContentType::NoBody,
            ContentType::Raw(_) => ContentType::Raw(body.clone()),
            ContentType::JSON(_) => ContentType::JSON(body.clone()),
            ContentType::XML(_) => ContentType::XML(body.clone()),
            ContentType::HTML(_) => ContentType::HTML(body.clone())
        };

        self.collection.items[selected_request_index].body = new_body;

        self.refresh_body_textarea(body);
        self.select_request_state();
    }

    pub fn modify_request_content_type(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        let body = selected_request.body.get_body_as_string();

        self.collection.items[selected_request_index].body = next_content_type(&selected_request.body);

        self.refresh_body_textarea(body);
    }

    pub fn quit_request_body(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &mut self.collection.items[selected_request_index];

        let body = selected_request.body.get_body_as_string();

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