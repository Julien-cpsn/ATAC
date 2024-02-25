use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::layout::Direction::Vertical;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::app::App;
use crate::utils::centered_rect::centered_rect;

impl App<'_> {
    pub fn render_creating_new_request_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));


        let area = centered_rect(40, 20, 6, 50, frame.size());

        let new_request_layout = Layout::new(
            Vertical,
            vec![
                Constraint::Length(3),
                Constraint::Length(3),
            ]
        )
            .split(area);


        let selected_collection_name = self.collections[self.new_request_popup.selected_collection].name.clone();
        let selected_collection_paragraph = Paragraph::new(selected_collection_name)
            .block(
                Block::new()
                    .title("Collection ↑ ↓")
                    .borders(Borders::ALL)
            );

        let new_request_name_paragraph = Paragraph::new(self.new_request_popup.text_input.text.as_str())
            .block(
                Block::new()
                    .title("Request name")
                    .borders(Borders::ALL)
            );

        frame.render_widget(popup_block, area);
        frame.render_widget(selected_collection_paragraph, new_request_layout[0]);
        frame.render_widget(new_request_name_paragraph, new_request_layout[1]);

        frame.set_cursor(
            new_request_layout[1].x + self.new_request_popup.text_input.cursor_position as u16 + 1,
            new_request_layout[1].y + 1
        )
    }
}