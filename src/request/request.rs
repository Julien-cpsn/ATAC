use ratatui::prelude::{Line, Modifier, Span};
use ratatui::style::Stylize;
use ratatui::widgets::ListItem;
use reqwest::Method;
use crate::request::body::ContentType;
use crate::request::method::get_method_bg;

#[derive(Default, Clone)]
pub struct Request<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub method: Method,
    pub body: ContentType,
    pub result: RequestResult
}

#[derive(Default, Clone)]
pub struct RequestResult {
    pub body: Option<String>,
    pub cookies: Option<String>,
    pub headers: Option<String>
}

impl Request<'_> {
    pub fn to_list_item(&self) -> ListItem {
        let prefix = Span::from(self.method.to_string())
            .style(Modifier::BOLD)
            .bg(get_method_bg(&self.method));

        let text = Span::from(self.name);

        let line = Line::from(vec![
            prefix,
            Span::from(" "),
            text,
        ]);

        ListItem::new(line)
    }
}