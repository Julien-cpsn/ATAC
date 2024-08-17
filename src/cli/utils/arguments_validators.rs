use clap::Error;
use clap::error::ErrorKind::InvalidValue;
use crate::models::request::KeyValue;

const ELEMENT_NAME_REGEX: &str = "[a-zA-Z0-9-_ ]+";

#[allow(dead_code)]
pub fn collection_validator(arg: &str) -> Result<String, Error> {
    let regex = regex::Regex::new(&format!(r#"^(?<collection>{ELEMENT_NAME_REGEX})$"#)).unwrap();
    match regex.captures(arg) {
        None => Err(Error::new(InvalidValue)),
        Some(capture) => Ok(capture["collection"].to_string())
    }
}

pub fn collection_slash_request_validator(arg: &str) -> Result<(String, String), Error> {
    let regex = regex::Regex::new(&format!(r#"^(?<collection>{ELEMENT_NAME_REGEX})/(?<request>{ELEMENT_NAME_REGEX})$"#)).unwrap();
    match regex.captures(arg) {
        None => Err(Error::new(InvalidValue)),
        Some(capture) => Ok((capture["collection"].to_string(), capture["request"].to_string()))
    }
}

#[allow(unused)]
pub fn key_value_array_validator(arg: &str) -> Result<KeyValue, Error> {
    dbg!(arg);
    let pair = match arg.split_once(" ") {
        None => return Err(Error::new(InvalidValue)),
        Some(pair) => pair
    };

    Ok(KeyValue {
        enabled: true,
        data: (pair.0.to_string(), pair.1.to_string()),
    })
}