use crate::app::app::App;
use crate::app::app_states::AppState;
use crate::request::request::{Request};

impl<'a> App<'a> {
    pub fn select_request(&mut self) {
        self.url_text_input.reset_input();
        self.collection.select();
        self.result_scrollbar.set_scroll(0);

        if let Some(selected_request_index) = self.collection.selected {
            let selected_request = &self.collection.items[selected_request_index];
            self.url_text_input.enter_str(selected_request.url);

            let body = selected_request.body.get_body_as_string();

            self.refresh_body_textarea(body);

            self.state = AppState::SelectedRequest;
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
            ..Default::default()
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
}