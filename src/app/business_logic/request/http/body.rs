use reqwest::header::CONTENT_TYPE;
use thiserror::Error;
use tracing::{info};

use crate::app::app::App;
use crate::app::business_logic::key_value::find_key;
use crate::models::protocol::http::body::ContentType;
use crate::models::request::KeyValue;

#[derive(Error, Debug)]
pub enum FormError {
    #[error("The request body is not a form")]
    NotAForm,
}

impl App<'_> {    
    pub fn modify_request_content_type(&mut self, collection_index: usize, request_index: usize, content_type: ContentType) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();
            let selected_http_request = selected_request.get_http_request_mut()?;

            info!("Body content-type set to \"{}\"", content_type);

            selected_http_request.body = content_type;

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

        self.save_collection_to_file(collection_index);
        
        Ok(())
    }

    pub fn find_form_data(&mut self, collection_index: usize, request_index: usize, key: &str) -> anyhow::Result<usize> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));
        let selected_request = local_selected_request.read();
        let selected_http_request = selected_request.get_http_request()?;

        let form = selected_http_request.body.get_form()?;
        find_key(form, key)
    }
    
    pub fn modify_request_form_data(&mut self, collection_index: usize, request_index: usize, value: String, column: usize, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();
            let selected_http_request = selected_request.get_http_request_mut()?;
            let form = selected_http_request.body.get_form_mut()?;

            let form_data_type = match column {
                0 => "key",
                1 => "value",
                _ => ""
            };
            
            info!("Form data {form_data_type} set to \"{value}\"");

            match column {
                0 => form[row].data.0 = value.clone(),
                1 => form[row].data.1 = value.clone(),
                _ => {}
            }
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn create_new_form_data(&mut self, collection_index: usize, request_index: usize, key: String, value: String) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();
            let selected_http_request = selected_request.get_http_request_mut()?;
            let form = selected_http_request.body.get_form_mut()?;

            info!("Key \"{key}\" with value \"{value}\" added to the body form");

            form.push(KeyValue {
                enabled: true,
                data: (key, value)
            });
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn delete_form_data(&mut self, collection_index: usize, request_index: usize, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();
            let selected_http_request = selected_request.get_http_request_mut()?;
            let form = selected_http_request.body.get_form_mut()?;

            info!("Form key deleted");

            form.remove(row);
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn toggle_form_data(&mut self, collection_index: usize, request_index: usize, state: Option<bool>, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();
            let selected_http_request = selected_request.get_http_request_mut()?;
            let form = selected_http_request.body.get_form_mut()?;

            let new_state = match state {
                None => {
                    let state = !form[row].enabled;
                    // Better user feedback
                    println!("{state}");
                    state
                },
                Some(state) => state
            };

            info!("Body form key state set to \"{new_state}\"");

            form[row].enabled = new_state;
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn duplicate_form_data(&mut self, collection_index: usize, request_index: usize, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();
            let selected_http_request = selected_request.get_http_request_mut()?;
            let form = selected_http_request.body.get_form_mut()?;
            

            info!("Body form key duplicated");

            let form_data = form[row].clone();
            form.insert(row, form_data);
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }
}