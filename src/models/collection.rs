use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::RwLock;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use serde::{Deserialize, Serialize};
use strum::Display;
use tui_tree_widget::TreeItem;
use rayon::prelude::*;
use crate::app::files::theme::THEME;
use crate::models::request::Request;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub requests: Vec<Arc<RwLock<Request>>>,

    #[serde(skip)]
    pub path: PathBuf,

    #[serde(skip)]
    pub file_format: CollectionFileFormat
}

#[derive(Debug, Default, Copy, Clone, Display, Serialize, Deserialize)]
pub enum CollectionFileFormat {
    #[default]
    #[serde(alias="json", alias="JSON")]
    #[strum(to_string = "json")]
    Json,
    #[serde(alias="yaml", alias="YAML")]
    #[strum(to_string = "yaml")]
    Yaml
}

impl Collection {
    pub fn to_tree_item<'a>(&self, identifier: usize) -> TreeItem<'a, usize> {
        let name = self.name.clone();

        let line = Line::from(vec![
            Span::raw(name).fg(THEME.read().ui.font_color),
            Span::from(format!(" ({})", self.requests.len()))
        ]);

        let items: Vec<TreeItem<usize>> = self.requests
            .par_iter()
            .enumerate()
            .map(|(request_index, request)| {
                request.read().to_tree_item(request_index)
            })
            .collect();

        TreeItem::new(identifier, line, items).unwrap()
    }
}