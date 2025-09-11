use arboard::ImageData;
use image::EncodableLayout;
use rayon::prelude::*;

use crate::app::app::App;
use crate::models::response::ResponseContent;
use crate::tui::ui::result_tabs::RequestResultTabs;

impl App<'_> {
    /// Copy the response's body content to the clipboard if it's present, otherwise does nothing
    pub fn copy_response_body_content_to_clipboard(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        match self.request_result_tab {
            RequestResultTabs::Body => match &selected_request.response.content {
                None => {}
                Some(content) => match content {
                    ResponseContent::Body(body) => {
                        self.clipboard.set_text(body).expect("Could not copy response content to clipboard");
                    }
                    ResponseContent::Image(image_response) => match &image_response.image {
                        None => {}
                        Some(image) => {
                            let rgba_image = image.to_rgba8();

                            self.clipboard
                                .set_image(ImageData {
                                    width: rgba_image.width() as usize,
                                    height: rgba_image.height() as usize,
                                    bytes: rgba_image.as_bytes().into()
                                })
                                .expect("Could not copy response image to clipboard");
                        }
                    }
                }
            }
            RequestResultTabs::Messages => {
                let ws_request = selected_request.get_ws_request().unwrap();
                let text = ws_request.messages
                    .iter()
                    .map(|m| format!(
                        "=== {} - New {} message from {} ===\n{}",
                        m.timestamp.format("%H:%M:%S %d/%m/%Y").to_string(),
                        m.content.to_string(),
                        m.sender,
                        m.content.to_content()
                    ))
                    .collect::<Vec<String>>()
                    .join("\n");

                if !text.is_empty() {
                    self.clipboard.set_text(text).expect("Could not copy messages content to clipboard");
                }
            },
            RequestResultTabs::Cookies => match &selected_request.response.cookies {
                None => {}
                Some(cookies) => self.clipboard.set_text(cookies).expect("Could not copy cookies to clipboard")
            }
            RequestResultTabs::Headers => {
                let headers_string: String = selected_request.response.headers
                    .par_iter()
                    .map(|(header, value)| format!("{}: {}\n", header, value))
                    .collect();

                if !headers_string.is_empty() {
                    self.clipboard.set_text(headers_string).expect("Could not copy headers to clipboard");
                }
            }
            RequestResultTabs::Console => {
                let text = match (&selected_request.console_output.pre_request_output, &selected_request.console_output.post_request_output) {
                    (None, None) => &String::new(),
                    (Some(pre_request_console_output), None) => pre_request_console_output,
                    (None, Some(post_request_console_output)) => post_request_console_output,
                    (Some(pre_request_console_output), Some(post_request_console_output)) => &format!("{}\n{}", pre_request_console_output, post_request_console_output)
                };

                if !text.is_empty() {
                    self.clipboard.set_text(text).expect("Could not copy console output to clipboard")
                }
            }
        }
    }

    pub fn get_max_line_length(&self, text: &str) -> usize {
        let mut max_length = text.par_lines().fold_with(0, |acc, line| acc + line.chars().count()).sum();

        if self.last_messages_area_size.0 > 0 {
            let max_width = (0.75 * self.last_messages_area_size.0 as f32) as usize + 1;
            if max_length > max_width {
                max_length = max_width;
            }
        }

        max_length
    }
}