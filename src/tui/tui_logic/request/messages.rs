use chrono::{DateTime, Local};
use crate::models::protocol::mqtt::mqtt::MqttMessageContent;
use crate::models::protocol::protocol::Protocol;
use crate::models::protocol::ws::ws::Sender;
use crate::models::request::Request;

/// A websocket or MQTT message, as displayed in the messages result tab
pub struct DisplayedMessage {
    pub timestamp: DateTime<Local>,
    pub sender: Sender,
    pub content: String,
    /// e.g. the message type, or the MQTT topic and QoS
    pub details: String,
}

pub fn get_displayed_messages(request: &Request) -> Vec<DisplayedMessage> {
    match &request.protocol {
        Protocol::HttpRequest(_) => vec![],
        Protocol::WsRequest(ws_request) => ws_request.messages
            .iter()
            .map(|message| DisplayedMessage {
                timestamp: message.timestamp,
                sender: message.sender.clone(),
                content: message.content.to_content(),
                details: message.content.to_string(),
            })
            .collect(),
        Protocol::MqttRequest(mqtt_request) => mqtt_request.messages
            .iter()
            .map(|message| match &message.content {
                MqttMessageContent::Publish { topic, payload, qos, retain } => DisplayedMessage {
                    timestamp: message.timestamp,
                    sender: message.sender.clone(),
                    content: payload.to_content(),
                    details: match retain {
                        true => format!("{topic} - {qos} - Retained {payload}"),
                        false => format!("{topic} - {qos} - {payload}"),
                    },
                },
                MqttMessageContent::Event(event) => DisplayedMessage {
                    timestamp: message.timestamp,
                    sender: message.sender.clone(),
                    content: event.clone(),
                    details: String::from("Event"),
                },
            })
            .collect(),
    }
}
