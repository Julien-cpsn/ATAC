use anyhow::anyhow;
use thiserror::Error;

use crate::app::app::App;
use crate::cli::utils::collection::FindElementError::{CollectionNotFound, RequestNotFound};
use crate::models::collection::Collection;

#[derive(Error, Debug)]
pub enum FindElementError {
    #[error("Collection not found")]
    CollectionNotFound,
    #[error("Request not found")]
    RequestNotFound,
}

impl App<'_> {
    pub fn find_collection(&mut self, collection_name: &str) -> anyhow::Result<usize> {
        for (index, collection) in self.collections.iter().enumerate() {
            if collection.name == collection_name {
                return Ok(index);
            }
        }

        return Err(anyhow!(CollectionNotFound));
    }

    #[allow(dead_code)]
    pub fn find_request(&mut self, collection: &Collection, request_name: &str) -> anyhow::Result<usize> {
        for (index, request) in collection.requests.iter().enumerate() {
            if request.read().name == request_name {
                return Ok(index);
            }
        }

        return Err(anyhow!(RequestNotFound));
    }

    pub fn find_collection_and_request(&mut self, collection_name: &str, request_name: &str) -> anyhow::Result<(usize, usize)> {
        for (collection_index, collection) in self.collections.iter().enumerate() {
            if collection.name != collection_name { 
                continue;
            }
            
            for (request_index, request) in collection.requests.iter().enumerate() {
                if request.read().name == request_name {
                    return Ok((collection_index, request_index));
                }
            }

            return Err(anyhow!(RequestNotFound));
        }

        return Err(anyhow!(CollectionNotFound));
    }
}