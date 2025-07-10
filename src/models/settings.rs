use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestSettings {
    pub use_config_proxy: Setting,
    pub allow_redirects: Setting,
    pub timeout: Setting,
    pub store_received_cookies: Setting,
    pub pretty_print_response_content: Setting,
    pub accept_invalid_certs: Setting,
    pub accept_invalid_hostnames: Setting
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Setting {
    Bool(bool),
    U32(u32)
}

impl Setting {
    pub fn as_bool(&self) -> bool {
        match self {
            Setting::Bool(bool) => *bool,
            Setting::U32(_) => panic!("Should not happen")
        }
    }

    pub fn as_u32(&self) -> u32 {
        match self {
            Setting::Bool(_) => panic!("Should not happen"),
            Setting::U32(u32) => *u32
        }
    }
}

impl FromStr for Setting {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, String> {
        match bool::from_str(&input.to_lowercase()) {
            Ok(bool) => Ok(Setting::Bool(bool)),
            Err(_) => match u32::from_str(input) {
                Ok(u32) => Ok(Setting::U32(u32)),
                Err(_) => Err(String::from("Value should either be a boolean or a positive int"))
            }
        }
    }
}

impl Display for Setting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Setting::Bool(bool) => bool.to_string(),
            Setting::U32(uint) => uint.to_string()
        };
        write!(f, "{}", str)
    }
}

impl Default for RequestSettings {
    fn default() -> Self {
        RequestSettings {
            use_config_proxy: Setting::Bool(true),
            allow_redirects: Setting::Bool(true),
            timeout: Setting::U32(30000),
            store_received_cookies: Setting::Bool(true),
            pretty_print_response_content: Setting::Bool(true),
            accept_invalid_certs: Setting::Bool(false),
            accept_invalid_hostnames: Setting::Bool(false),
        }
    }
}

impl RequestSettings {
    pub fn to_vec(&self) -> Vec<(String, Setting)> {
        vec![
            (String::from("Use config proxy"), self.use_config_proxy.clone()),
            (String::from("Allow redirects"), self.allow_redirects.clone()),
            (String::from("Timeout (ms)"), self.timeout.clone()),
            (String::from("Store received cookies"), self.store_received_cookies.clone()),
            (String::from("Pretty print response content"), self.pretty_print_response_content.clone()),
            (String::from("Accept invalid certs"), self.accept_invalid_certs.clone()),
            (String::from("Accept invalid hostnames"), self.accept_invalid_hostnames.clone()),
        ]
    }

    pub fn update_from_vec(&mut self, vec: &Vec<(String, Setting)>) {
        for (setting_name, setting_value) in vec {
            match setting_name.as_str() {
                "Use config proxy" => self.use_config_proxy = setting_value.clone(),
                "Allow redirects" => self.allow_redirects = setting_value.clone(),
                "Timeout (ms)" => self.timeout = setting_value.clone(),
                "Store received cookies" => self.store_received_cookies = setting_value.clone(),
                "Pretty print response content" => self.pretty_print_response_content = setting_value.clone(),
                "Accept invalid certs" => self.accept_invalid_certs = setting_value.clone(),
                "Accept invalid hostnames" => self.accept_invalid_hostnames = setting_value.clone(),
                _ => {}
            }
        }
    }
}
