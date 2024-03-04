use lazy_static::lazy_static;
use ratatui::prelude::{Line, Modifier, Span};
use ratatui::style::Stylize;
use serde::{Deserialize, Serialize};
use tui_tree_widget::TreeItem;
use crate::request::auth::{Auth};
use crate::request::body::ContentType;
use crate::request::method::Method;
use crate::request::settings::RequestSettings;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub url: String,
    pub method: Method,
    pub params: Vec<KeyValue>,
    pub headers: Vec<KeyValue>,
    pub body: ContentType,
    pub auth: Auth,
    pub settings: RequestSettings,

    #[serde(skip)]
    pub result: RequestResult,

    #[serde(skip)]
    pub is_pending: bool
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KeyValue {
    pub enabled: bool,
    pub data: (String, String)
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RequestResult {
    pub duration: Option<String>,
    pub status_code: Option<String>,
    pub body: Option<String>,
    pub cookies: Option<String>,
    pub headers: Vec<(String, String)>
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
        let index = self.headers.iter().position(|header| header.data.0 == input_header);

        match index {
            None => {}
            Some(index) => {
                self.headers.remove(index);
            }
        }
    }

    pub fn modify_or_create_header(&mut self, input_header: &str, value: &str) {
        let mut header_was_found = false;

        for header in &mut self.headers {
            if &header.data.0 == input_header {
                header.data.1 = value.to_string();
                header_was_found = true;
            }
        }

        if !header_was_found {
            self.headers.push(KeyValue {
                enabled: true,
                data: (input_header.to_string(), value.to_string()),
            })
        }
    }
}