extern crate jsonwebtoken as jwt;
use std::collections::HashMap;

use jwt::{Algorithm, encode, EncodingKey, Header};
use serde_json::Value;

pub fn get_signing_key(private_key_string: &str) -> EncodingKey {
    let private_key = private_key_string.as_bytes();
    let signing_key = EncodingKey::from_rsa_pem(private_key).unwrap();
    signing_key
}

pub fn generate_jwt_token(private_key_string: &str, claims: &str, alg: Option<Algorithm>, kid: Option<String>) -> String {
    let encoding_key = get_signing_key(private_key_string);
    let algorithm = match alg {
        Some(alg) => alg,
        None => Algorithm::RS384
    };

    let mut header = Header::new(algorithm);
    header.kid = kid;

    let payload: HashMap<String, Value> = serde_json::from_str(claims).unwrap();

    let token = encode(&header, &payload, &encoding_key).unwrap();
    token
}
