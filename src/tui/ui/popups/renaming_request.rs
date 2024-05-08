use ratatui::Frame;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::app::App;
use crate::tui::utils::centered_rect::centered_rect;

impl App<'_> {
    pub fn render_renaming_request_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Enter the new request name")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));


        let area = centered_rect(50, 3, frame.size());
        let renaming_request_area = popup_block.inner(area);

        let adjusted_input_length = renaming_request_area.width as usize;
        let (padded_text, input_cursor_position) = self.rename_request_input.get_padded_text_and_cursor(adjusted_input_length);
        
        let new_request_paragraph = Paragraph::new(padded_text);

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        frame.render_widget(new_request_paragraph, renaming_request_area);

        frame.set_cursor(
            renaming_request_area.x + input_cursor_position as u16,
            renaming_request_area.y
        )
    }
}