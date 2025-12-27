use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Default, Debug, Serialize, Deserialize)]
pub struct BearerToken {
    pub token: String
}