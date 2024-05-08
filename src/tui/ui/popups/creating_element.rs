use ratatui::Frame;
use ratatui::layout::Direction::Horizontal;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Color, Style};
use ratatui::style::Color::Yellow;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::app::App;
use crate::tui::utils::centered_rect::centered_rect;

impl App<'_> {
    pub fn render_creating_element_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Choose element to create")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));


        let nb_elements = self.creation_popup.choices.len() as u16;

        let area = centered_rect(nb_elements * 15, 3, frame.size());

        let element_percentage = 100 / nb_elements;
        let mut constraints: Vec<Constraint> = vec![];

        for _ in &self.creation_popup.choices {
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

        for (index, element) in self.creation_popup.choices.iter().enumerate() {
            let mut paragraph = Paragraph::new(element.clone()).centered();

            if index == self.creation_popup.selection {
                paragraph = paragraph.fg(Yellow).bold();
            }

            frame.render_widget(paragraph, creating_element_layout[index]);
        }
    }
}