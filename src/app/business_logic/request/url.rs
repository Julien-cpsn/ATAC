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
                found_params.push(KeyValue {
                    enabled: true,
                    data: (path_param.to_string(), String::from("value")),
                });
            }
            
            let query_params_pattern = Regex::new(r"(&?([^=]+)=([^&]+))").unwrap();
            for (_, [_, param_name, value]) in query_params_pattern.captures_iter(query_params).map(|c| c.extract()) {
                found_params.push(KeyValue {
                    enabled: true,
                    data: (param_name.to_string(), value.to_string()),
                });
            }
            
            selected_request.params.retain(|param|
                found_params.iter().any(|found| found.data.0 == param.data.0)
            );

            for found_param in found_params {
                if !selected_request.params.iter().any(|param| param.data.0 == found_param.data.0) {
                    selected_request.params.push(found_param);
                }
            }

            info!("URL set to \"{}\"", &final_url);

            selected_request.url = final_url;
        }
        
        self.save_collection_to_file(collection_index);
        
        Ok(())
    }
}