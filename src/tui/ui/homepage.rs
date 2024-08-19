use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::Stylize;
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph};
use tui_big_text::{BigTextBuilder, PixelSize};
use crate::app::app::App;
use crate::app::files::theme::THEME;

impl App<'_> {
    pub(super) fn render_homepage(&mut self, frame: &mut Frame, rect: Rect) {
        let block = Block::new();

        let inner_block_area = block.inner(rect);

        let inner_layout = Layout::new(
            Vertical,
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(4),
                Constraint::Length(1),
                Constraint::Percentage(50)
            ]
        )
            .split(inner_block_area);

        let title_length = 16;

        let title_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage((100-title_length)/2+2),
                Constraint::Length(title_length),
                Constraint::Percentage((100-title_length)/2),
            ]
        )
            .split(inner_layout[3]);

        let title = BigTextBuilder::default()
            .pixel_size(PixelSize::Quadrant)
            .lines([
                "ATAC".into(),
            ])
            .style(Style::new().fg(THEME.read().ui.font_color))
            .build();


        let welcome_to = Paragraph::new("Welcome to")
            .centered()
            .fg(THEME.read().ui.secondary_foreground_color);
        let description = Paragraph::new("{A}rguably a {T}erminal {A}PI {C}lient")
            .centered()
            .fg(THEME.read().ui.main_foreground_color);

        frame.render_widget(block, rect);
        frame.render_widget(welcome_to, inner_layout[1]);
        frame.render_widget(title, title_layout[1]);
        frame.render_widget(description, inner_layout[4]);
    }
}