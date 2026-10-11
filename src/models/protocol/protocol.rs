use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use thiserror::Error;
use crate::models::protocol::http::http::HttpRequest;
use crate::models::protocol::ws::ws::WsRequest;
use crate::models::protocol::mqtt::mqtt::MqttRequest;

#[derive(Error, Debug)]
pub enum ProtocolTypeError {
    #[error("The request is not an HTTP request")]
    NotAnHttpRequest,
    #[error("The request is not an websocket request")]
    NotAWsRequest,
    #[error("The request is not an MQTT request")]
    NotAnMqttRequest
}

#[derive(Debug, Clone, EnumString, Display, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Protocol {
    #[serde(rename = "http", alias = "http", alias = "HTTP")]
    #[strum(to_string = "HTTP")]
    HttpRequest(HttpRequest),

    #[serde(rename = "websocket", alias = "websocket", alias = "WEBSOCKET")]
    #[strum(to_string = "websocket")]
    WsRequest(WsRequest),

    #[serde(rename = "mqtt", alias = "mqtt", alias = "MQTT")]
    #[strum(to_string = "MQTT")]
    MqttRequest(MqttRequest)
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::HttpRequest(HttpRequest::default())
    }
}