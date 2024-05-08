use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::app::app::App;
use crate::{panic_error, print_if_not_in_command};
use crate::models::collection::CollectionFileFormat;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub disable_syntax_highlighting: Option<bool>,
    
    #[serde(default)]
    pub disable_cors: Option<bool>,
    
    #[serde(default)]
    pub disable_images_preview: Option<bool>,
    
    #[serde(default)]
    pub preferred_collection_file_format: Option<CollectionFileFormat>,
    
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
    
    pub fn is_cors_disabled(&self) -> bool {
        return self.disable_cors.unwrap_or(false)
    }
    
    pub fn is_image_preview_disabled(&self) -> bool {
        return self.disable_images_preview.unwrap_or(false)
    }
    
    pub fn get_preferred_collection_file_format(&self) -> CollectionFileFormat {
        return match &self.preferred_collection_file_format {
            None => CollectionFileFormat::default(),
            Some(file_format) => file_format.clone()
        };
    }
}

impl App<'_> {
    pub fn parse_config_file(&mut self, path_buf: PathBuf) {
        let mut file_content = String::new();

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

        self.config = config;

        print_if_not_in_command!("Config file parsed!");
    }
}