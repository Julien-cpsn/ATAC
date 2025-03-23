use clap::ValueEnum;
use serde::Serialize;
use strum::{Display, VariantArray};

#[derive(Debug, Default, Display, VariantArray, ValueEnum, Serialize, Clone)]
pub enum ExportFormat {
    #[default]
    #[strum(to_string = "HTTP")]
    #[clap(name = "http")]
    HTTP,

    #[strum(to_string = "cURL")]
    #[clap(name = "curl")]
    Curl,

    #[strum(to_string = "PHP\nGuzzle")]
    #[clap(name = "php")]
    PhpGuzzle,

    #[strum(to_string = "NodeJs\nAxios")]
    #[clap(name = "js")]
    NodeJsAxios,

    #[strum(to_string = "Rust\nReqwest")]
    #[clap(name = "rust")]
    RustReqwest
}

impl ExportFormat {
    pub fn to_extension(&self) -> Option<&str> {
        match self {
            ExportFormat::HTTP => None,
            ExportFormat::Curl => Some("sh"),
            ExportFormat::PhpGuzzle => Some("php"),
            ExportFormat::NodeJsAxios => Some("js"),
            ExportFormat::RustReqwest => Some("rs")
        }
    }
}