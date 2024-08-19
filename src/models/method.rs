use ratatui::prelude::Color;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use crate::app::files::theme::THEME;

#[derive(Default, Debug, Copy, Clone, EnumString, Display, Serialize, Deserialize)]
pub enum Method {
    #[default]
    #[strum(to_string = "GET")]
    GET,
    #[strum(to_string = "POST")]
    POST,
    #[strum(to_string = "PUT")]
    PUT,
    #[strum(to_string = "PATCH")]
    PATCH,
    #[strum(to_string = "DELETE")]
    DELETE,
    #[strum(to_string = "HEAD")]
    HEAD,
    #[strum(to_string = "OPTIONS")]
    OPTIONS
}

impl Method {
    pub fn get_color(&self) -> Color {
        let theme = THEME.read();
        
        match self {
            Method::GET => theme.methods.get,
            Method::POST => theme.methods.post,
            Method::PUT => theme.methods.put,
            Method::PATCH => theme.methods.patch,
            Method::DELETE => theme.methods.delete,
            Method::HEAD => theme.methods.head,
            Method::OPTIONS => theme.methods.options,
        }
    }

    pub fn to_reqwest(&self) -> reqwest::Method {
        match self {
            Method::GET => reqwest::Method::GET,
            Method::POST => reqwest::Method::POST,
            Method::PUT => reqwest::Method::PUT,
            Method::PATCH => reqwest::Method::PATCH,
            Method::DELETE => reqwest::Method::DELETE,
            Method::HEAD => reqwest::Method::HEAD,
            Method::OPTIONS => reqwest::Method::OPTIONS
        }
    }
}

pub fn next_method(method: &Method) -> Method {
    match method {
        &Method::GET => Method::POST,
        &Method::POST => Method::PUT,
        &Method::PUT => Method::PATCH,
        &Method::PATCH => Method::DELETE,
        &Method::DELETE => Method::HEAD,
        &Method::HEAD => Method::OPTIONS,
        &Method::OPTIONS => Method::GET
    }
}