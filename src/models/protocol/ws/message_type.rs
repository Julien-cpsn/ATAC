use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, Display, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    #[strum(to_string = "Text")]
    Text(String),

    #[strum(to_string = "Binary")]
    Binary(Box<[u8]>),

    #[strum(to_string = "Ping")]
    Ping(Box<[u8]>),

    #[strum(to_string = "Pong")]
    Pong(Box<[u8]>),

    #[strum(to_string = "Close")]
    Close(String)
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::Text(String::new())
    }
}

impl MessageType {
    pub fn to_content(&self) -> String {
        match &self {
            MessageType::Text(text) | MessageType::Close(text) => text.clone(),
            MessageType::Binary(bytes) | MessageType::Ping(bytes) | MessageType::Pong(bytes) => format!("{:?}", bytes)
        }
    }
}

pub fn next_message_type(message_type: &MessageType) -> MessageType {
    match message_type {
        MessageType::Text(text) => MessageType::Binary(text.as_bytes().to_vec().into_boxed_slice()),
        MessageType::Binary(binary) => MessageType::Ping(binary.clone()),
        MessageType::Ping(ping) => MessageType::Pong(ping.clone()),
        MessageType::Pong(pong) => MessageType::Close(String::from_utf8_lossy(&pong).to_string()),
        MessageType::Close(close) => MessageType::Text(close.clone()),
    }
}
