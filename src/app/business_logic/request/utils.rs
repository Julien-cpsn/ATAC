use std::sync::Arc;

use parking_lot::RwLock;

use crate::app::app::App;
use crate::models::request::Request;

impl App<'_> {
    pub fn get_selected_request_as_local(&self) -> Arc<RwLock<Request>> {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        self.collections[selected_request_index.0].requests[selected_request_index.1].clone()
    }

    pub fn get_request_as_local_from_indexes(&self, selected_request_index: &(usize, usize)) -> Arc<RwLock<Request>> {
        self.collections[selected_request_index.0].requests[selected_request_index.1].clone()
    }
}