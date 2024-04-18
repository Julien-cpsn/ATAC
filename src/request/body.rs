use regex::Regex;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::request::body::ContentType::{File, Form, Html, Json, Multipart, NoBody, Raw, Xml};
use crate::request::request::KeyValue;

#[derive(Default, Debug, Clone, Display, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    #[default]
    #[strum(to_string = "No Body")]
    NoBody,
    #[strum(to_string = "File")]
    File(String),
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
            File(_) => String::from("application/octet-stream"),
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
        Form(_) => File(String::new()),
        File(_) => Raw(String::new()),
        Raw(body) => Json(body.to_string()),
        Json(body) => Xml(body.to_string()),
        Xml(body) => Html(body.to_string()),
        Html(_) => NoBody,
    }
}

/// Iter through the headers and tries to catch a file format like `application/<file_format>`
pub fn find_file_format_in_content_type(headers: &Vec<(String, String)>) -> Option<String> {
    if let Some((_, content_type)) = headers.iter().find(|(header, _)| *header == "content-type") {
        // Regex that likely catches the file format
        let regex = Regex::new(r"\w+/(?<file_format>\w+)").unwrap();

        return match regex.captures(content_type) {
            // No file format found
            None => None,
            // File format found
            Some(capture) => Some(capture["file_format"].to_string())
        }
    }
    else {
        return None;
    }
}