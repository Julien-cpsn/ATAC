use anyhow::anyhow;
use regex::Regex;
use thiserror::Error;
use tracing::{info};

use crate::app::app::App;
use crate::models::request::KeyValue;

#[derive(Error, Debug)]
pub enum UrlError {
    #[error("The URL is empty")]
    UrlIsEmpty,
}

impl App<'_> {
    pub fn modify_request_url(&mut self, collection_index: usize, request_index: usize, url: String) -> anyhow::Result<()> {
        if url.trim().is_empty() {
            return Err(anyhow!(UrlError::UrlIsEmpty));
        }
        
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));
        
        {
            let mut selected_request = local_selected_request.write();

            let url_parts = url.trim().split_once("?");

            let final_url: String;
            let query_params: &str;

            if let Some((url, found_query_params)) = url_parts {
                final_url = url.to_string();
                query_params = found_query_params;
            } else {
                final_url = url;
                query_params = "";
            }

            let mut found_params = vec![];
            
            let path_params_pattern = Regex::new(r"(\{[\w-]+})").unwrap();
            for (_, [path_param]) in path_params_pattern.captures_iter(&final_url).map(|c| c.extract()) {
                found_params.push((path_param.to_string(), None));
            }
            
            let query_params_pattern = Regex::new(r"(&?([^=]+)=([^&]+))").unwrap();
            for (_, [_, param_name, value]) in query_params_pattern.captures_iter(query_params).map(|c| c.extract()) {
                found_params.push((param_name.to_string(), Some(value.to_string())));
            }
            
            selected_request.params.retain(|param|
                found_params.iter().any(|found| found.0 == param.data.0)
            );

            for found_param in found_params {
                let param = selected_request.params.iter_mut().find(|param| param.data.0 == found_param.0);

                if let Some(param) = param {
                    if let Some(value)  = found_param.1 {
                        param.data.1 = value;
                    }
                }
                else {
                    let value = found_param.1.unwrap_or_else(|| String::from("value"));
                    selected_request.params.push(KeyValue {
                        enabled: true,
                        data: (found_param.0, value),
                    });
                }
            }

            info!("URL set to \"{}\"", &final_url);

            selected_request.url = final_url;
        }
        
        self.save_collection_to_file(collection_index);
        
        Ok(())
    }
}