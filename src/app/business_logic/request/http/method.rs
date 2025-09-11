use tracing::{info};

use crate::app::app::App;
use crate::models::protocol::http::method::Method;

impl App<'_> {
    pub fn modify_request_method(&mut self, collection_index: usize, request_index: usize, method: Method) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();
            let selected_http_request = selected_request.get_http_request_mut()?;

            info!("Method set to \"{}\"", method);

            selected_http_request.method = method;
        }

        self.save_collection_to_file(collection_index);

        Ok(())
    }
}