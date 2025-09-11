use tracing::{info};
use crate::app::app::App;
use crate::models::protocol::http::method::next_method;

impl App<'_> {
    pub fn tui_next_request_method(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();
            let selected_http_request = selected_request.get_http_request_mut().unwrap();

            let next_method = next_method(&selected_http_request.method);
            
            info!("Method set to \"{}\"", next_method);
            
            selected_http_request.method = next_method;
        }

        self.save_collection_to_file(selected_request_index.0);
    }
}