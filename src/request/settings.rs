use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestSettings {
    pub use_config_proxy: bool,
    pub allow_redirects: bool,
    pub store_received_cookies: bool,
    pub pretty_print_response_content: bool,
    pub accept_invalid_certs: bool,
}

impl Default for RequestSettings {
    fn default() -> Self {
        RequestSettings {
            use_config_proxy: true,
            allow_redirects: true,
            store_received_cookies: true,
            pretty_print_response_content: true,
            accept_invalid_certs: false,
        }
    }
}

impl RequestSettings {
    pub fn to_vec(&self) -> Vec<(String, bool)> {
        vec![
            (String::from("Use config proxy"), self.use_config_proxy),
            (String::from("Allow redirects"), self.allow_redirects),
            (String::from("Store received cookies"), self.store_received_cookies),
            (String::from("Pretty print response content"), self.pretty_print_response_content),
            (String::from("Accept invalid certs"), self.accept_invalid_certs),
        ]
    }

    pub fn update_from_vec(&mut self, vec: &Vec<(String, bool)>) {
        for (setting_name, setting_value) in vec {
            match setting_name.as_str() {
                "Use config proxy" => self.use_config_proxy = *setting_value,
                "Allow redirects" => self.allow_redirects = *setting_value,
                "Store received cookies" => self.store_received_cookies = *setting_value,
                "Pretty print response content" => self.pretty_print_response_content = *setting_value,
                "Accept invalid certs" => self.accept_invalid_certs = *setting_value,

                _ => {}
            }
        }
    }
}
