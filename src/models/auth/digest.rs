use clap::{Args, ValueEnum};
use digest_auth::{AuthContext, WwwAuthenticateHeader};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use strum::Display;
use thiserror::Error;
use tracing::{info, warn};
use crate::models::auth::digest::DigestError::{InvalidAlgorithm, InvalidBooleanValue, InvalidCharset, InvalidHeaderSyntax, MissingRequired};

#[derive(Args, Default, Clone, Debug, Serialize, Deserialize)]
pub struct Digest {
    // Secrets part
    pub username: String,
    pub password: String,

    // Header part
    pub domains: String,
    /// Authorization realm (i.e. hostname, serial number...)
    pub realm: String,
    /// Server nonce
    pub nonce: String,
    /// Server opaque string
    pub opaque: String,
    /// True if the server nonce expired.
    /// This is sent in response to an auth attempt with an older digest.
    /// The response should contain a new WWW-Authenticate header.
    pub stale: bool,
    /// Hashing algo
    pub algorithm: DigestAlgorithm,
    /// Digest algorithm variant
    pub qop: DigestQop,
    /// Flag that the server supports user-hashes
    pub user_hash: bool,
    /// Server-supported charset
    pub charset: DigestCharset,

    #[serde(skip, default)]
    pub nc: u32
}

#[derive(Debug, Default, Clone, ValueEnum, Display, Serialize, Deserialize)]
pub enum DigestAlgorithm {
    #[default]
    #[strum(to_string = "MD5")]
    MD5,
    #[strum(to_string = "MD5-sess")]
    MD5Sess,
    #[strum(to_string = "SHA-256")]
    SHA256,
    #[strum(to_string = "SHA-256-sess")]
    SHA256Sess,
    #[strum(to_string = "SHA-512")]
    SHA512,
    #[strum(to_string = "SHA-512-sess")]
    SHA512Sess,
}

#[derive(Debug, Default, Clone, ValueEnum, Display, Serialize, Deserialize)]
pub enum DigestQop {
    #[default]
    #[strum(to_string = "None")]
    None,
    #[strum(to_string = "auth")]
    Auth,
    #[strum(to_string = "auth-int")]
    AuthInt
}

#[derive(Debug, Default, Clone, ValueEnum, Display, Serialize, Deserialize)]
pub enum DigestCharset {
    #[default]
    ASCII,
    UTF8,
}

impl Digest {
    pub fn update_from_www_authenticate_header(&mut self, headers: &Vec<(String, String)>) {
        let www_authenticate_header = headers.iter().find_map(
            |(header, value)| if header == "www-authenticate" {
                Some(value)
            }
            else {
                None
            }
        );

        if let Some(www_authenticate_header) = www_authenticate_header {
            info!("www-authenticate header found, trying to update digest auth data");

            match extract_www_authenticate_digest_data(www_authenticate_header) {
                Ok((domains, realm, nonce, opaque, stale, algorithm, qop, user_hash, charset)) => {
                    self.domains = domains;
                    self.realm = realm;
                    self.nonce = nonce;
                    self.opaque = opaque;
                    self.stale = stale;
                    self.algorithm = algorithm;
                    self.qop = qop;
                    self.user_hash = user_hash;
                    self.charset = charset;
                    info!("Digest auth data updated");
                },
                Err(error) => {
                    warn!("{}", error);
                    info!("Leaving digest auth data unupdated");
                }
            }
        }
    }
}

impl DigestAlgorithm {
    pub fn from_digest_auth_algorithm(algorithm: digest_auth::Algorithm) -> Self {
        match algorithm.algo {
            digest_auth::AlgorithmType::MD5 => match algorithm.sess {
                false => DigestAlgorithm::MD5,
                true => DigestAlgorithm::MD5Sess,
            },
            digest_auth::AlgorithmType::SHA2_256 => match algorithm.sess {
                false => DigestAlgorithm::SHA256,
                true => DigestAlgorithm::SHA256Sess,
            },
            digest_auth::AlgorithmType::SHA2_512_256 => match algorithm.sess {
                false => DigestAlgorithm::SHA512,
                true => DigestAlgorithm::SHA512Sess,
            }
        }
    }

    fn to_digest_auth_algorithm(&self) -> digest_auth::Algorithm {
        match self {
            DigestAlgorithm::MD5 => digest_auth::Algorithm::new(digest_auth::AlgorithmType::MD5, false),
            DigestAlgorithm::MD5Sess => digest_auth::Algorithm::new(digest_auth::AlgorithmType::MD5, true),
            DigestAlgorithm::SHA256 => digest_auth::Algorithm::new(digest_auth::AlgorithmType::SHA2_256, false),
            DigestAlgorithm::SHA256Sess => digest_auth::Algorithm::new(digest_auth::AlgorithmType::SHA2_256, true),
            DigestAlgorithm::SHA512 => digest_auth::Algorithm::new(digest_auth::AlgorithmType::SHA2_512_256, false),
            DigestAlgorithm::SHA512Sess => digest_auth::Algorithm::new(digest_auth::AlgorithmType::SHA2_512_256, true),
        }
    }
}

