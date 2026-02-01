use anyhow::anyhow;
use lazy_static::lazy_static;
use ratatui::prelude::{Line, Modifier, Span};
use ratatui::style::{Color, Stylize};
use tracing::trace;
use tui_tree_widget::TreeItem;
use rayon::prelude::*;
use regex::Regex;
use serde::Serialize;
use serde_versioning::Deserialize;
use tokio_util::sync::CancellationToken;

use crate::app::app::App;
use crate::app::files::config::SKIP_SAVE_REQUESTS_RESPONSE;
use crate::app::files::theme::THEME;
use crate::models::auth::auth::Auth;
use crate::models::legacy::request::RequestV0_20_2;
use crate::models::protocol::http::http::HttpRequest;
use crate::models::protocol::protocol::Protocol;
use crate::models::protocol::protocol::ProtocolTypeError::{NotAWsRequest, NotAnHttpRequest};
use crate::models::protocol::ws::ws::WsRequest;
use crate::models::response::RequestResponse;
use crate::models::scripts::RequestScripts;
use crate::models::settings::RequestSettings;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[versioning(previous_version = RequestV0_20_2)]
pub struct Request {
    pub name: String,
    pub url: String,
    pub params: Vec<KeyValue>,
    pub headers: Vec<KeyValue>,
    pub auth: Auth,
    pub scripts: RequestScripts,
    pub settings: RequestSettings,

    pub protocol: Protocol,

    #[serde(skip_serializing_if = "should_skip_requests_response", default = "RequestResponse::default")]
    pub response: RequestResponse,

    #[serde(skip)]
    pub console_output: ConsoleOutput,

    #[serde(skip)]
    pub is_pending: bool,

    #[serde(skip)]
    pub cancellation_token: CancellationToken,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KeyValue {
    pub enabled: bool,
    pub data: (String, String),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleOutput {
    pub pre_request_output: Option<String>,
    pub post_request_output: Option<String>,
}

fn should_skip_requests_response(_: &RequestResponse) -> bool {
    *SKIP_SAVE_REQUESTS_RESPONSE.get().unwrap_or(&true)
}

impl App<'_> {
    pub fn key_value_vec_to_tuple_vec(&self, key_value: &Vec<KeyValue>) -> Vec<(String, String)> {
        key_value
            .iter()
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
    pub fn get_http_request(&self) -> anyhow::Result<&HttpRequest> {
        match &self.protocol {
            Protocol::HttpRequest(request) => Ok(request),
            Protocol::WsRequest(_) => Err(anyhow!(NotAnHttpRequest))
        }
    }

    pub fn get_http_request_mut(&mut self) -> anyhow::Result<&mut HttpRequest> {
        match &mut self.protocol {
            Protocol::HttpRequest(request) => Ok(request),
            Protocol::WsRequest(_) => Err(anyhow!(NotAnHttpRequest))
        }
    }

    pub fn get_ws_request(&self) -> anyhow::Result<&WsRequest> {
        match &self.protocol {
            Protocol::HttpRequest(_) => Err(anyhow!(NotAWsRequest)),
            Protocol::WsRequest(request) => Ok(request)
        }
    }

    pub fn get_ws_request_mut(&mut self) -> anyhow::Result<&mut WsRequest> {
        match &mut self.protocol {
            Protocol::HttpRequest(_) => Err(anyhow!(NotAWsRequest)),
            Protocol::WsRequest(request) => Ok(request)
        }
    }

    pub fn to_tree_item<'a>(&self, identifier: usize) -> TreeItem<'a, usize> {
        let mut line_elements: Vec<Span> = vec![];

        let prefix = match &self.protocol {
            Protocol::HttpRequest(http_request) => Span::from(http_request.method.to_string())
                .style(Modifier::BOLD)
                .fg(Color::White)
                .bg(http_request.method.get_color()),
            Protocol::WsRequest(ws_request) => {
                let color = match ws_request.is_connected {
                    true => THEME.read().websocket.connection_status.connected,
                    false => THEME.read().websocket.connection_status.disconnected,
                };

                Span::from("WS")
                    .style(Modifier::BOLD)
                    .fg(Color::White)
                    .bg(color)
            }
        };

        line_elements.push(prefix);

        if self.is_pending {
            line_elements.push(Span::raw(" 🕛"));
        }
        else {
            line_elements.push(Span::raw(" "));
        }

        let text = Span::from(self.name.clone()).fg(THEME.read().ui.font_color);

        line_elements.push(text);

        let line = Line::from(line_elements);

        TreeItem::new_leaf(identifier, line)
    }

    pub fn update_url_and_params(&mut self, url: String) {
        let url_parts = url.trim().split_once("?");

        let final_url: String;
        let query_params: &str;

        if let Some((url, found_query_params)) = url_parts {
            final_url = url.to_string();
            query_params = found_query_params;
        } else {
            final_url = url;
            query_params = "";
        }

        let mut found_params = vec![];

        let path_params_pattern = Regex::new(r"(\{+[\w-]+}+)").unwrap();
        for (_, [path_param]) in path_params_pattern.captures_iter(&final_url).map(|c| c.extract()) {
            if path_param.starts_with("{{") || path_param.ends_with("}}") {
                continue;
            }
            
            found_params.push((path_param.to_string(), None));
        }

        let query_params_pattern = Regex::new(r"(&?([^=]+)=([^&]+))").unwrap();
        for (_, [_, param_name, value]) in query_params_pattern.captures_iter(query_params).map(|c| c.extract()) {
            found_params.push((param_name.to_string(), Some(value.to_string())));
        }

        self.params.retain(|param|
            found_params.iter().any(|found| found.0 == param.data.0)
        );

        for found_param in found_params {
            let param = self.params.iter_mut().find(|param| param.data.0 == found_param.0);

            if let Some(param) = param {
                if let Some(value)  = found_param.1 {
                    param.data.1 = value;
                }
            }
            else {
                let value = found_param.1.unwrap_or_else(|| String::from("value"));
                self.params.push(KeyValue {
                    enabled: true,
                    data: (found_param.0, value),
                });
            }
        }

        self.url = final_url;
    }


    pub fn url_with_params_to_string(&self) -> String {
        let mut base_url = self.url.to_string();

        if !self.params.is_empty() {
            let mut enabled_params: Vec<String> = vec![];

            for param in self.params.iter() {
                if !param.enabled || (param.data.0.starts_with("{") && param.data.0.ends_with("}")) {
                    continue;
                }
                
                enabled_params.push(format!("{}={}", param.data.0, param.data.1));
            }

            if !enabled_params.is_empty() {
                base_url += "?";

                for (index, enabled_param) in enabled_params.iter().enumerate() {
                    base_url += &enabled_param;

                    if index != enabled_params.len() - 1 {
                        base_url += "&";
                    }
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