use strum::{Display, VariantArray};

#[derive(Default, Display, VariantArray)]
pub enum ExportFormat {
    #[default]
    #[strum(to_string = "Raw")]
    Raw,

    #[strum(to_string = "cURL")]
    Curl,

    #[strum(to_string = "PHP\nGuzzle")]
    PhpGuzzle,

    #[strum(to_string = "NodeJs\nAxios")]
    NodeJsAxios,

    #[strum(to_string = "Rust\nReqwest")]
    RustReqwest
}