pub fn previous_digest_algorithm(algorithm: &DigestAlgorithm) -> DigestAlgorithm {
    match algorithm {
        DigestAlgorithm::MD5 => DigestAlgorithm::SHA512Sess,
        DigestAlgorithm::MD5Sess => DigestAlgorithm::MD5,
        DigestAlgorithm::SHA256 => DigestAlgorithm::MD5Sess,
        DigestAlgorithm::SHA256Sess => DigestAlgorithm::SHA256,
        DigestAlgorithm::SHA512 => DigestAlgorithm::SHA256Sess,
        DigestAlgorithm::SHA512Sess => DigestAlgorithm::SHA512,
    }
}

pub fn next_digest_algorithm(algorithm: &DigestAlgorithm) -> DigestAlgorithm {
    match algorithm {
        DigestAlgorithm::MD5 => DigestAlgorithm::MD5Sess,
        DigestAlgorithm::MD5Sess => DigestAlgorithm::SHA256,
        DigestAlgorithm::SHA256 => DigestAlgorithm::SHA256Sess,
        DigestAlgorithm::SHA256Sess => DigestAlgorithm::SHA512,
        DigestAlgorithm::SHA512 => DigestAlgorithm::SHA512Sess,
        DigestAlgorithm::SHA512Sess => DigestAlgorithm::MD5,
    }
}

impl DigestQop {
    pub fn from_digest_auth_qop(qop: digest_auth::Qop) -> Self {
        match qop {
            digest_auth::Qop::AUTH => DigestQop::Auth,
            digest_auth::Qop::AUTH_INT => DigestQop::AuthInt,
        }
    }
    
    fn to_digest_auth_qop(&self) -> Option<Vec<digest_auth::Qop>> {
        match self {
            DigestQop::None => None,
            DigestQop::Auth => Some(vec![digest_auth::Qop::AUTH]),
            DigestQop::AuthInt => Some(vec![digest_auth::Qop::AUTH_INT]),
        }
    }
}

pub fn previous_digest_qop(qop: &DigestQop) -> DigestQop {
    match qop {
        DigestQop::None => DigestQop::AuthInt,
        DigestQop::Auth => DigestQop::None,
        DigestQop::AuthInt => DigestQop::Auth
    }
}

pub fn next_digest_qop(qop: &DigestQop) -> DigestQop {
    match qop {
        DigestQop::None => DigestQop::Auth,
        DigestQop::Auth => DigestQop::AuthInt,
        DigestQop::AuthInt => DigestQop::None
    }
}

impl DigestCharset {
    fn from_digest_charset(charset: digest_auth::Charset) -> Self {
        match charset {
            digest_auth::Charset::ASCII => DigestCharset::ASCII,
            digest_auth::Charset::UTF8 => DigestCharset::UTF8
        }
    }

    fn to_digest_auth_charset(&self) -> digest_auth::Charset {
        match self {
            DigestCharset::ASCII => digest_auth::Charset::ASCII,
            DigestCharset::UTF8 => digest_auth::Charset::UTF8,
        }
    }
}

pub fn toggle_digest_charset(charset: &DigestCharset) -> DigestCharset {
    match charset {
        DigestCharset::ASCII => DigestCharset::UTF8,
        DigestCharset::UTF8 => DigestCharset::ASCII,
    }
}

pub fn digest_to_authorization_header(
    username: &str,
    password: &str,
    url_path: &str,
    domains: String,
    realm: String,
    nonce: String,
    opaque: String,
    stale: bool,
    algorithm: &DigestAlgorithm,
    qop: &DigestQop,
    user_hash: bool,
    charset: &DigestCharset,
    nc: u32
) -> String {
    let context = AuthContext::new(username, password, url_path);

    let mut www_authenticate_header = WwwAuthenticateHeader {
        domain: match domains.is_empty() {
            true => None,
            false => {
                let domains: Vec<&str> = domains.split(' ').collect();
                Some(domains.iter().map(|x| x.trim().to_string()).collect())
            }
        },
        realm,
        nonce,
        opaque: match opaque.is_empty() {
            true => None,
            false => Some(opaque),
        },
        stale,
        algorithm: algorithm.to_digest_auth_algorithm(),
        qop: qop.to_digest_auth_qop(),
        userhash: user_hash,
        charset: charset.to_digest_auth_charset(),
        nc,
    };

    let answer = www_authenticate_header.respond(&context).unwrap().to_header_string();

    answer
}

#[derive(Error, Debug)]
pub enum DigestError {
    #[error("Invalid header syntax: {0}")]
    InvalidHeaderSyntax(String),

