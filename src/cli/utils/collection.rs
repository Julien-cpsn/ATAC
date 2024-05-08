use std::sync::Arc;
use parking_lot::RwLock;
use thiserror::Error;
use crate::app::app::App;
use crate::cli::utils::collection::FindElementError::{CollectionNotFound, RequestNotFound};
use crate::models::collection::Collection;
use crate::models::request::Request;

#[derive(Error, Debug)]
pub enum FindElementError {
    #[error("Collection not found")]
    CollectionNotFound,
    #[error("Request not found")]
    RequestNotFound,
}

impl<'a> App<'a> {
    pub fn find_collection(&mut self, collection_name: String) -> Result<&Collection, FindElementError> {
        for collection in &self.collections {
            if collection.name == collection_name {
                return Ok(collection);
            }
        }

        return Err(CollectionNotFound);
    }

    pub fn find_request(&mut self, collection: &Collection, request_name: String) -> Result<Arc<RwLock<Request>>, FindElementError> {
        for request in &collection.requests {
            if request.read().name == request_name {
                return Ok(request.clone());
            }
        }

        return Err(RequestNotFound);
    }

    pub fn find_collection_and_request(&mut self, collection_name: String, request_name: String) -> Result<Arc<RwLock<Request>>, FindElementError> {
        for collection in &self.collections {
            if collection.name != collection_name { 
                continue;
            }
            
            for request in &collection.requests {
                if request.read().name == request_name {
                    return Ok(request.clone());
                }
            }

            return Err(RequestNotFound);
        }

        return Err(CollectionNotFound);
    }
}