use clap::Error;
use clap::error::ErrorKind::InvalidValue;
use crate::cli::send::CollectionAndRequestArg;
use crate::cli::send::CollectionAndRequestArg::{CollectionAndRequest, CollectionOnly};

pub fn collection_and_request_validator(arg: &str) -> Result<CollectionAndRequestArg, Error> {
    let regex = regex::Regex::new(r#"^(?<collection>[a-zA-Z0-9_ ]+)(/(?<request>[a-zA-Z0-9_ ]+))?$"#).unwrap();
    match regex.captures(arg) {
        None => Err(Error::new(InvalidValue)),
        Some(capture) => match capture.name("request") {
            None => Ok(
                CollectionOnly(capture["collection"].to_string())
            ),
            Some(request_name) => Ok(
                CollectionAndRequest(capture["collection"].to_string(), request_name.as_str().to_string())
            )
        }
    }
}