use reqwest::header::CONTENT_TYPE;
use tracing::{info};

use crate::app::app::App;
use crate::models::protocol::http::body::{ContentType, next_content_type};

impl App<'_> {
    /// Reset selection if body form data is provided, either set it to none
    pub fn tui_update_body_table_selection(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();
        let selected_http_request = selected_request.get_http_request().unwrap();

        {
            if let Ok(form) = selected_http_request.body.get_form() {
                match form.is_empty() {
                    false => self.body_form_table.update_selection(Some((0, 0))),
                    true => self.body_form_table.update_selection(None)
                }
            }
        }
    }

    pub fn tui_modify_request_form_data(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();

        let selection = self.body_form_table.selection.unwrap();
        let input_text = self.body_form_table.selection_text_input.to_string();

        if let Err(_) = self.modify_request_form_data(selected_request_index.0, selected_request_index.1, input_text, selection.1, selection.0) {
            return;
        }

        self.select_request_state();
    }

    pub fn tui_create_new_form_data(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();

        if let Err(_) = self.create_new_form_data(selected_request_index.0, selected_request_index.1, String::from("key"), String::from("value")) {
            return;
        }

        self.tui_update_body_table_selection();
        self.update_inputs();
    }

    pub fn tui_delete_form_data(&mut self) {
        if self.body_form_table.selection.is_none() {
            return;
        }

        let selection = self.body_form_table.selection.unwrap();
        let selected_request_index = &self.collections_tree.selected.unwrap();

        if let Err(_) = self.delete_form_data(selected_request_index.0, selected_request_index.1, selection.0) {
            return;
        }

        self.tui_update_body_table_selection();
        self.update_inputs();
    }

    pub fn tui_toggle_form_data(&mut self) {
        if self.body_form_table.rows.is_empty() {
            return;
        }

        let selection = self.body_form_table.selection.unwrap();
        let selected_request_index = &self.collections_tree.selected.unwrap();

        if let Err(_) = self.toggle_form_data(selected_request_index.0, selected_request_index.1, None, selection.0) {
            return;
        }

        self.update_inputs();
    }

    pub fn tui_duplicate_form_data(&mut self) {
        if self.body_form_table.rows.is_empty() {
            return;
        }

        let selection = self.body_form_table.selection.unwrap();
        let selected_request_index = &self.collections_tree.selected.unwrap();

        if let Err(_) = self.duplicate_form_data(selected_request_index.0, selected_request_index.1, selection.0) {
            return;
        }

        self.update_inputs();
    }

    pub fn tui_modify_request_body(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();
            let selected_http_request = selected_request.get_http_request_mut().unwrap();

            let body_form = &self.body_form_table.rows;
            let body_file = self.body_file_text_input.to_string();
            let body_string = self.body_text_area.to_string();

            let new_body = match selected_http_request.body {
                ContentType::NoBody => ContentType::NoBody,
                ContentType::Multipart(_) => ContentType::Multipart(body_form.clone()),
                ContentType::Form(_) => ContentType::Form(body_form.clone()),
                ContentType::File(_) => ContentType::File(body_file),
                ContentType::Raw(_) => ContentType::Raw(body_string.clone()),
                ContentType::Json(_) => ContentType::Json(body_string.clone()),
                ContentType::Xml(_) => ContentType::Xml(body_string.clone()),
                ContentType::Html(_) => ContentType::Html(body_string.clone()),
                ContentType::Javascript(_) => ContentType::Javascript(body_string.clone()),
            };

            info!("Body set to \"{}\"", new_body);

            selected_http_request.body = new_body;
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    pub fn tui_next_request_content_type(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();
            let selected_http_request = selected_request.get_http_request_mut().unwrap();

            let new_content_type = next_content_type(&selected_http_request.body);

            info!("Body content-type set to \"{}\"", new_content_type);

            selected_http_request.body = new_content_type;

            match &selected_http_request.body {
                // Removes Content-Type header if there is no more body
                ContentType::NoBody => {
                    selected_request.find_and_delete_header(CONTENT_TYPE.as_str())
                },
                // TODO: Impossible to set the header for multipart yet, because of boundary and content-length that are computed on reqwest's side
                ContentType::Multipart(_) => {},
                // Create or replace Content-Type header with new body content type
                ContentType::File(_) | ContentType::Form(_) | ContentType::Raw(_) | ContentType::Json(_) | ContentType::Xml(_) | ContentType::Html(_) | ContentType::Javascript(_) => {
                    let content_type = &selected_http_request.body.to_content_type();
                    selected_request.modify_or_create_header(CONTENT_TYPE.as_str(), content_type)
                }
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.tui_update_body_table_selection();
        self.tui_load_request_body_param_tab();
    }
}