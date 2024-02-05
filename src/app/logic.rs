use reqwest::{Client, Method};
use tui_textarea::{TextArea};
use crate::app::app::{App, AppState};
use crate::app::tabs::tabs::RequestTabs;
use crate::request::method::next_method;
use crate::request::request::Request;

impl<'a> App<'a> {
    pub fn select_request(&mut self) {
        self.url_text_input.reset_input();
        self.collection.select();
        self.result_scrollbar.set_scroll(0);

        if let Some(selected_request_index) = self.collection.selected {
            let selected_request = &self.collection.items[selected_request_index];
            self.url_text_input.enter_str(selected_request.url);

            let body = selected_request.body.clone().unwrap_or(String::new());
            self.refresh_body_textarea(body);
        }
    }

    pub fn unselect_request(&mut self) {
        self.url_text_input.reset_input();
        self.collection.unselect();
    }

    pub fn new_request(&mut self) {
        let new_request_name = &self.new_request_input.text;

        if new_request_name.len() == 0 {
            return;
        }

        let new_request = Request::<'a> {
            name: new_request_name.clone().leak(),
            url: "",
            method: Method::GET,
            body: None,
            result: None,
        };

        self.collection.items.push(new_request);

        self.state = AppState::Normal;
    }

    pub fn delete_request(&mut self) {
        if let Some(selected_request_index) = self.collection.state.selected() {
            self.collection.unselect();
            self.collection.items.remove(selected_request_index);
        }
    }

    pub fn modify_request_url(&mut self) {
        self.state = AppState::Normal;

        let input_text = self.url_text_input.text.clone();
        let selected_request_index = self.collection.selected.unwrap();

        self.collection.items[selected_request_index].url = input_text.leak();
    }

    pub fn modify_request_method(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let next_method = next_method(&self.collection.items[selected_request_index].method);
        self.collection.items[selected_request_index].method = next_method;
    }

    pub fn load_request_body_tab(&mut self) {
        self.request_tab = RequestTabs::Body;

        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        if selected_request.body.is_some() {
            self.state = AppState::EditingBody;
        }
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

        self.state = AppState::Normal;
        self.refresh_body_textarea(body);
    }

    pub fn toggle_request_body(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        let body = String::new();

        match selected_request.body {
            None => {
                self.collection.items[selected_request_index].body = Some(body.clone());
                self.state = AppState::EditingBody;
            }
            Some(_) => {
                self.collection.items[selected_request_index].body = None;
                self.state = AppState::Normal;
            }
        }

        self.refresh_body_textarea(body);
    }

    pub fn quit_request_body(&mut self) {
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &mut self.collection.items[selected_request_index];

        let body = selected_request.body.clone().unwrap_or(String::new());

        self.refresh_body_textarea(body);
        self.state = AppState::Normal;
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


        let result = match request.send().await {
            Ok(result) => result.text().await.unwrap(),
            Err(error) => error.to_string()
        };


        let lines_count = result.lines().count();

        self.result_scrollbar.set_scroll(lines_count);

        selected_request.result = Some(result);
    }
}