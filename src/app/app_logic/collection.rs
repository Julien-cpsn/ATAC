use std::sync::{Arc, RwLock};
use crate::app::app::App;
use crate::app::startup::args::ARGS;
use crate::request::auth::Auth;
use crate::request::body::ContentType;
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
        self.body_form_table.selection_text_input.reset_input();
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

            let param_text = match selection {
                (x, 0) => selected_request.params[x].data.0.clone(),
                (x, 1) => selected_request.params[x].data.1.clone(),
                _ => String::new() // Should not happen
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

            let header_text = match selection {
                (x, 0) => selected_request.headers[x].data.0.clone(),
                (x, 1) => selected_request.headers[x].data.1.clone(),
                _ => String::new() // Should not happen
            };

            self.headers_table.selection_text_input.enter_str(&header_text);
        }

        match &selected_request.body {
            ContentType::NoBody => {
                self.body_form_table.rows = Vec::new();
                self.refresh_body_textarea(&String::new());
            }
            ContentType::Multipart(form) | ContentType::Form(form) => {
                self.body_form_table.rows = form.clone();

                if !form.is_empty() {
                    let selection = self.body_form_table.selection.unwrap();

                    let form_text = match selection {
                        (x, 0) => form[x].data.0.clone(),
                        (x, 1) => form[x].data.1.clone(),
                        _ => String::new() // Should not happen
                    };

                    self.body_form_table.selection_text_input.enter_str(&form_text);
                }

                self.refresh_body_textarea(&String::new());
            }
            ContentType::Raw(body) | ContentType::Json(body) | ContentType::Xml(body) | ContentType::Html(body) => {
                self.body_form_table.rows = Vec::new();
                self.refresh_body_textarea(body);
            }
        }
    }

    pub fn select_request(&mut self) {
        if self.collections_tree.state.selected().len() == 2 {
            self.collections_tree.set_selected();
            self.update_query_params_selection();
            self.update_headers_selection();
            self.update_body_table_selection();
            self.refresh_result_scrollbars();
            
            self.select_request_state();
        }
    }

    pub fn unselect_request(&mut self) {
        self.collections_tree.state.select(Vec::new());
        self.collections_tree.set_unselected();
        self.normal_state()
    }

    pub fn select_request_or_expand_collection(&mut self) {
        match self.collections_tree.state.selected().len() {
            1 => {
                self.collections_tree.state.toggle_selected();
            },
            2 => {
                self.select_request();
            },
            _ => {}
        }
    }
    
    pub fn new_collection(&mut self) {
        let new_collection_name = &self.new_collection_input.text;

        if new_collection_name.trim().is_empty() {
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

        if new_request_name.trim().is_empty() {
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

    pub fn rename_element(&mut self) {
        match self.collections_tree.state.selected().len() {
            // Selection on a collection
            1 => self.rename_collection_state(),
            // Selection on a request
            2 => self.rename_request_state(),
            _ => {}
        }
    }

    pub fn rename_collection(&mut self) {
        let new_collection_name = &self.rename_collection_input.text;

        if new_collection_name.trim().is_empty() {
            return;
        }

        let selected_request_index = self.collections_tree.state.selected();

        self.collections[selected_request_index[0]].name = new_collection_name.to_string();

        self.save_collection_to_file(selected_request_index[0]);
        self.normal_state();
    }

    pub fn rename_request(&mut self) {
        let new_request_name = &self.rename_request_input.text;

        if new_request_name.trim().is_empty() {
            return;
        }

        let selected_request_index = self.collections_tree.state.selected();
        let local_selected_request = self.get_request_as_local_from_indexes(&(selected_request_index[0], selected_request_index[1]));

        {
            let mut selected_request = local_selected_request.write().unwrap();
            
            selected_request.name = new_request_name.to_string();
        }
        
        self.save_collection_to_file(selected_request_index[0]);
        self.normal_state();
    }

    pub fn move_request_up(&mut self) {
        if self.collections_tree.state.selected().len() != 2 {
            return;
        }

        let mut selection = self.collections_tree.state.selected();

        // Cannot decrement selection further
        if selection[1] == 0 {
            return;
        }

        // Retrieve the request
        let request = self.collections[selection[0]].requests.remove(selection[1]);

        // Increment selection
        selection[1] -= 1;

        // Insert the request at its new index
        self.collections[selection[0]].requests.insert(selection[1], request);

        // Update the selection in order to move with the element
        self.collections_tree.state.select(selection.clone());

        self.save_collection_to_file(selection[0]);
    }

    pub fn move_request_down(&mut self) {
        if self.collections_tree.state.selected().len() != 2 {
            return;
        }

        let mut selection = self.collections_tree.state.selected();

        // Cannot increment selection further
        if selection[1] == self.collections[selection[0]].requests.len() - 1 {
            return;
        }

        // Retrieve the request
        let request = self.collections[selection[0]].requests.remove(selection[1]);

        // Increment selection
        selection[1] += 1;

        // Insert the request at its new index
        self.collections[selection[0]].requests.insert(selection[1], request);

        // Update the selection in order to move with the element
        self.collections_tree.state.select(selection.clone());

        self.save_collection_to_file(selection[0]);
    }
}