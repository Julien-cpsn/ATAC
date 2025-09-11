use std::io::stdout;
use crate::app::app::App;
use crate::cli::commands::request_commands::send::SendCommand;
use crate::models::request::Request;
use crate::models::response::ResponseContent;
use anyhow::anyhow;
use parking_lot::RwLock;
use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::prelude::CrosstermBackend;
use ratatui::{Terminal, TerminalOptions, Viewport};
use ratatui_image::picker::Picker;
use ratatui_image::{Resize, ResizeEncodeRender};
use std::sync::Arc;
use chrono::Local;
use futures_util::SinkExt;
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing::info;
use crate::app::business_logic::request::http::send::send_http_request;
use crate::app::business_logic::request::ws::send::send_ws_request;
use crate::models::protocol::protocol::Protocol;
use crate::models::protocol::ws::message_type::MessageType;
use crate::models::protocol::ws::ws::{Message, Sender};

impl App<'_> {
    pub async fn cli_send_request(&mut self, collection_index: usize, request_index: usize, send_command: &SendCommand) -> anyhow::Result<()> {
        let local_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        self.local_send_request(&send_command, local_request).await?;

        if self.config.should_save_requests_reponse() {
            self.save_collection_to_file(collection_index);
        }

        Ok(())
    }

    pub async fn cli_send_collection(&mut self, collection_name: &str, send_command: &SendCommand) -> anyhow::Result<()> {
        let collection_index = self.find_collection(collection_name)?;
        let collection = &self.collections[collection_index];

        let mut requests: Vec<Arc<RwLock<Request>>> = vec![];

        for request in &collection.requests {
            let local_request = request.clone();
            requests.push(local_request);
        }

        for request in requests {
            self.local_send_request(&send_command, request).await?;

            if self.config.should_save_requests_reponse() {
                self.save_collection_to_file(collection_index);
            }
        }

        Ok(())
    }

    pub async fn local_send_request(&mut self, send_command: &SendCommand, local_request: Arc<RwLock<Request>>) -> anyhow::Result<()> {
        let mut request = local_request.write();

         if let Some(env_name )= &send_command.env {
            let env_index = self.find_environment(env_name)?;
             self.selected_environment = env_index;
        };
        
        if send_command.request_name {
            println!("{}", request.name);
        }
        
        let prepared_request = match self.prepare_request(&mut request).await {
            Ok(prepared_request) => prepared_request,
            Err(error) => {
                if send_command.console {
                    if let Some(pre_request_output) = &request.console_output.pre_request_output {
                        println!("{}", pre_request_output);
                    }
                }

                return Err(anyhow!(error));
            }
        };

        let protocol = request.protocol.clone();

        drop(request);

        let local_env = self.get_selected_env_as_local();
        let response = match protocol {
            Protocol::HttpRequest(_) => send_http_request(prepared_request, local_request.clone(), &local_env).await?,
            Protocol::WsRequest(_) => send_ws_request(prepared_request, local_request.clone(), &local_env, self.received_response.clone()).await?,
        };

        let request = local_request.read();

        if send_command.status_code {
            println!("{}", response.status_code.as_ref().unwrap());
        }

        if send_command.duration {
            println!("{}", response.duration.unwrap());
        }

        if send_command.cookies {
            println!("{}", response.cookies.unwrap());
        }

        if send_command.headers {
            println!("{:?}", response.headers);
        }

        if send_command.console {
            let console_output = match (&request.console_output.post_request_output, &request.console_output.post_request_output) {
                (None, None) => &String::new(),
                (Some(pre_request_console_output), None) => pre_request_console_output,
                (None, Some(post_request_console_output)) => post_request_console_output,
                (Some(pre_request_console_output), Some(post_request_console_output)) => &format!("{pre_request_console_output}\n{post_request_console_output}")
            };

            println!("{}", console_output);
        }

        if !send_command.hide_content {
            match response.content {
                None => {},
                Some(content) => match content {
                    ResponseContent::Body(body) => println!("{}", body),
                    ResponseContent::Image(image) => match image.image {
                        None => {
                            println!("{:?}", image.data)
                        }
                        Some(dynamic_image) => {
                            let image_width = dynamic_image.width() as f32;
                            let image_height = dynamic_image.height() as f32;

                            let backend = CrosstermBackend::new(stdout());
                            let terminal_size = backend.size()?;

                            let width_ratio = terminal_size.width as f32 / image_width;
                            let height_ratio = terminal_size.height as f32 / image_height;

                            let ratio = width_ratio.min(height_ratio);

                            let mut terminal = Terminal::with_options(
                                backend,
                                TerminalOptions {
                                    viewport: Viewport::Inline((image_height * ratio) as u16),
                                }
                            )?;

                            let mut picker = Picker::from_query_stdio()
                                .unwrap_or(Picker::from_fontsize((7, 14)))
                                .new_resize_protocol(dynamic_image);

                            terminal.draw(|frame|
                                picker.resize_encode_render(
                                    &Resize::Fit(None),
                                    Rect {
                                        x: 0,
                                        y: 0,
                                        width: (image_width * ratio) as u16,
                                        height: (image_height * ratio) as u16,
                                    },
                                    frame.buffer_mut()
                                ))?;
                        }
                    }
                }
            };
        }
        drop(request);

        if let Protocol::WsRequest(_) = &protocol {
            let mut last_length = 0;
            let local_local_request = local_request.clone();

            tokio::spawn(async move {
                let stdin = io::stdin();
                let reader = BufReader::new(stdin);
                let mut lines = reader.lines();
                let mut buffer = String::new();

                loop {
                    if let Ok(Some(line)) = lines.next_line().await {
                        if line.ends_with("\u{1b}") {
                            let line = &line[..line.len() - 1];
                            buffer.push_str(&line);
                            buffer.push('\n');
                        }
                        else {
                            buffer.push_str(&line);
                            let text = buffer.clone();
                            buffer.clear();

                            let mut request = local_local_request.write();
                            let ws_request = request.get_ws_request_mut().unwrap();

                            if ws_request.is_connected {
                                if let Some(websocket) = &ws_request.websocket {
                                    info!("Sending message");

                                    websocket.tx.lock().send(reqwest_websocket::Message::Text(text.clone())).await.unwrap();

                                    ws_request.messages.push(Message {
                                        timestamp: Local::now(),
                                        sender: Sender::You,
                                        content: MessageType::Text(text),
                                    });
                                }
                            }
                        }
                    }
                }
            });

            loop {
                if let Some(request) = local_request.try_read() {
                    let ws_request = request.get_ws_request()?;

                    if !ws_request.is_connected {
                        break;
                    }

                    let messages = &ws_request.messages[last_length..];

                    for message in messages {
                        println!(
                            "=== {} - New {} message from {} ===\n{}",
                            message.timestamp.format("%H:%M:%S %d/%m/%Y").to_string(),
                            message.content.to_string(),
                            message.sender,
                            message.content.to_content()
                        )
                    }

                    last_length = ws_request.messages.len();
                }
            }
        }

        Ok(())
    }
}