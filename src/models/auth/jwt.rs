use clap::{Args, ValueEnum};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fs;
use strum::Display;
use thiserror::Error;
use tracing::{debug, trace};

#[derive(Args, Default, Clone, Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub algorithm: JwtAlgorithm,
    pub secret_type: JwtSecretType,
    pub secret: String,
    pub payload: String,
}

#[derive(Debug, Default, Clone, ValueEnum, Display, Serialize, Deserialize)]
pub enum JwtAlgorithm {
    #[default]
    #[strum(to_string = "HS256")]
    /// HMAC using SHA-256
    HS256,
    #[strum(to_string = "HS384")]
    /// HMAC using SHA-384
    HS384,
    #[strum(to_string = "HS512")]
    /// HMAC using SHA-512
    HS512,
    #[strum(to_string = "ES256")]
    /// ECDSA using SHA-256
    ES256,
    #[strum(to_string = "ES384")]
    /// ECDSA using SHA-384
    ES384,
    #[strum(to_string = "RS256")]
    /// RSASSA-PKCS1-v1_5 using SHA-256
    RS256,
    #[strum(to_string = "RS384")]
    /// RSASSA-PKCS1-v1_5 using SHA-384
    RS384,
    #[strum(to_string = "RS512")]
    /// RSASSA-PKCS1-v1_5 using SHA-512
    RS512,
    #[strum(to_string = "PS256")]
    /// RSASSA-PSS using SHA-256
    PS256,
    #[strum(to_string = "PS384")]
    /// RSASSA-PSS using SHA-384
    PS384,
    #[strum(to_string = "PS512")]
    /// RSASSA-PSS using SHA-512
    PS512,
    #[strum(to_string = "EdDSA")]
    /// Edwards-curve Digital Signature Algorithm (EdDSA)
    EdDSA,
}


#[derive(Debug, Default, Clone, ValueEnum, Display, Serialize, Deserialize)]
pub enum JwtSecretType {
    #[default]
    #[strum(serialize = "Text")]
    #[serde(alias = "text", alias = "TEXT")]
    Text,
    #[strum(serialize = "Base64")]
    #[serde(alias = "base64", alias = "BASE64")]
    Base64,
    #[strum(serialize = "URL safe Base64")]
    #[serde(alias = "url_safe_base64", alias = "URL_SAFE_BASE64")]
    UrlSafeBase64,
    #[strum(serialize = "PEM")]
    #[serde(alias = "pem", alias = "PEM")]
    Pem,
    #[strum(serialize = "DER")]
    #[serde(alias = "der", alias = "DER")]
    Der
}

impl JwtAlgorithm {
    pub fn to_jsonwebtoken_algorithm(&self) -> jsonwebtoken::Algorithm {
        match self {
            JwtAlgorithm::HS256 => jsonwebtoken::Algorithm::HS256,
            JwtAlgorithm::HS384 => jsonwebtoken::Algorithm::HS384,
            JwtAlgorithm::HS512 => jsonwebtoken::Algorithm::HS512,
            JwtAlgorithm::ES256 => jsonwebtoken::Algorithm::ES256,
            JwtAlgorithm::ES384 => jsonwebtoken::Algorithm::ES384,
            JwtAlgorithm::RS256 => jsonwebtoken::Algorithm::RS256,
            JwtAlgorithm::RS384 => jsonwebtoken::Algorithm::RS384,
            JwtAlgorithm::RS512 => jsonwebtoken::Algorithm::RS512,
            JwtAlgorithm::PS256 => jsonwebtoken::Algorithm::PS256,
            JwtAlgorithm::PS384 => jsonwebtoken::Algorithm::PS384,
            JwtAlgorithm::PS512 => jsonwebtoken::Algorithm::PS512,
            JwtAlgorithm::EdDSA => jsonwebtoken::Algorithm::EdDSA,
        }
    }

