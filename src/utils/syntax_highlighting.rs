use std::sync::{Arc, RwLock};

use ratatui::prelude::Color;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub struct SyntaxHighlighting {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
    pub last_highlighted: Arc<RwLock<Option<Vec<Vec<(String, Color)>>>>>
}

impl SyntaxHighlighting {
    pub fn highlight(&self, string: &'_ str, extension: &str) {
        let syntax = match self.syntax_set.find_syntax_by_extension(extension) {
            None => {
                *self.last_highlighted.write().unwrap() = None;
                return;
            }
            Some(syntax) => syntax
        };
        
        let mut highlight = HighlightLines::new(syntax, &self.theme_set.themes["base16-ocean.dark"]);

        let mut lines: Vec<Vec<(String, Color)>> = vec![];
        
        for line in string.lines() {
            let result = highlight.highlight_line(line, &self.syntax_set).unwrap();

            let mut highlighted_line: Vec<(String, Color)> = vec![];

            for &(ref style, text) in result.iter() {
                highlighted_line.push((text.to_string(), Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b)));
            }

            lines.push(highlighted_line)
        }
        
        let mut last_highlighted = self.last_highlighted.write().unwrap();
        *last_highlighted = Some(lines);
    }
}

pub fn last_highlighted_to_lines<'a>(last_highlighted: Vec<Vec<(String, Color)>>) -> Vec<Line<'a>> {
    let mut lines: Vec<Line> = vec![];

    for highlighted_line in last_highlighted {
        let mut spans: Vec<Span> = vec![];

        for (text, color) in highlighted_line {
            spans.push(Span::raw(text).style(Style::new().fg(color.clone())))
        }

        let line = Line::from(spans);
        lines.push(line);
    }

    return lines;
}