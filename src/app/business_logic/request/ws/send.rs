use std::sync::Arc;
use std::time::{Duration, Instant};
use chrono::{Local};
use futures_util::{StreamExt, TryStreamExt};
use parking_lot::{Mutex, RwLock};
use rayon::prelude::*;
use reqwest::header::CONTENT_TYPE;
use reqwest_websocket::RequestBuilderExt;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::{info, trace};
use crate::app::app::App;
use crate::app::business_logic::request::send::RequestResponseError;
use crate::models::environment::Environment;
use crate::models::protocol::ws::message_type::MessageType;
use crate::models::protocol::ws::ws::{Message, Sender, Websocket};
use crate::models::request::Request;
use crate::models::response::{RequestResponse, ResponseContent};

pub async fn send_ws_request(prepared_request: reqwest_middleware::RequestBuilder, local_request: Arc<RwLock<Request>>, env: &Option<Arc<RwLock<Environment>>>, received_response: Arc<Mutex<bool>>) -> Result<RequestResponse, RequestResponseError> {
    info!("Sending request");

    let mut request = local_request.write();
    request.is_pending = true;

    let cancellation_token = request.cancellation_token.clone();

    let ws_request = request.get_ws_request_mut()?;
    ws_request.is_connected = false;

    let timeout = tokio::time::sleep(Duration::from_secs(30));

    let request_start = Instant::now();
    let elapsed_time: Duration;
    let mut response = tokio::select! {
        _ = cancellation_token.cancelled() => {
            elapsed_time = request_start.elapsed();

            RequestResponse {
                duration: None,
                status_code: Some(String::from("CANCELED")),
                content: None,
                cookies: None,
                headers: vec![],
            }
        },
        _ = timeout => {
            elapsed_time = request_start.elapsed();

            RequestResponse {
                duration: None,
                status_code: Some(String::from("TIMEOUT")),
                content: None,
                cookies: None,
                headers: vec![],
            }
        },
        response = prepared_request.upgrade().send() => match response {
            Ok(response) => {
                elapsed_time = request_start.elapsed();

                let status_code = response.status().to_string();

                let mut is_image = false;

                let headers: Vec<(String, String)> = response.headers()
                    .clone()
                    .iter()
                    .map(|(header_name, header_value)| {
                        let value = header_value.to_str().unwrap_or("").to_string();

                        if header_name == CONTENT_TYPE && value.starts_with("image/") {
                            is_image = true;
                        }

                        (header_name.to_string(), value)
                    })
                    .collect();

                let cookies = response.cookies()
                    .par_bridge()
                    .map(|cookie| {
                        format!("{}: {}", cookie.name(), cookie.value())
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                let websocket = match response.into_websocket().await {
                    Ok(websocket) => websocket,
                    Err(error) => return Err(RequestResponseError::WebsocketError(error))
                };
                let (tx, rx) = websocket.split();
                ws_request.websocket = Some(Websocket {
                    rx: Arc::new(Mutex::new(rx)),
                    tx: Arc::new(Mutex::new(tx))
                });

                RequestResponse {
                    duration: None,
                    status_code: Some(status_code),
                    content: None,
                    cookies: Some(cookies),
                    headers,
                }
            },
            Err(error) => {
                elapsed_time = request_start.elapsed();

                let error = error.to_string();
                let response_status_code = Some(error.clone());
                let result_body = ResponseContent::Body(error);

                RequestResponse {
                    duration: None,
                    status_code: response_status_code,
                    content: Some(result_body),
                    cookies: None,
                    headers: vec![],
                }
            }
        }
    };


    response.duration = Some(format!("{:?}", elapsed_time));

    trace!("Request sent");

    /* POST-REQUEST SCRIPT */

    let (modified_response, post_request_output) = App::handle_post_request_script(&request, response, env)?;

    drop(request);

    {
        let mut request = local_request.write();

        request.console_output.post_request_output = post_request_output;
        request.is_pending = false;
        request.cancellation_token = CancellationToken::new();

        let ws_request = request.get_ws_request_mut().unwrap();
        ws_request.messages = vec![];

        if modified_response.status_code != Some(String::from("101 Switching Protocols")) {
            return Ok(modified_response);
        }
        else {
            ws_request.is_connected = true;
        }
    }

    let local_request = local_request.clone();
    let mut request = local_request.write();
    let ws_request = request.get_ws_request_mut().unwrap();
    let local_websocket = ws_request.websocket.clone().unwrap();

    drop(request);

    tokio::spawn(async move {
        'websocket_loop : loop  {
            if cancellation_token.is_cancelled() {
                let mut request = local_request.write();
                let ws_request = request.get_ws_request_mut().unwrap();
                ws_request.is_connected = false;
                break 'websocket_loop;
            }

            let mut websocket_rx = local_websocket.rx.lock();
            let message = websocket_rx.try_next().await;
            drop(websocket_rx);
            match message {
                Ok(message) => match message {
                    Some(message) => {
                        let message_type = match message {
                            reqwest_websocket::Message::Text(text) => MessageType::Text(text),
                            reqwest_websocket::Message::Binary(binary) => MessageType::Binary(binary.to_vec().into_boxed_slice()),
                            reqwest_websocket::Message::Ping(ping) => MessageType::Ping(ping.to_vec().into_boxed_slice()),
                            reqwest_websocket::Message::Pong(pong) => MessageType::Pong(pong.to_vec().into_boxed_slice()),
                            reqwest_websocket::Message::Close { code, reason } => match reason.is_empty() {
                                true => MessageType::Close(format!("Close code: {}", code)),
                                false => MessageType::Close(format!("Close code: {}, reason: {}", code, reason)),
                            }
                        };

                        let mut request = local_request.write();
                        let ws_request = request.get_ws_request_mut().unwrap();
                        ws_request.messages.push(Message {
                            timestamp: Local::now(),
                            content: message_type,
                            sender: Sender::Server
                        });

                        *received_response.lock() = true;
                    },
                    None => {}
                }
                Err(error) => {
                    let mut request = local_request.write();
                    let ws_request = request.get_ws_request_mut().unwrap();
                    ws_request.is_connected = false;
                    ws_request.messages.push(Message {
                        timestamp: Local::now(),
                        content: MessageType::Close(format!("Connection closed: {}", error)),
                        sender: Sender::Server,
                    });
                    break 'websocket_loop;
                }
            }

            sleep(Duration::from_millis(100)).await;
        }
    });

    return Ok(modified_response);
}