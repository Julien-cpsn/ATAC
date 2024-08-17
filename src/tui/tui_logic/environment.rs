use chrono::Utc;
use parking_lot::RwLock;
use uuid::Uuid;
use std::sync::Arc;

use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use regex::Regex;

use crate::app::app::App;

impl App<'_> {
    pub fn tui_next_environment(&mut self) {
        if self.selected_environment + 1 < self.environments.len() {
            self.selected_environment += 1;
        }
        else {
            self.selected_environment = 0;
        }
    }

    pub fn tui_add_color_to_env_keys(&self, input: &str) -> Line {
        if self.environments.is_empty() || !input.contains('{') {
            return Line::raw(input.to_string());
        }

        let mut spans: Vec<Span> = vec![];

        let regex = Regex::new(r"\{\{(\w+)}}").unwrap();
        let mut tmp_index: usize = 0;

        let local_env = self.get_selected_env_as_local();

        if let Some(local_env) = local_env {
            let env = local_env.read();

            let mut keys: Vec<&str> = env.values.keys().map(|key| key.as_str()).collect();
            keys.extend(vec![
                "NOW",
                "TIMESTAMP",
                "UUIDv4",
                "UUIDv7"
            ]);

            for match_ in regex.captures_iter(input) {
                for sub_match in match_.iter() {
                    if let Some(sub_match) = sub_match {
                        for key in &keys {
                            if sub_match.as_str() == &format!("{{{{{}}}}}", key) {
                                let range = sub_match.range();

                                spans.push(Span::raw(input[tmp_index..range.start].to_string()));
                                spans.push(Span::raw(sub_match.as_str().to_owned()).cyan());

                                tmp_index = range.end;
                            }
                        }
                    }
                }
            }

            spans.push(Span::raw(String::from(&input[tmp_index..input.len()])));
        }
        else {
            spans.push(Span::raw(input.to_string()));
        }

        return Line::from(spans);
    }
}