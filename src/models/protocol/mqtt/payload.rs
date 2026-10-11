use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "lowercase")]
pub enum MqttPayload {
    #[strum(to_string = "Text")]
    Text(String),

    #[strum(to_string = "Binary")]
    Binary(Box<[u8]>),
}

impl Default for MqttPayload {
    fn default() -> Self {
        MqttPayload::Text(String::new())
    }
}

impl MqttPayload {
    /// MQTT payloads are raw bytes, show them as text when they are valid UTF-8
    pub fn from_bytes(bytes: &[u8]) -> Self {
        match std::str::from_utf8(bytes) {
            Ok(text) => MqttPayload::Text(text.to_string()),
            Err(_) => MqttPayload::Binary(bytes.to_vec().into_boxed_slice()),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match &self {
            MqttPayload::Text(text) => text.as_bytes().to_vec(),
            MqttPayload::Binary(bytes) => bytes.to_vec(),
        }
    }

    pub fn to_content(&self) -> String {
        match &self {
            MqttPayload::Text(text) => text.clone(),
            MqttPayload::Binary(bytes) => format!("{:?}", bytes)
        }
    }
}

pub fn next_payload_type(payload: &MqttPayload) -> MqttPayload {
    match payload {
        MqttPayload::Text(text) => MqttPayload::Binary(text.as_bytes().to_vec().into_boxed_slice()),
        MqttPayload::Binary(binary) => MqttPayload::Text(String::from_utf8_lossy(binary).to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::models::protocol::mqtt::payload::{next_payload_type, MqttPayload};

    #[test]
    fn utf8_bytes_are_text() {
        assert_eq!(MqttPayload::from_bytes("21.5 °C".as_bytes()), MqttPayload::Text(String::from("21.5 °C")));
        assert_eq!(MqttPayload::from_bytes(b""), MqttPayload::Text(String::new()));
    }

    #[test]
    fn other_bytes_are_binary_and_kept_intact() {
        let bytes: Vec<u8> = (0..=255).collect();
        let payload = MqttPayload::from_bytes(&bytes);

        assert!(matches!(payload, MqttPayload::Binary(_)));
        assert_eq!(payload.to_bytes(), bytes);
    }

    #[test]
    fn switching_type_keeps_the_text() {
        let text = MqttPayload::Text(String::from("hello"));
        let binary = next_payload_type(&text);

        assert_eq!(binary.to_bytes(), b"hello");
        assert_eq!(next_payload_type(&binary), text);
    }

    #[test]
    fn saved_payload_round_trips() {
        for payload in [MqttPayload::Text(String::from("a")), MqttPayload::Binary(vec![0, 255].into_boxed_slice())] {
            let json = serde_json::to_string(&payload).unwrap();
            assert_eq!(serde_json::from_str::<MqttPayload>(&json).unwrap(), payload);
        }
    }
}
