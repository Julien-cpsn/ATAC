use serde::{Deserialize, Serialize};
use strum::{Display};
use crate::request::body::ContentType::{NoBody, Html, Json, Raw, Xml};

#[derive(Clone, Default, Display, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    #[default]
    #[strum(to_string = "No Body")]
    NoBody,
    #[strum(to_string = "Text")]
    Raw(String),
    #[strum(to_string = "JSON")]
    Json(String),
    #[strum(to_string = "XML")]
    Xml(String),
    #[strum(to_string = "HTML")]
    Html(String)
}

impl ContentType {
    pub fn get_body_as_string(&self) -> String {
        match &self {
            NoBody => String::new(),
            Raw(body) | Json(body) | Xml(body) | Html(body) => body.to_string()
        }
    }

    pub fn to_content_type(&self) -> String {
        match &self {
            NoBody => String::new(),
            Raw(_) => String::from("text/plain"),
            Json(_) | Xml(_) | Html(_) => format!("application/{}", self.to_string().to_lowercase())
        }
    }
}

pub fn next_content_type(content_type: &ContentType) -> ContentType {
    match content_type {
        NoBody => Raw(String::new()),
        Raw(body) => Json(body.to_string()),
        Json(body) => Xml(body.to_string()),
        Xml(body) => Html(body.to_string()),
        Html(_) => NoBody
    }
}