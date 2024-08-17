use lazy_static::lazy_static;
use ratatui::prelude::{Line, Modifier, Span};
use ratatui::style::Stylize;
use serde::{Deserialize, Serialize};
use tracing::trace;
use tui_tree_widget::TreeItem;
use rayon::prelude::*;

use crate::app::app::App;
use crate::models::auth::Auth;
use crate::models::body::ContentType;
use crate::models::method::Method;
use crate::models::response::RequestResponse;
use crate::models::scripts::RequestScripts;
use crate::models::settings::RequestSettings;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub url: String,
    pub method: Method,
    pub params: Vec<KeyValue>,
    pub headers: Vec<KeyValue>,
    pub body: ContentType,
    pub auth: Auth,
    pub scripts: RequestScripts,
    pub settings: RequestSettings,

    #[serde(skip)]
    pub response: RequestResponse,

    #[serde(skip)]
    pub is_pending: bool
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KeyValue {
    pub enabled: bool,
    pub data: (String, String),
}

impl App<'_> {
    pub fn key_value_vec_to_tuple_vec(&self, key_value: &Vec<KeyValue>) -> Vec<(String, String)> {
        key_value
            .par_iter()
            .filter_map(|param| {
                if param.enabled {
                    let key = self.replace_env_keys_by_value(&param.data.0);
                    let value = self.replace_env_keys_by_value(&param.data.1);

                    Some((key, value))
                } else {
                    None
                }
            })
            .collect()
    }
}

lazy_static! {
    pub static ref DEFAULT_HEADERS: Vec<KeyValue> = vec![
        KeyValue {
            enabled: true,
            data: (String::from("cache-control"), String::from("no-cache")),
        },
        KeyValue {
            enabled: true,
            data: (String::from("user-agent"), format!("ATAC/v{}", env!("CARGO_PKG_VERSION"))),
        },
        KeyValue {
            enabled: true,
            data: (String::from("accept"), String::from("*/*")),
        },
        KeyValue {
            enabled: true,
            data: (String::from("accept-encoding"), String::from("gzip, deflate, br")),
        },
        KeyValue {
            enabled: true,
            data: (String::from("connection"), String::from("keep-alive")),
        },
    ];
}

impl Request {
    pub fn to_tree_item<'a>(&self, identifier: usize) -> TreeItem<'a, usize> {
        let mut line_elements: Vec<Span> = vec![];

        let prefix = Span::from(self.method.to_string())
            .style(Modifier::BOLD)
            .bg(self.method.get_color());

        line_elements.push(prefix);

        if self.is_pending {
            line_elements.push(Span::raw(" 🕛"));
        }
        else {
            line_elements.push(Span::raw(" "));
        }

        let text = Span::from(self.name.clone());

        line_elements.push(text);

        let line = Line::from(line_elements);

        TreeItem::new_leaf(identifier, line)
    }

    pub fn url_with_params_to_string(&self) -> String {
        let mut base_url = self.url.to_string();

        if !self.params.is_empty() {
            let mut enabled_params: Vec<String> = vec![];

            for (index, param) in self.params.iter().enumerate() {
                if !param.enabled {
                    continue;
                }

                let mut new_param = format!("{}={}", param.data.0, param.data.1);

                if index != self.params.len() - 1 {
                    new_param += "&";
                }

                enabled_params.push(new_param);
            }

            if !enabled_params.is_empty() {
                base_url += "?";

                for enabled_param in enabled_params {
                    base_url += &enabled_param;
                }
            }
        }

        return base_url;
    }

    pub fn find_and_delete_header(&mut self, input_header: &str) {
        trace!("Trying to find and delete header \"{}\"", input_header);
        let index = self.headers
            .par_iter()
            .position_any(|header| header.data.0 == input_header);

        match index {
            None => {
                trace!("Not found")
            }
            Some(index) => {
                trace!("Found, deleting");
                self.headers.remove(index);
            }
        }
    }

    pub fn modify_or_create_header(&mut self, input_header: &str, value: &str) {
        trace!("Trying to modify or create header \"{}\"", input_header);

        let mut was_header_found = false;

        for header in &mut self.headers {
            if header.data.0.to_lowercase() == input_header.to_lowercase() {
                trace!("Found, modifying");

                header.data.1 = value.to_string();
                was_header_found = true;
            }
        }

        if !was_header_found {
            trace!("Not found, creating");

            self.headers.push(KeyValue {
                enabled: true,
                data: (input_header.to_string(), value.to_string()),
            })
        }
    }
}