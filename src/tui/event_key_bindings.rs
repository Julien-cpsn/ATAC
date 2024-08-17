use crokey::KeyCombination;
use ratatui::prelude::Span;
use ratatui::style::{Color, Stylize};

use crate::tui::app_states::EMPTY_KEY;
use crate::app::files::key_bindings::unique_key_and_help;

#[derive(Clone)]
pub struct EventKeyBinding {
    pub keys: Vec<KeyCombination>,
    pub event_name: String,
    pub short_name: Option<String>,
}

impl EventKeyBinding {
    pub fn new(keys: Vec<KeyCombination>, event_name: &str, short_name: Option<&str>) -> EventKeyBinding {
        EventKeyBinding {
            keys,
            event_name: String::from(event_name),
            short_name: match short_name {
                None => None,
                Some(short_name) => Some(String::from(short_name))
            }
        }
    }

    pub fn to_spans(&self, fg_color: Color, bg_color: Color, short_only: bool, is_documentation: bool) -> Option<Vec<Span<'static>>> {
        if self.keys.is_empty() {
            return None;
        }

        let name = if short_only {
            if let Some(short_name) = &self.short_name {
                short_name
            }
            else {
                return None;
            }
        }
        else { 
            &self.event_name
        };

        if is_documentation && self.keys.contains(&EMPTY_KEY) {
            return Some(vec![
                Span::raw(name.clone()).bg(bg_color),
                Span::raw(" "),
                Span::default(),
            ])
        }

        let mut spans = unique_key_and_help(
            Span::raw(name.clone()).bg(bg_color),
            Span::raw(self.keys[0].to_string()).fg(fg_color)
        );

        spans.push(Span::raw(" "));

        if short_only {
            return Some(spans);
        }
        
        if let Some(key) = self.keys.get(1) {
            spans.push(Span::raw(key.to_string()).fg(fg_color));
            spans.push(Span::raw(" "));
        }

        return Some(spans);
    }
}
