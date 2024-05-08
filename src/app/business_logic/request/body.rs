use reqwest::header::CONTENT_TYPE;
use thiserror::Error;

use crate::app::app::App;
use crate::app::business_logic::request::body::FormError::NotAForm;
use crate::models::body::ContentType;
use crate::models::request::KeyValue;

#[derive(Error, Debug)]
pub enum FormError {
    #[error("The request body is not a form")]
    NotAForm,
}

impl App<'_> {
    pub fn modify_request_body(&mut self, content_type: ContentType, collection_index: usize, request_index: usize) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            selected_request.body = content_type;
        }

        self.save_collection_to_file(collection_index);
    }

    pub fn modify_request_content_type(&mut self, content_type: ContentType, collection_index: usize, request_index: usize) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            selected_request.body = content_type;

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

        self.save_collection_to_file(collection_index);
    }

    pub fn modify_request_form_data(&mut self, input_text: String, column: usize, row: usize, collection_index: usize, request_index: usize) -> Result<(), FormError> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            match selected_request.body.get_form_mut() {
                Some(form) => match column {
                    0 => form[row].data.0 = input_text.clone(),
                    1 => form[row].data.1 = input_text.clone(),
                    _ => {}
                },
                None => {
                    return Err(NotAForm)
                }
            }
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn create_new_form_data(&mut self, key: String, value: String, collection_index: usize, request_index: usize) -> Result<(), FormError> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            match selected_request.body.get_form_mut() {
                Some(form) => {
                    form.push(KeyValue {
                        enabled: true,
                        data: (key, value)
                    });
                },
                None => {
                    return Err(NotAForm)
                }
            }
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn delete_form_data(&mut self, row: usize, collection_index: usize, request_index: usize) -> Result<(), FormError> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            match selected_request.body.get_form_mut() {
                Some(form) => {
                    form.remove(row);
                },
                None => {
                    return Err(NotAForm)
                }
            }
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn toggle_form_data(&mut self, row: usize, collection_index: usize, request_index: usize) -> Result<(), FormError> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            match selected_request.body.get_form_mut() {
                Some(form) => {
                    form[row].enabled = !form[row].enabled;
                },
                None => {
                    return Err(NotAForm)
                }
            }
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }
}