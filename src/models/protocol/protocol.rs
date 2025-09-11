use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use thiserror::Error;
use crate::models::protocol::http::http::HttpRequest;
use crate::models::protocol::ws::ws::WsRequest;

#[derive(Error, Debug)]
pub enum ProtocolTypeError {
    #[error("The request is not an HTTP request")]
    NotAnHttpRequest,
    #[error("The request is not an websocket request")]
    NotAWsRequest
}

#[derive(Debug, Clone, EnumString, Display, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Protocol {
    #[serde(rename = "http", alias = "http", alias = "HTTP")]
    #[strum(to_string = "HTTP")]
    HttpRequest(HttpRequest),

    #[serde(rename = "websocket", alias = "websocket", alias = "WEBSOCKET")]
    #[strum(to_string = "websocket")]
    WsRequest(WsRequest)
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::HttpRequest(HttpRequest::default())
    }
}