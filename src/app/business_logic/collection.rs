use std::sync::Arc;
use anyhow::anyhow;

use parking_lot::RwLock;
use thiserror::Error;

use crate::app::app::App;
use crate::app::business_logic::collection::CollectionError::{CollectionNameAlreadyExists, CollectionNameIsEmpty};
use crate::app::business_logic::collection::RequestError::RequestNameIsEmpty;
use crate::cli::args::ARGS;
use crate::models::collection::Collection;
use crate::models::request::{DEFAULT_HEADERS, Request};
use crate::models::settings::RequestSettings;

#[derive(Error, Debug)]
pub enum CollectionError {
    #[error("The collection name is empty")]
    CollectionNameIsEmpty,
    #[error("A collection with this name already exists")]
    CollectionNameAlreadyExists,
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("The request name is empty")]
    RequestNameIsEmpty,
}

impl App<'_> {
    pub fn new_collection(&mut self, new_collection_name: String) -> anyhow::Result<()> {
        if new_collection_name.trim().is_empty() {
            return Err(anyhow!(CollectionNameIsEmpty));
        }

        // Check that collection names are unique (like files)
        for collection in &self.collections {
            if new_collection_name == collection.name {
                return Err(anyhow!(CollectionNameAlreadyExists));
            }
        }
        
        let file_format = self.config.get_preferred_collection_file_format();
        
        let new_collection = Collection {
            name: new_collection_name.clone(),
            requests: vec![],
            path: ARGS.directory.join(format!("{}.{}", new_collection_name, file_format.to_string())),
            file_format,
        };

        self.collections.push(new_collection);

        let collection_index= self.collections.len() - 1;

        self.save_collection_to_file(collection_index);

        Ok(())
    }

    pub fn new_request(&mut self, new_request_name: String) -> Result<(), RequestError> {
        if new_request_name.trim().is_empty() {
            return Err(RequestNameIsEmpty);
        }

        let new_request = Request {
            name: new_request_name,
            headers: DEFAULT_HEADERS.clone(),
            settings: RequestSettings::default(),
            ..Default::default()
        };

        let selected_collection = self.new_request_popup.selected_collection;

        self.collections[selected_collection].requests.push(Arc::new(RwLock::new(new_request)));

        self.save_collection_to_file(selected_collection);

        Ok(())
    }

    pub fn delete_collection(&mut self, collection_index: usize) {
        let collection = self.collections.remove(collection_index);
        self.delete_collection_file(collection);
    }

    pub fn delete_request(&mut self, collection_index: usize, request_index: usize) {
        self.collections[collection_index].requests.remove(request_index);
        self.save_collection_to_file(collection_index);
    }

    pub fn rename_collection(&mut self, new_collection_name: String, collection_index: usize) -> anyhow::Result<()> {
        if new_collection_name.trim().is_empty() {
            return Err(anyhow!(CollectionNameIsEmpty));
        }

        // Check that collection names are unique (like files)
        for collection in &self.collections {
            if new_collection_name == collection.name {
                return Err(anyhow!(CollectionNameIsEmpty));
            }
        }

        self.collections[collection_index].name = new_collection_name.to_string();
        self.save_collection_to_file(collection_index);

        Ok(())
    }

    pub fn rename_request(&mut self, new_request_name: String, collection_index: usize, request_index: usize) -> Result<(), RequestError> {
        if new_request_name.trim().is_empty() {
            return Err(RequestNameIsEmpty);
        }

        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();
            selected_request.name = new_request_name.to_string();
        }
        
        self.save_collection_to_file(collection_index);
        
        Ok(())
    }
}