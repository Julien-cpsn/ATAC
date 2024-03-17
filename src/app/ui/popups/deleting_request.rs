use ratatui::Frame;
use ratatui::layout::Direction::Horizontal;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Color, Style};
use ratatui::style::Color::Yellow;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::app::App;
use crate::utils::centered_rect::centered_rect;

impl App<'_> {
    pub fn render_deleting_request_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Confirm delete request")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(25, 100, frame.size()); //TODO centered_rect(30, 20, 3, 30, frame.size());

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

        let mut no_paragraph = Paragraph::new("no").centered();
        let mut yes_paragraph = Paragraph::new("yes").centered();

        match self.delete_request_popup.state {
            false => no_paragraph = no_paragraph.fg(Yellow).bold(),
            true => yes_paragraph = yes_paragraph.fg(Yellow).bold(),
        }

        frame.render_widget(popup_block, area);
        frame.render_widget(no_paragraph, deleting_request_layout[0]);
        frame.render_widget(yes_paragraph, deleting_request_layout[1]);
    }
}