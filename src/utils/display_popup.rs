use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Offset};
use ratatui::layout::Direction::Vertical;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation};

use crate::utils::centered_rect::centered_rect;
use crate::utils::colors::DARK_BLACK;
use crate::utils::stateful_scrollbar::StatefulScrollbar;

#[derive(Default)]
pub struct DisplayPopup {
    pub title: String,
    pub content: String,
    pub vertical_scrollbar: StatefulScrollbar,
    pub horizontal_scrollbar: StatefulScrollbar,
}

impl DisplayPopup {
    pub fn render(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title(format!("{} export", self.title))
            .borders(Borders::ALL)
            .white()
            .bg(*DARK_BLACK);

        let area = centered_rect(100, 25, frame.size());
        let outer_area = area.offset(Offset { x: 1, y: 1 } );

        frame.render_widget(Clear, outer_area);
        frame.render_widget(popup_block, area);

        let layout = Layout::new(
            Vertical,
            vec![
                Constraint::Fill(1)
            ]
        )
            .vertical_margin(2)
            .horizontal_margin(2)
            .split(area);

        let content = Paragraph::new(self.content.clone())
            .scroll((self.vertical_scrollbar.scroll, self.horizontal_scrollbar.scroll))
            .dark_gray();

        frame.render_widget(content, layout[0]);
        
        let vertical_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        let horizontal_scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
            .thumb_symbol("■");

        frame.render_stateful_widget(
            vertical_scrollbar,
            outer_area,
            &mut self.vertical_scrollbar.state
        );

        frame.render_stateful_widget(
            horizontal_scrollbar,
            outer_area,
            &mut self.horizontal_scrollbar.state
        );
    }
}
