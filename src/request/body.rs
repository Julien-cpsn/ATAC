use serde::{Deserialize, Serialize};
use strum::{Display};
use crate::request::body::ContentType::{NoBody, Html, Json, Raw, Xml, Multipart, Form};
use crate::request::request::KeyValue;

#[derive(Default, Debug, Clone, Display, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    #[default]
    #[strum(to_string = "No Body")]
    NoBody,
    #[strum(to_string = "Multipart")]
    Multipart(Vec<KeyValue>),
    #[strum(to_string = "Form")]
    Form(Vec<KeyValue>),
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
    pub fn to_content_type(&self) -> String {
        match &self {
            NoBody => String::new(),
            Multipart(_) => String::from("multipart/form-data"),
            Form(_) => String::from("application/x-www-form-urlencoded"),
            Raw(_) => String::from("text/plain"),
            Json(_) | Xml(_) | Html(_) => format!("application/{}", self.to_string().to_lowercase())
        }
    }

    pub fn get_form(&self) -> Option<&Vec<KeyValue>> {
        match self {
            Multipart(form) | Form(form) => Some(form),
            _ => None
        }
    }

    pub fn get_form_mut(&mut self) -> Option<&mut Vec<KeyValue>> {
        match self {
            Multipart(form) | Form(form) => Some(form),
            _ => None
        }
    }
}

pub fn next_content_type(content_type: &ContentType) -> ContentType {
    match content_type {
        NoBody => Multipart(Vec::new()),
        Multipart(_) => Form(Vec::new()),
        Form(_) => Raw(String::new()),
        Raw(body) => Json(body.to_string()),
        Json(body) => Xml(body.to_string()),
        Xml(body) => Html(body.to_string()),
        Html(_) => NoBody,
    }
}