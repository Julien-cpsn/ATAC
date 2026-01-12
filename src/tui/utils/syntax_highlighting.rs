use std::sync::Arc;
use lazy_static::lazy_static;
use ratatui::prelude::Color;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::{SyntaxDefinition, SyntaxReference, SyntaxSet, SyntaxSetBuilder};

#[derive(Default)]
pub struct SyntaxHighlighting {
    pub highlighted_body: Option<Vec<Line<'static>>>,
    pub highlighted_console_output: Vec<Line<'static>>,
}

lazy_static! {
    pub static ref SYNTAX_SET: Arc<SyntaxSet> = Arc::new(SyntaxSet::load_defaults_newlines());
    pub static ref ENV_VARIABLE_SYNTAX_SET: Arc<SyntaxSet> = Arc::new(generate_env_variable_syntax_set());

    pub static ref ENV_VARIABLE_SYNTAX_REF: &'static SyntaxReference = ENV_VARIABLE_SYNTAX_SET.syntaxes().first().unwrap();
    pub static ref JSON_SYNTAX_REF: &'static SyntaxReference = SYNTAX_SET.find_syntax_by_extension("json").unwrap();
    pub static ref XML_SYNTAX_REF: &'static SyntaxReference = SYNTAX_SET.find_syntax_by_extension("xml").unwrap();
    pub static ref HTML_SYNTAX_REF: &'static SyntaxReference = SYNTAX_SET.find_syntax_by_extension("html").unwrap();
    pub static ref JS_SYNTAX_REF: &'static SyntaxReference = SYNTAX_SET.find_syntax_by_extension("js").unwrap();

    pub static ref THEME_SET: Arc<ThemeSet> = Arc::new(ThemeSet::load_defaults());
    pub static ref SYNTAX_THEME: &'static Theme = &THEME_SET.themes["base16-ocean.dark"];
}

pub fn highlight(string: &str, extension: &str) -> Option<Vec<Line<'static>>> {
    let syntax = match extension {
        "json" => &*JSON_SYNTAX_REF,
        "xml" => &*HTML_SYNTAX_REF,
        "html" => &*XML_SYNTAX_REF,
        "js" => &*JS_SYNTAX_REF,
        _ => match SYNTAX_SET.find_syntax_by_extension(&extension) {
            None => return None,
            Some(syntax) => syntax
        }
    };

    let mut highlight = HighlightLines::new(syntax, &SYNTAX_THEME);

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

fn generate_env_variable_syntax_set() -> SyntaxSet {
    let mut syntax_set_builder = SyntaxSetBuilder::new();

    let syntax_def = SyntaxDefinition::load_from_str(
        r#"%YAML 1.2
---
name: Double Brace Variables
file_extensions:
  - dblvars
scope: source.dblvars

contexts:
  main:
    - match: '\{\{[A-Za-z0-9_-]+\}\}'
      scope: variable"#,
        true,
        None
    )
        .unwrap();

    syntax_set_builder.add(syntax_def);

    syntax_set_builder.build()
}