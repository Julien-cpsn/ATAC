use anyhow::anyhow;
use thiserror::Error;
use tracing::{info};

use crate::app::app::App;

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

            selected_request.update_url_and_params(url);

            info!("URL set to \"{}\"", &selected_request.url);
        }
        
        self.save_collection_to_file(collection_index);
        
        Ok(())
    }
}