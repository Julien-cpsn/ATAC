use strum::{Display, VariantArray};

#[derive(Default, Display, VariantArray)]
pub enum ExportFormat {
    #[default]
    #[strum(to_string = "HTTP")]
    HTTP,

    #[strum(to_string = "cURL")]
    Curl,

    #[strum(to_string = "PHP\nGuzzle")]
    PhpGuzzle,

    #[strum(to_string = "NodeJs\nAxios")]
    NodeJsAxios,

    #[strum(to_string = "Rust\nReqwest")]
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