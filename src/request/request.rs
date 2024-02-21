use ratatui::prelude::{Line, Modifier, Span};
use ratatui::style::Stylize;
use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};
use crate::request::auth::{Auth};
use crate::request::body::ContentType;
use crate::request::method::Method;
use crate::utils::stateful_custom_table::Param;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub url: String,
    pub method: Method,
    pub params: Vec<Param>,
    pub body: ContentType,
    pub auth: Auth,
    pub result: RequestResult
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct RequestResult {
    pub status_code: Option<u16>,
    pub body: Option<String>,
    pub cookies: Option<String>,
    pub headers: Option<String>
}

impl Request {
    pub fn to_list_item(&self) -> ListItem {
        let prefix = Span::from(self.method.to_string())
            .style(Modifier::BOLD)
            .bg(self.method.get_color());

        let text = Span::from(&self.name);

        let line = Line::from(vec![
            prefix,
            Span::from(" "),
            text,
        ]);

        ListItem::new(line)
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
}