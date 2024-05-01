use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;
use ratatui::prelude::Color;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

#[derive(Default)]
pub struct SyntaxHighlighting {
    pub highlighted_body: Arc<RwLock<Option<Vec<Line<'static>>>>>,
    pub highlighted_console_output: Arc<RwLock<Vec<Line<'static>>>>,
}

lazy_static! {
    pub static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    pub static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
}

pub fn highlight(string: &str, extension: &str) -> Option<Vec<Line<'static>>> {
    let syntax = match SYNTAX_SET.find_syntax_by_extension(&extension) {
        None => {
            return None;
        }
        Some(syntax) => syntax
    };

    let mut highlight = HighlightLines::new(syntax, &THEME_SET.themes["base16-ocean.dark"]);

    let mut lines: Vec<Line> = vec![];


    for line in string.lines() {
        let result = highlight.highlight_line(line, &SYNTAX_SET).unwrap();

        let mut highlighted_line: Vec<Span> = vec![];

        for &(ref style, text) in result.iter() {
            highlighted_line.push(Span::raw(text.to_string()).fg(Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b)));
        }

        lines.push(Line::from(highlighted_line));
    }

    return Some(lines);
}