    pub fn get_helper(&self) -> &str {
        match self {
            JwtAlgorithm::HS256 | JwtAlgorithm::HS384 | JwtAlgorithm::HS512 => "HMAC",
            JwtAlgorithm::ES256 | JwtAlgorithm::ES384 => "EC key file path",
            JwtAlgorithm::RS256 | JwtAlgorithm::RS384 | JwtAlgorithm::RS512 |
            JwtAlgorithm::PS256 | JwtAlgorithm::PS384 | JwtAlgorithm::PS512 => "RSA key file path",
            JwtAlgorithm::EdDSA => "ED key file path"
        }
    }

    pub fn default_secret_type(&self) -> JwtSecretType {
        match self {
            JwtAlgorithm::HS256 | JwtAlgorithm::HS384 | JwtAlgorithm::HS512 => JwtSecretType::Text,
            JwtAlgorithm::ES256 | JwtAlgorithm::ES384 |
            JwtAlgorithm::RS256 | JwtAlgorithm::RS384 | JwtAlgorithm::RS512 |
            JwtAlgorithm::PS256 | JwtAlgorithm::PS384 | JwtAlgorithm::PS512 |
            JwtAlgorithm::EdDSA => JwtSecretType::Pem,
        }
    }
}

pub fn previous_jwt_algorithm(algorithm: &JwtAlgorithm) -> JwtAlgorithm {
    match algorithm {
        JwtAlgorithm::HS256 => JwtAlgorithm::EdDSA,
        JwtAlgorithm::HS384 => JwtAlgorithm::HS256,
        JwtAlgorithm::HS512 => JwtAlgorithm::HS384,
        JwtAlgorithm::ES256 => JwtAlgorithm::HS512,
        JwtAlgorithm::ES384 => JwtAlgorithm::ES256,
        JwtAlgorithm::RS256 => JwtAlgorithm::ES384,
        JwtAlgorithm::RS384 => JwtAlgorithm::RS256,
        JwtAlgorithm::RS512 => JwtAlgorithm::RS384,
        JwtAlgorithm::PS256 => JwtAlgorithm::RS512,
        JwtAlgorithm::PS384 => JwtAlgorithm::PS256,
        JwtAlgorithm::PS512 => JwtAlgorithm::PS384,
        JwtAlgorithm::EdDSA => JwtAlgorithm::PS512,
    }
}

pub fn next_jwt_algorithm(algorithm: &JwtAlgorithm) -> JwtAlgorithm {
    match algorithm {
        JwtAlgorithm::HS256 => JwtAlgorithm::HS384,
        JwtAlgorithm::HS384 => JwtAlgorithm::HS512,
        JwtAlgorithm::HS512 => JwtAlgorithm::ES256,
        JwtAlgorithm::ES256 => JwtAlgorithm::ES384,
        JwtAlgorithm::ES384 => JwtAlgorithm::RS256,
        JwtAlgorithm::RS256 => JwtAlgorithm::RS384,
        JwtAlgorithm::RS384 => JwtAlgorithm::RS512,
        JwtAlgorithm::RS512 => JwtAlgorithm::PS256,
        JwtAlgorithm::PS256 => JwtAlgorithm::PS384,
        JwtAlgorithm::PS384 => JwtAlgorithm::PS512,
        JwtAlgorithm::PS512 => JwtAlgorithm::EdDSA,
        JwtAlgorithm::EdDSA => JwtAlgorithm::HS256,
    }
}

pub fn previous_jwt_secret_type(secret_type: &JwtSecretType) -> JwtSecretType {
    match secret_type {
        JwtSecretType::Text => JwtSecretType::UrlSafeBase64,
        JwtSecretType::Base64 => JwtSecretType::Text,
        JwtSecretType::UrlSafeBase64 => JwtSecretType::Base64,
        JwtSecretType::Pem => JwtSecretType::Der,
        JwtSecretType::Der => JwtSecretType::Pem,
    }
}

