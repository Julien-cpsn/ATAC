use std::sync::Arc;
use futures_util::SinkExt;
use reqwest_websocket::CloseCode;
use tokio::task;
use tracing::info;
use crate::app::app::App;
use crate::app::business_logic::request::http::send::send_http_request;
use crate::app::business_logic::request::mqtt::send::{mqtt_disconnect, send_mqtt_request};
use crate::app::business_logic::request::send::PreparedRequest;
use crate::app::business_logic::request::ws::send::send_ws_request;
use crate::models::auth::auth::Auth;
use crate::models::protocol::protocol::Protocol;

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

        match &mut selected_request.protocol {
            Protocol::HttpRequest(_) => {}
            Protocol::MqttRequest(mqtt_request) => if mqtt_request.is_connected {
                mqtt_disconnect(mqtt_request);
                return;
            }
            Protocol::WsRequest(ws_request) => if ws_request.is_connected {
                if let Some(websocket) = ws_request.websocket.clone() {
                    drop(websocket.rx);

                    // Lock each time to avoid potential deadlock if the user is spamming "send request"
                    websocket.tx
                        .lock()
                        .send(reqwest_websocket::Message::Close {
                            code: CloseCode::Normal,
                            reason: String::new(),
                        })
                        .await
                        .unwrap();

                    websocket.tx.lock().close().await.unwrap();

                    ws_request.websocket = None;
                    ws_request.is_connected = false;
                    return;
                }
            }
        }

        /* PRE-REQUEST SCRIPT */

        let prepared_request = match self.prepare_request_for_protocol(&mut selected_request).await {
            Ok(result) => result,
            Err(prepare_request_error) => {
                selected_request.response.status_code = Some(prepare_request_error.to_string());
                return;
            }
        };

        let protocol = selected_request.protocol.clone();
        let local_selected_request = self.get_selected_request_as_local();
        let local_env = self.get_selected_env_as_local();

        let local_should_refresh_scrollbars = Arc::clone(&self.received_response);

        /* SEND REQUEST */

        task::spawn(async move {
            let response = match (protocol, prepared_request) {
                (Protocol::HttpRequest(_), PreparedRequest::Reqwest(prepared_request)) => send_http_request(prepared_request, local_selected_request.clone(), &local_env).await,
                (Protocol::WsRequest(_), PreparedRequest::Reqwest(prepared_request)) => send_ws_request(prepared_request, local_selected_request.clone(), &local_env, local_should_refresh_scrollbars.clone()).await,
                (Protocol::MqttRequest(_), PreparedRequest::Mqtt(prepared_request)) => send_mqtt_request(prepared_request, local_selected_request.clone(), &local_env, local_should_refresh_scrollbars.clone()).await,
                _ => unreachable!()
            };

            match response {
                Ok(response) => {
                    let mut selected_request = local_selected_request.write();

                    match &mut selected_request.auth {
                        Auth::Digest(digest) => digest.update_from_www_authenticate_header(&response.headers),
                        _ => {}
                    }

                    selected_request.response = response;

                    *local_should_refresh_scrollbars.lock() = true;
                    return;
                },
                Err(response_error) => {
                    let mut selected_request = local_selected_request.write();
                    selected_request.response.status_code = Some(response_error.to_string());
                    return;
                }
            };
        });
    }
}