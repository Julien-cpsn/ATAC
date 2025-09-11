use crate::models::protocol::protocol::Protocol;
use crate::models::protocol::http::http::HttpRequest;
use crate::models::protocol::ws::ws::WsRequest;
use crate::tui::utils::stateful::text_input::TextInput;

#[derive(Default)]
pub struct NewRequestPopup {
    pub selection: usize,

    pub selected_collection: usize,
    pub max_collection_selection: usize,

    pub protocol: Protocol,

    pub text_input: TextInput
}

impl NewRequestPopup {
    pub fn next_input(&mut self) {
        self.selection = match self.selection {
            0 => 1,
            1 => 2,
            2 => 0,
            _ => unreachable!()
        }
    }

    pub fn previous_input(&mut self) {
        self.selection = match self.selection {
            0 => 2,
            1 => 0,
            2 => 1,
            _ => unreachable!()
        }
    }

    pub fn input_left(&mut self) {
        match self.selection {
            0 => self.previous_collection(),
            1 => self.previous_protocol(),
            2 => self.text_input.move_cursor_left(),
            _ => unreachable!()
        }
    }

    pub fn input_right(&mut self) {
        match self.selection {
            0 => self.next_collection(),
            1 => self.next_protocol(),
            2 => self.text_input.move_cursor_right(),
            _ => unreachable!()
        }
    }

    pub fn next_collection(&mut self) {
        if self.selected_collection + 1 < self.max_collection_selection {
            self.selected_collection += 1;
        }
        else {
            self.selected_collection = 0;
        }
    }

    pub fn previous_collection(&mut self) {
        if self.selected_collection as isize - 1 >= 0 {
            self.selected_collection -= 1;
        }
        else {
            self.selected_collection = self.max_collection_selection - 1;
        }
    }

    pub fn next_protocol(&mut self) {
        self.protocol = match self.protocol {
            Protocol::HttpRequest(_) => Protocol::WsRequest(WsRequest::default()),
            Protocol::WsRequest(_) => Protocol::HttpRequest(HttpRequest::default()),
        }
    }

    pub fn previous_protocol(&mut self) {
        self.protocol = match self.protocol {
            Protocol::HttpRequest(_) => Protocol::WsRequest(WsRequest::default()),
            Protocol::WsRequest(_) => Protocol::HttpRequest(HttpRequest::default()),
        }
    }
}