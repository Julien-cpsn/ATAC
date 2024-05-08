use serde::{Deserialize, Serialize};
use strum::Display;
use crate::models::auth::Auth::{BasicAuth, BearerToken, NoAuth};

#[derive(Clone, Default, Debug, Display, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Auth {
    #[default]
    #[strum(to_string = "No Auth")]
    NoAuth,
    #[strum(to_string = "Basic")]
    BasicAuth(String, String),
    #[strum(to_string = "Bearer")]
    BearerToken(String)
}

pub fn next_auth(auth: &Auth) -> Auth {
    match auth {
        NoAuth => BasicAuth(String::new(), String::new()),
        BasicAuth(_, _) => BearerToken(String::new()),
        BearerToken(_) => NoAuth
    }
}