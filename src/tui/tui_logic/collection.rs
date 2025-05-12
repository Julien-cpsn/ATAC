use crate::app::app::App;
use crate::models::auth::Auth;
use crate::models::body::ContentType;
use crate::models::request::{Request, DEFAULT_HEADERS};
use crate::models::settings::RequestSettings;

impl App<'_> {
    pub fn reset_inputs(&mut self) {
        self.url_text_input.reset_input();
        self.query_params_table.selection_text_input.reset_input();
        self.auth_basic_username_text_input.reset_input();
        self.auth_basic_password_text_input.reset_input();
        self.auth_bearer_token_text_input.reset_input();
        self.headers_table.selection_text_input.reset_input();
        self.body_form_table.selection_text_input.reset_input();
        self.body_file_text_input.reset_input();
    }

    pub fn update_inputs(&mut self) {
        self.reset_inputs();

        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

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
            Auth::BasicAuth { username, password } => {
                self.auth_text_input_selection.max_selection = 2;
                self.auth_text_input_selection.usable = true;

                self.auth_basic_username_text_input.enter_str(username);
                self.auth_basic_password_text_input.enter_str(password);
            }
            Auth::BearerToken { token: bearer_token } => {
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
            ContentType::File(file_path) =>  {
                self.body_file_text_input.enter_str(file_path);
            },
            ContentType::Raw(body) | ContentType::Json(body) | ContentType::Xml(body) | ContentType::Html(body) | ContentType::Javascript(body) => {
                self.body_form_table.rows = Vec::new();
                self.refresh_body_textarea(body);
            }
        }
        
        let pre_request_script = match &selected_request.scripts.pre_request_script {
            None => "",
            Some(pre_request_script) => &pre_request_script
        };

        let post_request_script = match &selected_request.scripts.post_request_script {
            None => "",
            Some(pre_request_script) => &pre_request_script
        };
        
        self.tui_refresh_pre_request_script_textarea(pre_request_script);
        self.tui_refresh_post_request_script_textarea(post_request_script);
    }
    
    pub fn reset_cursors(&mut self) {
        self.url_text_input.reset_cursor();
        self.query_params_table.selection_text_input.reset_cursor();
        self.auth_basic_username_text_input.reset_cursor();
        self.auth_basic_password_text_input.reset_cursor();
        self.auth_bearer_token_text_input.reset_cursor();
        self.headers_table.selection_text_input.reset_cursor();
        self.body_form_table.selection_text_input.reset_cursor();
        self.body_file_text_input.reset_cursor();
    }

    pub fn select_request(&mut self) {
        if self.collections_tree.state.selected().len() == 2 {
            self.collections_tree.set_selected();
            self.tui_update_query_params_selection();
            self.tui_update_headers_selection();
            self.tui_update_body_table_selection();
            self.tui_refresh_result_scrollbars();
            
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
    
    pub fn new_element(&mut self) {
        match self.creation_popup.selection {
            0 => self.create_new_collection_state(),
            1 => self.create_new_request_state(),
            _ => {}
        }
    }
    
    pub fn tui_new_collection(&mut self) {
        let new_collection_name = self.new_collection_input.text.clone();

        match self.new_collection(new_collection_name) {
            Ok(_) => {}
            Err(_) => {
                return;
            }
        }
        
        self.normal_state();
    }

    pub fn tui_new_request(&mut self) {
        let new_request_name = self.new_request_popup.text_input.text.trim().to_string();

        let selected_collection_index = self.new_request_popup.selected_collection;
        let new_request = Request {
            name: new_request_name,
            headers: DEFAULT_HEADERS.clone(),
            settings: RequestSettings::default(),
            ..Default::default()
        };
        
        match self.new_request(selected_collection_index, new_request) {
            Ok(_) => {}
            Err(_) => {
                return;
            }
        }

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

    pub fn tui_delete_collection(&mut self) {
        let selected_request_index = self.collections_tree.state.selected().to_vec();
        
        self.collections_tree.state.select(Vec::new());
        self.collections_tree.selected = None;
        
        self.delete_collection(selected_request_index[0]);
        
        self.normal_state();
    }

    pub fn tui_delete_request(&mut self) {
        let selected_request_index = self.collections_tree.state.selected().to_vec();
        let collection_index = selected_request_index[0];
        let request_index = selected_request_index[1];

        self.collections_tree.state.select(Vec::new());
        self.collections_tree.selected = None;

        match self.delete_request(collection_index, request_index) {
            Ok(_) => {}
            Err(_) => return
        }

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

    pub fn tui_rename_collection(&mut self) {
        let new_collection_name = self.rename_collection_input.text.clone();
        let selected_request_index = self.collections_tree.state.selected();

        match self.rename_collection(selected_request_index[0], new_collection_name) {
            Ok(_) => {}
            Err(_) => {
                return;
            }
        }
        
        self.normal_state();
    }

    pub fn tui_rename_request(&mut self) {
        let new_request_name = self.rename_request_input.text.clone();
        let selected_request_index = self.collections_tree.state.selected();

        match self.rename_request(selected_request_index[0], selected_request_index[1], new_request_name) {
            Ok(_) => {}
            Err(_) => return
        }

        self.normal_state();
    }

    pub fn duplicate_element(&mut self) {
        match self.collections_tree.state.selected().len() {
            // Selection on a collection
            1 => {
                let selected_request_index = self.collections_tree.state.selected().to_vec();
                let collection_index = selected_request_index[0];
                
                match self.duplicate_collection(collection_index) {
                    Ok(_) => {}
                    Err(_) => return
                }
            }
            // Selection on a request
            2 => {
                let selected_request_index = self.collections_tree.state.selected().to_vec();

                match self.duplicate_request(selected_request_index[0], selected_request_index[1]) {
                    Ok(_) => {}
                    Err(_) => return
                }
            }
            _ => {}
        }

    }
    pub fn tui_move_element_up(&mut self) {
        match self.collections_tree.state.selected().len() {
            1 => self.tui_move_collection_up(),
            2 => self.tui_move_request_up(),
            _ => {}
        }
    }

    pub fn tui_move_collection_up(&mut self) {
        let mut selection = self.collections_tree.state.selected().to_vec();

        // Cannot decrement selection further
        if selection[0] == 0 {
            return;
        }

        let collection = self.collections.remove(selection[0]);
        
        // Decrement selection
        selection[0] -= 1;

        self.collections.insert(selection[0], collection);

        // Update the selection in order to move with the element
        self.collections_tree.state.select(selection.clone());
        
        self.update_collections_last_position();
    }

    pub fn tui_move_request_up(&mut self) {
        let mut selection = self.collections_tree.state.selected().to_vec();

        // Cannot decrement selection further
        if selection[1] == 0 {
            return;
        }

        // Retrieve the request
        let request = self.collections[selection[0]].requests.remove(selection[1]);

        // Decrement selection
        selection[1] -= 1;

        // Insert the request at its new index
        self.collections[selection[0]].requests.insert(selection[1], request);

        // Update the selection in order to move with the element
        self.collections_tree.state.select(selection.clone());

        self.save_collection_to_file(selection[0]);
    }

    pub fn tui_move_element_down(&mut self) {
        match self.collections_tree.state.selected().len() {
            1 => self.tui_move_collection_down(),
            2 => self.tui_move_request_down(),
            _ => {}
        }
    }

    pub fn tui_move_collection_down(&mut self) {
        let mut selection = self.collections_tree.state.selected().to_vec();

        // Cannot increment selection further
        if selection[0] == self.collections.len() - 1 {
            return;
        }

        let collection = self.collections.remove(selection[0]);

        // Increment selection
        selection[0] += 1;

        self.collections.insert(selection[0], collection);

        // Update the selection in order to move with the element
        self.collections_tree.state.select(selection.clone());

        self.update_collections_last_position();
    }

    pub fn tui_move_request_down(&mut self) {
        let mut selection = self.collections_tree.state.selected().to_vec();

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
