use chrono::Local;
use futures_util::SinkExt;
use reqwest_websocket::{Bytes, CloseCode};
use textwrap::wrap;
use tracing::info;
use crate::app::app::App;
use crate::models::protocol::ws::message_type::{next_message_type, MessageType};
use crate::models::protocol::ws::ws::{Message, Sender};

impl App<'_> {
    pub async fn tui_send_request_message(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();
            let selected_ws_request = selected_request.get_ws_request_mut().unwrap();

            info!("Sending message");

            if selected_ws_request.is_connected {
                if let Some(websocket) = &selected_ws_request.websocket {
                    let lines = self.message_text_area.to_lines();

                    selected_ws_request.message_type = match selected_ws_request.message_type {
                        MessageType::Text(_) => MessageType::Text(lines.join("\n")),
                        MessageType::Binary(_) => MessageType::Binary(lines.join("").as_bytes().to_vec().into_boxed_slice()),
                        MessageType::Ping(_) => MessageType::Ping(lines.join("").as_bytes().to_vec().into_boxed_slice()),
                        MessageType::Pong(_) => MessageType::Pong(lines.join("").as_bytes().to_vec().into_boxed_slice()),
                        MessageType::Close(_) => MessageType::Close(lines.join("\n")),
                    };

                    let message = match &selected_ws_request.message_type {
                        MessageType::Text(text) => reqwest_websocket::Message::Text(text.clone()),
                        MessageType::Binary(binary) => reqwest_websocket::Message::Binary(Bytes::from(binary.clone())),
                        MessageType::Ping(ping) => reqwest_websocket::Message::Ping(Bytes::from(ping.clone())),
                        MessageType::Pong(pong) => reqwest_websocket::Message::Pong(Bytes::from(pong.clone())),
                        MessageType::Close(close) => reqwest_websocket::Message::Close {
                            code: CloseCode::Normal,
                            reason: close.clone(),
                        },
                    };

                    websocket.tx.lock().send(message).await.ok();

                    selected_ws_request.messages.push(Message {
                        timestamp: Local::now(),
                        content: selected_ws_request.message_type.clone(),
                        sender: Sender::You,
                    });

                    info!("Message sent");

                    *self.received_response.lock() = true;
                }
            }
            else {
                info!("Websocket is not connected");
            }
        }

        self.tui_load_request_message_param_tab();
        self.select_request_state();
    }

    pub fn tui_next_request_message_type(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();
            let selected_ws_request = selected_request.get_ws_request_mut().unwrap();

            let next_message_type = next_message_type(&selected_ws_request.message_type);

            info!("Message type set to \"{}\"", next_message_type);

            selected_ws_request.message_type = next_message_type;
        }

        self.save_collection_to_file(selected_request_index.0);
    }

    pub fn get_messages_lines_count(&self) -> usize {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();
        let ws_request = selected_request.get_ws_request().unwrap();

        let mut line_count = 0;
        let mut last_sender = None;

        for message in &ws_request.messages {
            let content = message.content.to_content();
            let max_length = self.get_max_line_length(&content);
            let lines = wrap(&content, max_length);

            match message.sender {
                Sender::You => line_count += lines.len() + 1,
                Sender::Server => match last_sender == Some(&message.sender) {
                    true => line_count += lines.len() + 1,
                    false => line_count += lines.len() + 2,
                }
            }

            last_sender = Some(&message.sender);
        }

        line_count
    }
}