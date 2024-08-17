use clap::Subcommand;
use crate::cli::commands::key_value::KeyValueCommand;
use crate::models::body::ContentType;

#[derive(Subcommand, Debug, Clone)]
pub enum BodySubcommand {
    /// Print the current request body
    Get,

    /// Set the request body
    Set {
        #[command(subcommand)]
        content_type: ContentTypeAsArg
    },

    /// Add, get or set a key/value pair (multipart & form body only)
    Key {
        #[command(subcommand)]
        subcommand: KeyValueCommand
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum ContentTypeAsArg {
    NoBody,

    File {
        file_path: String
    },

    Multipart,

    Form,

    Raw {
        text: String
    },

    Json {
        json: String
    },

    Xml {
        xml: String
    },

    Html {
        html: String
    },

    Javascript {
        javascript: String
    },
}

impl ContentTypeAsArg {
    pub fn to_content_type(&self) -> ContentType {
        match self {
            ContentTypeAsArg::NoBody => ContentType::NoBody,
            ContentTypeAsArg::File { file_path } => ContentType::File(file_path.clone()),
            ContentTypeAsArg::Multipart => ContentType::Multipart(vec![]),
            ContentTypeAsArg::Form => ContentType::Form(vec![]),
            ContentTypeAsArg::Raw { text } => ContentType::Raw(text.clone()),
            ContentTypeAsArg::Json { json } => ContentType::Json(json.clone()),
            ContentTypeAsArg::Xml { xml } => ContentType::Xml(xml.clone()),
            ContentTypeAsArg::Html { html } => ContentType::Html(html.clone()),
            ContentTypeAsArg::Javascript { javascript } => ContentType::Javascript(javascript.clone())
        }
    }
}