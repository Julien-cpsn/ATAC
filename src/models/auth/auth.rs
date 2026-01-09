use crate::models::auth::basic::BasicAuth;
use crate::models::auth::bearer_token::BearerToken;
use crate::models::auth::jwt::JwtToken;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use strum::Display;
use crate::models::auth::digest::Digest;

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
    BasicAuth(BasicAuth),
    
    #[strum(to_string = "Bearer")]
    #[clap(visible_alias = "bearer")]
    /// Bearer token auth method
    BearerToken(BearerToken),

    #[strum(to_string = "JWT")]
    #[clap(visible_alias = "jwt")]
    /// JWT token auth method
    JwtToken(JwtToken),

    #[strum(to_string = "Digest")]
    #[clap(visible_alias = "digest")]
    /// Digest auth method
    Digest(Digest),
}

impl Auth {
    pub fn get_jwt(&self) -> &JwtToken {
        match self {
            Auth::JwtToken(jwt_token) => jwt_token,
            _ => unreachable!()
        }
    }

    pub fn get_jwt_mut(&mut self) -> &mut JwtToken {
        match self {
            Auth::JwtToken(jwt_token) => jwt_token,
            _ => unreachable!()
        }
    }

    pub fn get_digest(&self) -> &Digest {
        match self {
            Auth::Digest(digest) => digest,
            _ => unreachable!()
        }
    }

    pub fn get_digest_mut(&mut self) -> &mut Digest {
        match self {
            Auth::Digest(digest) => digest,
            _ => unreachable!()
        }
    }
}

pub fn next_auth(auth: &Auth) -> Auth {
    match auth {
        Auth::NoAuth => Auth::BasicAuth(BasicAuth::default()),
        Auth::BasicAuth(_) => Auth::BearerToken(BearerToken::default()),
        Auth::BearerToken(_) => Auth::JwtToken(JwtToken::default()),
        Auth::JwtToken(_) => Auth::Digest(Digest::default()),
        Auth::Digest(_) => Auth::NoAuth
    }
}