    #[error("Missing required {0}")]
    MissingRequired(&'static str),

    #[error("Invalid algorithm: {0}")]
    InvalidAlgorithm(String),

    #[error("Invalid boolean for {0}: {1}")]
    InvalidBooleanValue(&'static str, String),

    #[error("Invalid charset: {0}")]
    InvalidCharset(String),
}

pub fn extract_www_authenticate_digest_data(www_authenticate_header: &str) -> Result<(String, String, String, String, bool, DigestAlgorithm, DigestQop, bool, DigestCharset), DigestError> {
    let mut prompt_kv = parse_header_map(www_authenticate_header)?;
    let domains = prompt_kv.remove("domain").unwrap_or_default();
    let realm = match prompt_kv.remove("realm") {
        Some(v) => v,
        None => return Err(MissingRequired("realm"))
    };
    let nonce = match prompt_kv.remove("nonce") {
        Some(v) => v,
        None => return Err(MissingRequired("nonce"))

    };
    let opaque = prompt_kv.remove("opaque").unwrap_or_default();
    let stale = match prompt_kv.get("stale") {
        Some(v) => match v.to_ascii_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => return Err(InvalidBooleanValue("stale", v.to_string())),
        },
        None => false,
    };
    let algorithm = match prompt_kv.get("algorithm") {
        Some(a) => match digest_auth::Algorithm::from_str(a.as_str()) {
            Ok(algorithm) => DigestAlgorithm::from_digest_auth_algorithm(algorithm),
            Err(_) => return Err(InvalidAlgorithm(a.to_string()))
        },
        _ => DigestAlgorithm::default(),
    };
    let qop = match prompt_kv.get("qop") {
        Some(domains) => {
            let domains: Vec<&str> = domains.split(',').collect();

            if domains.is_empty() {
                DigestQop::None
            } else if domains.contains(&"auth-int") {
                DigestQop::AuthInt
            } else if domains.contains(&"auth") {
                DigestQop::Auth
            } else {
                return Err(MissingRequired("QOP"))
            }
        },
        None => DigestQop::None
    };
    let user_hash = match prompt_kv.get("userhash") {
        Some(v) => match v.to_ascii_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => return Err(InvalidBooleanValue("userhash", v.to_string())),
        },
        None => false,
    };
    let charset = match prompt_kv.get("charset") {
        Some(v) => match digest_auth::Charset::from_str(v) {
            Ok(charset) => DigestCharset::from_digest_charset(charset),
            Err(_) => return Err(InvalidCharset(v.to_string()))
        },
        None => DigestCharset::ASCII,
    };

    Ok((domains, realm, nonce, opaque, stale, algorithm, qop, user_hash, charset))
}

/// Borrowed from the digest-auth crate
fn parse_header_map(input: &str) -> Result<HashMap<String, String>, DigestError> {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum ParserState {
        P_WHITE,
        P_NAME(usize),
        P_VALUE_BEGIN,
        P_VALUE_QUOTED,
        P_VALUE_QUOTED_NEXTLITERAL,
        P_VALUE_PLAIN,
    }

    let mut input = input.trim();

    // Remove leading "Digest"
    if input.starts_with("Digest") {
        input = &input["Digest".len()..];
    }

    let mut state = ParserState::P_WHITE;

    let mut parsed = HashMap::<String, String>::new();
    let mut current_token = None;
    let mut current_value = String::new();

    for (char_n, c) in input.chars().enumerate() {
        match state {
            ParserState::P_WHITE => {
                if c.is_alphabetic() {
                    state = ParserState::P_NAME(char_n);
                }
            }
            ParserState::P_NAME(name_start) => {
                if c == '=' {
                    current_token = Some(&input[name_start..char_n]);
                    state = ParserState::P_VALUE_BEGIN;
                }
            }
            ParserState::P_VALUE_BEGIN => {
                current_value.clear();
                state = match c {
                    '"' => ParserState::P_VALUE_QUOTED,
                    _ => {
                        current_value.push(c);
                        ParserState::P_VALUE_PLAIN
                    }
                };
            }
            ParserState::P_VALUE_QUOTED => {
                match c {
                    '"' => {
                        parsed.insert(current_token.unwrap().to_string(), current_value.clone());

                        current_token = None;
                        current_value.clear();

                        state = ParserState::P_WHITE;
                    }
                    '\\' => {
                        state = ParserState::P_VALUE_QUOTED_NEXTLITERAL;
                    }
                    _ => {
                        current_value.push(c);
                    }
                };
            }
            ParserState::P_VALUE_PLAIN => {
                if c == ',' || c.is_ascii_whitespace() {
                    parsed.insert(current_token.unwrap().to_string(), current_value.clone());

                    current_token = None;
                    current_value.clear();

                    state = ParserState::P_WHITE;
                } else {
                    current_value.push(c);
                }
            }
            ParserState::P_VALUE_QUOTED_NEXTLITERAL => {
                current_value.push(c);
                state = ParserState::P_VALUE_QUOTED
            }
        }
    }

    match state {
        ParserState::P_VALUE_PLAIN => {
            parsed.insert(current_token.unwrap().to_string(), current_value); // consume the value here
        }
        ParserState::P_WHITE => {}
        _ => return Err(InvalidHeaderSyntax(input.to_string())),
    }

    Ok(parsed)
}