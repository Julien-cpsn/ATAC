use clap::Error;
use clap::error::ErrorKind::InvalidValue;

const ELEMENT_NAME_REGEX: &str = "[a-zA-Z0-9-_ ]+";

pub fn collection_validator(arg: &str) -> Result<String, Error> {
    let regex = regex::Regex::new(&format!(r#"^(?<collection>{ELEMENT_NAME_REGEX})$"#)).unwrap();
    match regex.captures(arg) {
        None => Err(Error::new(InvalidValue)),
        Some(capture) => Ok(capture["collection"].to_string())
    }
}

pub fn collection_and_request_validator(arg: &str) -> Result<(String, String), Error> {
    let regex = regex::Regex::new(&format!(r#"^(?<collection>{ELEMENT_NAME_REGEX})/(?<request>{ELEMENT_NAME_REGEX})$"#)).unwrap();
    match regex.captures(arg) {
        None => Err(Error::new(InvalidValue)),
        Some(capture) => Ok((capture["collection"].to_string(), capture["request"].to_string()))
    }
}