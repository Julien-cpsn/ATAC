use std::sync::Arc;
use chrono::{DateTime, Local};
use futures_util::stream::{SplitSink, SplitStream};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use strum::Display;
use crate::app::files::config::SKIP_SAVE_REQUESTS_RESPONSE;
use crate::models::protocol::ws::message_type::MessageType;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct WsRequest {
    #[serde(skip_serializing_if = "should_skip_requests_messages", default = "Vec::default")]
    pub messages: Vec<Message>,

    #[serde(skip)]
    pub message_type: MessageType,

    #[serde(skip)]
    pub websocket: Option<Websocket>,

    #[serde(skip)]
    pub is_connected: bool,
}

#[derive(Debug, Clone)]
pub struct Websocket {
    pub rx: Arc<Mutex<SplitStream<reqwest_websocket::WebSocket>>>,
    pub tx: Arc<Mutex<SplitSink<reqwest_websocket::WebSocket, reqwest_websocket::Message>>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub timestamp: DateTime<Local>,
    pub sender: Sender,
    pub content: MessageType,
}

#[derive(Default, Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sender {
    #[default]
    You,
    Server
}

pub fn should_skip_requests_messages(_: &Vec<Message>) -> bool {
    *SKIP_SAVE_REQUESTS_RESPONSE.get().unwrap_or(&true)
}