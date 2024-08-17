use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RequestScripts {
    pub pre_request_script: Option<String>,
    pub post_request_script: Option<String>,
}

#[derive(ValueEnum, Debug, Clone, Display)]
pub enum ScriptType {
    Pre,
    Post
}