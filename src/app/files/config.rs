use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::app::app::App;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
}

impl App<'_> {
    pub fn parse_config_file(&mut self, path_buf: PathBuf) {
        let mut file_content = String::new();

        let mut config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path_buf.clone())
            .expect("\tCould not open config file");

        config_file.read_to_string(&mut file_content).expect("\tCould not read config file");

        if file_content.len() == 0 {
            let config_toml = toml::to_string_pretty(&self.config).expect("\tCould not serialize config file");
            config_file.write_all(config_toml.as_bytes()).expect("\tCould not write to config file");
        }
        else {
            let config: Config = toml::from_str(&file_content).expect("\tCould not parse config file");

            self.config = config
        }
    }
}