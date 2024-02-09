use strum::{Display};
use crate::request::body::ContentType::{NoBody, HTML, JSON, Raw, XML};

#[derive(Clone, Default, Display)]
pub enum ContentType {
    #[default]
    #[strum(to_string = "No Body")]
    NoBody,
    #[strum(to_string = "Text")]
    Raw(String),
    #[strum(to_string = "JSON")]
    JSON(String),
    #[strum(to_string = "XML")]
    XML(String),
    #[strum(to_string = "HTML")]
    HTML(String)
}

impl ContentType {
    pub fn get_body_as_string(&self) -> String {
        match &self {
            NoBody => String::new(),
            Raw(body) | JSON(body) | XML(body) | HTML(body) => body.to_string()
        }
    }

    pub fn to_content_type(&self) -> String {
        match &self {
            NoBody => String::new(),
            Raw(_) => String::from("text/plain"),
            JSON(_) | XML(_) | HTML(_) => format!("application/{}", self.to_string().to_lowercase())
        }
    }
}

pub fn next_content_type(content_type: &ContentType) -> ContentType {
    match content_type {
        NoBody => Raw(String::new()),
        Raw(body) => JSON(body.to_string()),
        JSON(body) => XML(body.to_string()),
        XML(body) => HTML(body.to_string()),
        HTML(_) => NoBody
    }
}