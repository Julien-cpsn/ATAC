use ratatui::Frame;
use ratatui::layout::Position;
use ratatui::prelude::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::utils::centered_rect::centered_rect;

impl App<'_> {
    pub fn render_renaming_collection_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Enter the new collection name")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color)
            .bg(THEME.read().ui.main_background_color);


        let area = centered_rect(50, 3, frame.area());
        let renaming_collection_area = popup_block.inner(area);

        let adjusted_input_length = renaming_collection_area.width as usize;
        let (padded_text, input_cursor_position) = self.rename_collection_input.get_padded_text_and_cursor(adjusted_input_length);
        
        let new_request_paragraph = Paragraph::new(padded_text).fg(THEME.read().ui.font_color);

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        frame.render_widget(new_request_paragraph, renaming_collection_area);

        frame.set_cursor_position(Position::new(
            renaming_collection_area.x + input_cursor_position as u16,
            renaming_collection_area.y
        ));
    }
}