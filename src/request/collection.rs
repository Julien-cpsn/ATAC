use ratatui::text::{Line, Span};
use serde::{Deserialize, Serialize};
use tui_tree_widget::TreeItem;
use crate::request::request::Request;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub requests: Vec<Request>
}

impl Collection {
    pub fn to_tree_item<'a>(&self, identifier: usize) -> TreeItem<'a, usize> {
        let name = self.name.clone();

        let line = Line::from(vec![
            Span::raw(name),
            Span::from(format!(" ({})", self.requests.len()))
        ]);

        let items: Vec<TreeItem<usize>> = self.requests
            .iter()
            .enumerate()
            .map(|(index, request)| request.to_tree_item(index))
            .collect();

        TreeItem::new(identifier, line, items).unwrap()
    }
}