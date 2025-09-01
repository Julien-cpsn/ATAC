use std::sync::Arc;

use tokio::task;
use tracing::info;
use crate::app::app::App;
use crate::app::business_logic::request::send::send_request;

impl App<'_> {
    pub async fn tui_send_request(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let selected_request = local_selected_request.read();
            
            if selected_request.is_pending {
                selected_request.cancellation_token.cancel();
                info!("Request canceled");
                return;
            }
        }
        
        let mut selected_request = local_selected_request.write();

        /* PRE-REQUEST SCRIPT */

        let prepared_request = match self.prepare_request(&mut selected_request).await {
            Ok(result) => result,
            Err(prepare_request_error) => {
                selected_request.response.status_code = Some(prepare_request_error.to_string());
                return;
            }
        };

        let local_selected_request = self.get_selected_request_as_local();
        let local_env = self.get_selected_env_as_local();

        let local_should_refresh_scrollbars = Arc::clone(&self.received_response);

        /* SEND REQUEST */

        task::spawn(async move {
            let response = match send_request(prepared_request, local_selected_request.clone(), &local_env).await {
                Ok(response) => response,
                Err(response_error) => {
                    let mut selected_request = local_selected_request.write();
                    selected_request.response.status_code = Some(response_error.to_string());
                    return;
                }
            };


            let mut selected_request = local_selected_request.write();
            selected_request.response = response;
            *local_should_refresh_scrollbars.lock() = true;
        });
    }
}