use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::app::app::App;
use crate::panic_error;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub disable_syntax_highlighting: Option<bool>,
    #[serde(default)]
    pub disable_cors: Option<bool>,
    #[serde(default)]
    pub enable_display_images: Option<bool>,
    pub proxy: Option<Proxy>
}

#[derive(Default, Serialize, Deserialize)]
pub struct Proxy {
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
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

        println!("Config file parsed!");
    }
}