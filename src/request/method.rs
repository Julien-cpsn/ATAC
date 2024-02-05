use ratatui::prelude::Color;
use reqwest::Method;


pub fn get_method_bg(method: &Method) -> Color {
    match method {
        &Method::GET => Color::Green,
        &Method::POST => Color::Yellow,
        _ => Color::Black
    }
}

pub fn next_method(method: &Method) -> Method {
    match method {
        &Method::GET => Method::POST,
        &Method::POST => Method::GET,
        _ => Method::GET
    }
}