pub fn next_jwt_secret_type(secret_type: &JwtSecretType) -> JwtSecretType {
    match secret_type {
        JwtSecretType::Text => JwtSecretType::Base64,
        JwtSecretType::Base64 => JwtSecretType::UrlSafeBase64,
        JwtSecretType::UrlSafeBase64 => JwtSecretType::Text,
        JwtSecretType::Pem => JwtSecretType::Der,
        JwtSecretType::Der => JwtSecretType::Pem,
    }
}

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Invalid JSON payload")]
    InvalidJsonPayload,

    #[error("Could not decode Base64 secret")]
    Base64DecodeError,

    #[error("Could not open secret file")]
    CouldNotOpenSecretFile,

    #[error("Invalid key format")]
    InvalidKeyFormat,

    #[error("JWT encoding failed")]
    EncodingFailed,
}

pub fn jwt_do_jaat(algorithm: &JwtAlgorithm, secret_type: &JwtSecretType, secret: String, payload: String) -> Result<String, JwtError> {
    debug!("Doing JAAT");

    let claims: Map<String, Value> = match payload.is_empty() {
        true => {
            trace!("Empty JWT claims");

            Map::new()
        },
        false => {
            trace!("Generating JWT claims");

            match serde_json::from_str(&payload) {
                Ok(claims) => claims,
                Err(_) => return Err(JwtError::InvalidJsonPayload)
            }
        }
    };
    let header = Header::new(algorithm.to_jsonwebtoken_algorithm());

    let encoding_key = match algorithm {
        JwtAlgorithm::HS256 | JwtAlgorithm::HS384 | JwtAlgorithm::HS512 => match secret_type {
            JwtSecretType::Text => {
                trace!("Loading JWT secret");

                EncodingKey::from_secret(secret.as_bytes())
            },
            JwtSecretType::Base64 => {
                trace!("Loading JWT Base64 secret");

                EncodingKey::from_base64_secret(secret.as_str()).map_err(|_| JwtError::Base64DecodeError)?
            },
            JwtSecretType::UrlSafeBase64 => {
                trace!("Loading JWT URL safe Base64 secret");

                EncodingKey::from_urlsafe_base64_secret(secret.as_str()).map_err(|_| JwtError::Base64DecodeError)?
            },
            _ => unreachable!()
        }
        JwtAlgorithm::ES256 | JwtAlgorithm::ES384 => {
            let bytes = fs::read(&secret).map_err(|_| JwtError::CouldNotOpenSecretFile)?;

            trace!("Loading JWT EC key from {} file", secret_type);

            match secret_type {
                JwtSecretType::Pem => EncodingKey::from_ec_pem(&bytes).map_err(|_| JwtError::InvalidKeyFormat)?,
                JwtSecretType::Der => EncodingKey::from_ec_der(&bytes),
                _ => unreachable!()
            }
        },
        JwtAlgorithm::RS256 | JwtAlgorithm::RS384 | JwtAlgorithm::RS512 |
        JwtAlgorithm::PS256 | JwtAlgorithm::PS384 | JwtAlgorithm::PS512 => {
            let bytes = fs::read(&secret).map_err(|_| JwtError::CouldNotOpenSecretFile)?;

            trace!("Loading JWT RSA key from {} file", secret_type);

            match secret_type {
                JwtSecretType::Pem => EncodingKey::from_rsa_pem(&bytes).map_err(|_| JwtError::InvalidKeyFormat)?,
                JwtSecretType::Der => EncodingKey::from_rsa_der(&bytes),
                _ => unreachable!()
            }
        },
        JwtAlgorithm::EdDSA => {
            let bytes = fs::read(&secret).map_err(|_| JwtError::CouldNotOpenSecretFile)?;

            trace!("Loading JWT ED key from {} file", secret_type);

            match secret_type {
                JwtSecretType::Pem => EncodingKey::from_ed_pem(&bytes).map_err(|_| JwtError::InvalidKeyFormat)?,
                JwtSecretType::Der => EncodingKey::from_ed_der(&bytes),
                _ => unreachable!()
            }
        }
    };

    trace!("Encoding JWT header");

    let encoded_header = match encode(&header, &claims, &encoding_key) {
        Ok(encoded_header) => encoded_header,
        Err(_) => return Err(JwtError::EncodingFailed)
    };

    debug!("JAAT done");

    Ok(encoded_header)
}