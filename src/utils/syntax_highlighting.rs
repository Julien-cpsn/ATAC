use std::marker::PhantomData;
use ratatui::prelude::{Color, Span, Style};
use ratatui::text::Line;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub struct SyntaxHighlighting<'a> {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
    pub phantom_data: PhantomData<&'a ()>
}

impl<'a> SyntaxHighlighting<'a> {
    pub fn highlight(&self, string: &'a str, extension: &str) -> Vec<Line> {
        let syntax = self.syntax_set.find_syntax_by_extension(extension).unwrap();
        let mut highlight = HighlightLines::new(syntax, &self.theme_set.themes["base16-ocean.dark"]);

        let mut highlighted_lines: Vec<Line> = vec![];

        for line in string.lines() {
            let result = highlight.highlight_line(line, &self.syntax_set).unwrap();

            let mut highlighted_line = Line::default();

            for &(ref style, text) in result.iter() {
                highlighted_line.spans.push(
                    Span::raw(text)
                        .style(Style::new().fg(Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b)))
                );
            }

            highlighted_lines.push(highlighted_line);
        }

        highlighted_lines
    }
}