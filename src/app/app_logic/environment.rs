use std::ops::Range;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use regex::Regex;
use crate::app::app::App;

impl App<'_> {
    pub fn next_environment(&mut self) {
        if self.selected_environment + 1 < self.environments.len() {
            self.selected_environment += 1;
        }
        else {
            self.selected_environment = 0;
        }
    }

    pub fn replace_env_keys_by_value(&self, input: &String) -> String {
        if self.environments.is_empty() {
            return input.to_string();
        }

        let mut tmp_string = input.to_string();

        for (key, value) in &self.environments[self.selected_environment].values {
            tmp_string = tmp_string.replace(&format!("{{{{{}}}}}", key), value);
        }

        return tmp_string;
    }

    pub fn add_color_to_env_keys(&self, input: &String) -> Line {
        if self.environments.is_empty() {
            return Line::raw(input.to_string());
        }

        let tmp_string = input;
        let mut spans: Vec<Span> = vec![];

        let regex = Regex::new(r"\{\{(\w+)}}").unwrap();
        let mut tmp_index: usize = 0;

        for match_ in regex.captures_iter(tmp_string) {
            for sub_match in match_.iter() {
                if let Some(sub_match) = sub_match {
                    for (key, _) in &self.environments[self.selected_environment].values {
                        if sub_match.as_str() == &format!("{{{{{}}}}}", key) {
                            let range = sub_match.range();

                            spans.push(Span::raw(String::from(&tmp_string[tmp_index..range.start])));
                            spans.push(Span::raw(String::from(sub_match.as_str())).cyan());

                            tmp_index = range.end;
                        }
                    }
                }
            }
        }

        spans.push(Span::raw(String::from(&tmp_string[tmp_index..tmp_string.len()])));

        return Line::from(spans);
    }
}