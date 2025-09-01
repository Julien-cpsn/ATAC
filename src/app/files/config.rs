use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use std::sync::OnceLock;
use tracing::trace;
use serde::{Deserialize, Serialize};

use crate::app::app::App;
use crate::panic_error;
use crate::models::collection::CollectionFileFormat;

pub static SKIP_SAVE_REQUESTS_RESPONSE: OnceLock<bool> = OnceLock::new();

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub disable_syntax_highlighting: Option<bool>,

    #[serde(default)]
    pub save_requests_response: Option<bool>,
    
    #[serde(default)]
    pub disable_cors: Option<bool>,
    
    #[serde(default)]
    pub disable_images_preview: Option<bool>,

    #[serde(default)]
    pub wrap_responses: Option<bool>,

    #[serde(default)]
    pub preferred_collection_file_format: Option<CollectionFileFormat>,

    #[serde(default)]
    pub proxy: Option<Proxy>
}

#[derive(Default, Serialize, Deserialize)]
pub struct Proxy {
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
}

impl Config {
    pub fn is_syntax_highlighting_disabled(&self) -> bool {
        return self.disable_syntax_highlighting.unwrap_or(false)
    }

    pub fn should_save_requests_reponse(&self) -> bool {
        self.save_requests_response.unwrap_or(false)
    }
    pub fn set_should_skip_requests_reponse(&self) {
        SKIP_SAVE_REQUESTS_RESPONSE.get_or_init(|| match self.save_requests_response {
            None => true,
            Some(save_requests_response) => !save_requests_response
        });
    }
    
    pub fn is_cors_disabled(&self) -> bool {
        return self.disable_cors.unwrap_or(false)
    }
    
    pub fn is_image_preview_disabled(&self) -> bool {
        return self.disable_images_preview.unwrap_or(false)
    }
    
    pub fn should_wrap_body(&self) -> bool {
        return self.wrap_responses.unwrap_or(false)
    }
    
    pub fn get_preferred_collection_file_format(&self) -> CollectionFileFormat {
        match &self.preferred_collection_file_format {
            None => CollectionFileFormat::default(),
            Some(file_format) => file_format.clone()
        }
    }
}

impl App<'_> {
    pub fn parse_config_file(&mut self, path_buf: &PathBuf) {
        let mut file_content = String::new();

        trace!("Trying to open or create \"atac.toml\" config file");

        let mut config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path_buf.clone())
            .expect("\tCould not open config file");

        config_file.read_to_string(&mut file_content).expect("\tCould not read config file");

        let config: Config = match toml::from_str(&file_content) {
            Ok(config) => config,
            Err(e) => panic_error(format!("Could not parse config file\n\t{e}"))
        };

        config.set_should_skip_requests_reponse();

        self.config = config;

        trace!("Config file parsed!");
    }
}
