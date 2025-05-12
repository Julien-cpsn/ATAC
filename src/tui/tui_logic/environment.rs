use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use regex::Regex;

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::models::request::KeyValue;

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
                                spans.push(
                                    Span::raw(sub_match.as_str().to_owned())
                                        .fg(THEME.read().others.environment_variable_highlight_color)
                                );

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

    pub fn tui_update_env_variable_table(&mut self) {
        let local_env = self.get_selected_env_as_local().unwrap();
        let env = local_env.read();

        let rows: Vec<KeyValue> = env.values
            .iter()
            .map(|(key, value)| KeyValue {
                enabled: true,
                data: (key.clone(), value.clone()),
            })
            .collect();

        match rows.is_empty() {
            false => self.env_editor_table.update_selection(Some((0, 0))),
            true => self.env_editor_table.update_selection(None),
        };
        self.env_editor_table.rows = rows;
    }

    pub fn tui_modify_env_variable(&mut self) {
        let selected_env_index = self.selected_environment;
        let (row, column) = self.env_editor_table.selection.unwrap();

        let input_text = self.env_editor_table.selection_text_input.text.clone();

        match column {
            0 => match self.rename_env_key_by_index(selected_env_index, row, input_text) {
                Ok(_) => {}
                Err(_) => return,
            },
            1 => match self.set_env_value_by_index(selected_env_index, row, input_text) {
                Ok(_) => {}
                Err(_) => return,
            }
            _ => {}
        }

        self.display_env_editor_state();
    }

    pub fn tui_create_env_variable(&mut self) {
        let selected_env_index = self.selected_environment;

        match self.create_env_value(selected_env_index, None, String::from("VALUE")) {
            Ok(_) => {}
            Err(_) => return,
        }

        self.tui_update_env_variable_table();
    }

    pub fn tui_delete_env_variable(&mut self) {
        if self.env_editor_table.rows.is_empty() || self.env_editor_table.selection.is_none() {
            return;
        }

        let (row, _) = self.env_editor_table.selection.unwrap();
        let selected_env_index = self.selected_environment;

        match self.delete_env_index(selected_env_index, row) {
            Ok(_) => {}
            Err(_) => return
        }

        self.tui_update_env_variable_table();
    }
}