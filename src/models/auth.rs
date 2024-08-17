use clap::Subcommand;
use serde::{Deserialize, Serialize};
use strum::Display;
use crate::models::auth::Auth::{BasicAuth, BearerToken, NoAuth};

#[derive(Subcommand, Clone, Default, Debug, Display, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Auth {
    #[default]
    #[strum(to_string = "No Auth")]
    /// No auth method
    NoAuth,
    
    #[strum(to_string = "Basic")]
    #[clap(visible_alias = "basic")]
    /// Basic auth method
    BasicAuth {
        username: String,
        password: String
    },
    
    #[strum(to_string = "Bearer")]
    #[clap(visible_alias = "bearer")]
    /// Bearer token auth method
    BearerToken {
        token: String
    }
}

pub fn next_auth(auth: &Auth) -> Auth {
    match auth {
        NoAuth => BasicAuth {
            username: String::new(),
            password: String::new(),
        },
        BasicAuth { .. } => BearerToken {
            token: String::new(),
        },
        BearerToken { .. } => NoAuth
    }
}