use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Default, Debug, Serialize, Deserialize)]
pub struct BasicAuth {
    pub username: String,
    pub password: String
}