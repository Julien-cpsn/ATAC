use std::sync::{Arc, RwLock};

use arboard::{Clipboard, ImageData};
use image::EncodableLayout;

use crate::app::app::App;
use crate::app::ui::result_tabs::RequestResultTabs;
use crate::request::request::{Request, ResponseContent};

impl App<'_> {
    pub fn get_selected_request_as_local(&self) -> Arc<RwLock<Request>> {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        self.collections[selected_request_index.0].requests[selected_request_index.1].clone()
    }

    pub fn get_request_as_local_from_indexes(&self, selected_request_index: &(usize, usize)) -> Arc<RwLock<Request>> {
        self.collections[selected_request_index.0].requests[selected_request_index.1].clone()
    }

    /// Copy the response's body content to the clipboard if it's present, otherwise does nothing
    pub fn copy_response_body_content_to_clipboard(&self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read().unwrap();

        let mut clipboard = Clipboard::new().unwrap();

        match self.request_result_tab {
            RequestResultTabs::Body => match &selected_request.result.content {
                None => {}
                Some(content) => match content {
                    ResponseContent::Body(body) => {
                        clipboard.set_text(body).expect("Could not copy response content to clipboard");
                    }
                    ResponseContent::Image(image_response) => match &image_response.image {
                        None => {}
                        Some(image) => {
                            let rgba_image = image.to_rgba8();

                            clipboard
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
            RequestResultTabs::Cookies => match &selected_request.result.cookies {
                None => {}
                Some(cookies) => clipboard.set_text(cookies).expect("Could not copy cookies to clipboard")
            }
            RequestResultTabs::Headers => {
                let headers_string: String = selected_request.result.headers
                    .iter()
                    .map(|(header, value)| format!("{}: {}\n", header, value))
                    .collect();

                clipboard.set_text(headers_string).expect("Could not copy headers to clipboard")
            }
        }
    }
}