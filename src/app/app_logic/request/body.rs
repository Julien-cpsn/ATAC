use reqwest::header::CONTENT_TYPE;
use tui_textarea::TextArea;
use rayon::prelude::*;

use crate::app::app::App;
use crate::request::body::{ContentType, next_content_type};
use crate::request::request::KeyValue;

impl App<'_> {
    /// Reset selection if body form data is provided, either set it to none
    pub fn update_body_table_selection(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        {
            if let Some(form) = selected_request.body.get_form() {
                match form.is_empty() {
                    false => {
                        self.body_form_table.selection = Some((0, 0));
                        self.body_form_table.left_state.select(Some(0));
                        self.body_form_table.right_state.select(Some(0));
                    },
                    true => {
                        self.body_form_table.selection = None;
                        self.body_form_table.left_state.select(None);
                        self.body_form_table.right_state.select(None);
                    }
                }
            }
        }
    }

    pub fn modify_request_form_data(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            let selection = self.body_form_table.selection.unwrap();
            let input_text = &self.body_form_table.selection_text_input.text;

            if let Some(form) = selected_request.body.get_form_mut() {
                match selection {
                    (x, 0) => form[x].data.0 = input_text.clone(),
                    (x, 1) => form[x].data.1 = input_text.clone(),
                    (_, _) => {}
                };
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    pub fn create_new_form_data(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            if let Some(form) = selected_request.body.get_form_mut() {
                form.push(KeyValue {
                    enabled: true,
                    data: (String::from("key"), String::from("value"))
                });
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.update_body_table_selection();
        self.update_inputs();
    }

    pub fn delete_form_data(&mut self) {
        if self.body_form_table.selection.is_none() {
            return;
        }

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            if let Some(form) = selected_request.body.get_form_mut() {
                let selection = self.body_form_table.selection.unwrap();
                form.remove(selection.0);
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.update_body_table_selection();
        self.update_inputs();
    }

    pub fn toggle_form_data(&mut self) {
        if self.body_form_table.rows.is_empty() {
            return;
        }

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            if let Some(form) = selected_request.body.get_form_mut() {
                let row = self.body_form_table.selection.unwrap().0;
                form[row].enabled = !form[row].enabled;
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.update_inputs();
    }

    pub fn refresh_body_textarea(&mut self, text: &String) {
        let lines: Vec<String> = text
            .par_lines()
            .map(|line| line.to_string())
            .collect();

        self.body_text_area = TextArea::new(lines);
    }

    pub fn modify_request_body(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            let body_form = &self.body_form_table.rows;
            let body_file = &self.body_file_text_input.text;
            let body_string = self.body_text_area.lines().join("\n");

            let new_body = match selected_request.body {
                ContentType::NoBody => ContentType::NoBody,
                ContentType::Multipart(_) => ContentType::Multipart(body_form.clone()),
                ContentType::Form(_) => ContentType::Form(body_form.clone()),
                ContentType::File(_) => ContentType::File(body_file.clone()),
                ContentType::Raw(_) => ContentType::Raw(body_string.clone()),
                ContentType::Json(_) => ContentType::Json(body_string.clone()),
                ContentType::Xml(_) => ContentType::Xml(body_string.clone()),
                ContentType::Html(_) => ContentType::Html(body_string.clone()),
                ContentType::Javascript(_) => ContentType::Javascript(body_string.clone()),
            };

            selected_request.body = new_body;
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    pub fn modify_request_content_type(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            selected_request.body = next_content_type(&selected_request.body);

            match &selected_request.body {
                // Removes Content-Type header if there is no more body
                ContentType::NoBody => {
                    selected_request.find_and_delete_header(CONTENT_TYPE.as_str())
                },
                // TODO: Impossible to set the header for multipart yet, because of boundary and content-length that are computed on reqwest's side
                ContentType::Multipart(_) => {},
                // Create or replace Content-Type header with new body content type
                ContentType::File(_) | ContentType::Form(_) | ContentType::Raw(_) | ContentType::Json(_) | ContentType::Xml(_) | ContentType::Html(_) | ContentType::Javascript(_) => {
                    let content_type = &selected_request.body.to_content_type();
                    selected_request.modify_or_create_header(CONTENT_TYPE.as_str(), content_type)
                }
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.update_body_table_selection();
        self.load_request_body_param_tab();
    }
}