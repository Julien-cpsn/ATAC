use anyhow::anyhow;
use ratatui::prelude::Stylize;
use thiserror::Error;
use tracing::trace;

use crate::app::business_logic::key_value::KeyValueError::KeyNotFound;
use crate::models::request::KeyValue;

#[derive(Error, Debug)]
pub enum KeyValueError {
    #[error("Key not found")]
    KeyNotFound,
}

pub fn find_key(key_value_array: &Vec<KeyValue>, key: &str) -> anyhow::Result<usize> {
    trace!("Trying to find key \"{}\"", key);

    for (index, key_value) in key_value_array.iter().enumerate() {
        if key_value.data.0 == key {
            trace!("Found");
            return Ok(index);
        }
    }

    trace!("Not found");
    Err(anyhow!(KeyNotFound))
}

pub fn print_key_value_vector(array: &Vec<KeyValue>, prefix: Option<&str>) {
    let prefix = prefix.unwrap_or("");
    
    for key_value in array {
        let text = format!("{prefix}{}: {}", key_value.data.0, key_value.data.1);

        if key_value.enabled {
            println!("{}", text);
        }
        else {
            println!("{}", text.dark_gray());
        }
    }
}