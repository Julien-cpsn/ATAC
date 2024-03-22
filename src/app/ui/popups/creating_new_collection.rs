use ratatui::Frame;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use crate::app::app::App;
use crate::utils::centered_rect::centered_rect;

impl App<'_> {
    pub fn render_creating_new_collection_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Enter the new collection name")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));


        let area = centered_rect(50, 3, frame.size());
        let new_request_area = popup_block.inner(area);

        let new_request_paragraph = Paragraph::new(self.new_collection_input.text.as_str());

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        frame.render_widget(new_request_paragraph, new_request_area);

        frame.set_cursor(
            new_request_area.x + self.new_collection_input.cursor_position as u16,
            new_request_area.y
        )
    }
}