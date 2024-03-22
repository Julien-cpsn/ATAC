use ratatui::Frame;
use ratatui::layout::Direction::Horizontal;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Color, Style};
use ratatui::style::Color::Yellow;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use crate::app::app::App;
use crate::utils::centered_rect::centered_rect;

impl App<'_> {
    pub fn render_creating_element_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Choose element to create")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(30, 3, frame.size());

        let deleting_request_layout = Layout::new(
            Horizontal,
            vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(1)
            .split(area);

        let mut collection_paragraph = Paragraph::new("Collection").centered();
        let mut request_paragraph = Paragraph::new("Request").centered();

        match self.creation_popup.state {
            false => collection_paragraph = collection_paragraph.fg(Yellow).bold(),
            true => request_paragraph = request_paragraph.fg(Yellow).bold(),
        }

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        frame.render_widget(collection_paragraph, deleting_request_layout[0]);
        frame.render_widget(request_paragraph, deleting_request_layout[1]);
    }
}