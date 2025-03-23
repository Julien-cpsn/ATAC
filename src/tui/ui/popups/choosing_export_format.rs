use crate::app::app::App;
use crate::tui::utils::centered_rect::centered_rect;
use ratatui::Frame;
use ratatui::layout::Direction::Horizontal;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Color, Style};
use ratatui::style::Color::Yellow;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

impl App<'_> {
    pub fn render_export_format_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Choose request export format")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));


        let nb_elements = self.export_request.choices.len() as u16;

        let area = centered_rect(nb_elements * 15, 4, frame.area());

        let element_percentage = 100 / nb_elements;
        let mut constraints: Vec<Constraint> = vec![];

        for _ in &self.export_request.choices {
            constraints.push(Constraint::Percentage(element_percentage));
        }

        let creating_element_layout = Layout::new(
            Horizontal,
            constraints
        )
            .vertical_margin(1)
            .horizontal_margin(1)
            .split(area);

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);

        for (index, element) in self.export_request.choices.iter().enumerate() {
            let mut paragraph = Paragraph::new(element.clone()).centered();

            if index == self.export_request.selection {
                paragraph = paragraph.fg(Yellow).bold();
            }

            frame.render_widget(paragraph, creating_element_layout[index]);
        }
    }
}