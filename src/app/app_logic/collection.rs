use std::sync::{Arc, RwLock};
use crate::app::app::App;
use crate::app::startup::args::ARGS;
use crate::request::auth::Auth;
use crate::request::collection::Collection;
use crate::request::request::{DEFAULT_HEADERS, Request};
use crate::request::settings::RequestSettings;

impl App<'_> {
    pub fn reset_inputs(&mut self) {
        self.url_text_input.reset_input();
        self.query_params_table.selection_text_input.reset_input();
        self.auth_basic_username_text_input.reset_input();
        self.auth_basic_password_text_input.reset_input();
        self.auth_bearer_token_text_input.reset_input();
        self.headers_table.selection_text_input.reset_input();
    }

    pub fn update_inputs(&mut self) {
        self.reset_inputs();

        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read().unwrap();

        self.url_text_input.enter_str(&selected_request.url_with_params_to_string());
        self.query_params_table.rows = selected_request.params.clone();
        self.headers_table.rows = selected_request.headers.clone();

        if !selected_request.params.is_empty() {
            let selection = self.query_params_table.selection.unwrap();

            let param_text = match selection.1 {
                0 => selected_request.params[selection.0].data.0.clone(),
                1 => selected_request.params[selection.0].data.1.clone(),
                _ => String::new()
            };

            self.query_params_table.selection_text_input.enter_str(&param_text);
        }

        match &selected_request.auth {
            Auth::NoAuth => {
                self.auth_text_input_selection.max_selection = 0;
                self.auth_text_input_selection.usable = false;
            }
            Auth::BasicAuth(username, password) => {
                self.auth_text_input_selection.max_selection = 2;
                self.auth_text_input_selection.usable = true;

                self.auth_basic_username_text_input.enter_str(username);
                self.auth_basic_password_text_input.enter_str(password);
            }
            Auth::BearerToken(bearer_token) => {
                self.auth_text_input_selection.max_selection = 1;
                self.auth_text_input_selection.usable = true;

                self.auth_bearer_token_text_input.enter_str(bearer_token);
            }
        }

        if !selected_request.headers.is_empty() {
            let selection = self.headers_table.selection.unwrap();

            let header_text = match selection.1 {
                0 => selected_request.headers[selection.0].data.0.clone(),
                1 => selected_request.headers[selection.0].data.1.clone(),
                _ => String::new()
            };

            self.headers_table.selection_text_input.enter_str(&header_text);
        }

        let body = selected_request.body.get_body_as_string();
        self.refresh_body_textarea(body);
    }

    pub fn select_request(&mut self) {
        if self.collections_tree.state.selected().len() == 2 {
            self.collections_tree.set_selected();
            self.update_query_params_selection();
            self.update_headers_selection();

            self.select_request_state();
        }
    }

    pub fn unselect_request(&mut self) {
        self.collections_tree.state.select(Vec::new());
        self.collections_tree.set_unselected();
        self.normal_state()
    }

    pub fn new_collection(&mut self) {
        let new_collection_name = &self.new_collection_input.text;

        if new_collection_name.is_empty() {
            return;
        }

        // Check that collection names are unique (like files)
        for collection in &self.collections {
            if new_collection_name == &collection.name {
                return;
            }
        }

        let new_collection = Collection {
            name: new_collection_name.clone(),
            requests: vec![],
            path: ARGS.directory.join(format!("{}.json", new_collection_name.clone()))
        };

        self.collections.push(new_collection);

        let collection_index= self.collections.len() - 1;

        self.save_collection_to_file(collection_index);
        self.normal_state();
    }

    pub fn new_request(&mut self) {
        let new_request_name = &self.new_request_popup.text_input.text;

        if new_request_name.is_empty() {
            return;
        }

        let new_request = Request {
            name: new_request_name.clone(),
            headers: DEFAULT_HEADERS.clone(),
            settings: RequestSettings {
                use_config_proxy: true,
                allow_redirects: true,
                store_received_cookies: true,
            },
            ..Default::default()
        };

        let selected_collection = self.new_request_popup.selected_collection;

        self.collections[selected_collection].requests.push(Arc::new(RwLock::new(new_request)));

        self.save_collection_to_file(selected_collection);
        self.normal_state();
    }

    pub fn delete_element(&mut self) {
        match self.collections_tree.state.selected().len() {
            // Selection on a collection
            1 => self.delete_collection_state(),
            // Selection on a request
            2 => self.delete_request_state(),
            _ => {}
        }
    }

    pub fn delete_collection(&mut self) {
        let selected_request_index = self.collections_tree.state.selected();

        let collection = self.collections.remove(selected_request_index[0]);

        self.collections_tree.state.select(Vec::new());
        self.collections_tree.selected = None;

        self.delete_collection_file(collection);
        self.normal_state();
    }

    pub fn delete_request(&mut self) {
        let selected_request_index = self.collections_tree.state.selected();
        self.collections[selected_request_index[0]].requests.remove(selected_request_index[1]);

        self.collections_tree.state.select(Vec::new());
        self.collections_tree.selected = None;

        self.save_collection_to_file(selected_request_index[0]);
        self.normal_state();
    }
}