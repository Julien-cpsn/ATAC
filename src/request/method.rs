use ratatui::prelude::Color;
use reqwest::Method;


pub fn get_method_bg(method: &Method) -> Color {
    match method {
        &Method::GET => Color::Green,
        &Method::POST => Color::Rgb(231, 186, 0),
        &Method::PUT => Color::LightBlue,
        &Method::DELETE => Color::LightRed,
        &Method::PATCH => Color::LightCyan,
        _ => Color::Black
    }
}

pub fn next_method(method: &Method) -> Method {
    match method {
        &Method::GET => Method::POST,
        &Method::POST => Method::PUT,
        &Method::PUT => Method::DELETE,
        &Method::DELETE => Method::PATCH,
        &Method::PATCH => Method::GET,
        _ => Method::GET
    }
}