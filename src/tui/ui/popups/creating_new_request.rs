use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position};
use ratatui::layout::Direction::Vertical;
use ratatui::prelude::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::utils::centered_rect::centered_rect;
impl App<'_> {
    pub fn render_creating_new_request_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color)
            .bg(THEME.read().ui.main_background_color);


        let area = centered_rect(50, 6, frame.area());

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
            .fg(THEME.read().ui.font_color)
            .block(
                Block::new()
                    .title("Collection ↑ ↓")
                    .borders(Borders::ALL)
                    .fg(THEME.read().ui.main_foreground_color)
            );

        let adjusted_input_length = new_request_layout[1].width as usize - 2;
        let (padded_text, input_cursor_position) = self.new_request_popup.text_input.get_padded_text_and_cursor(adjusted_input_length);
        
        let new_request_name_paragraph = Paragraph::new(padded_text)
            .fg(THEME.read().ui.font_color)
            .block(
                Block::new()
                    .title("Request name")
                    .borders(Borders::ALL)
                    .fg(THEME.read().ui.main_foreground_color)
            );

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        frame.render_widget(selected_collection_paragraph, new_request_layout[0]);
        frame.render_widget(new_request_name_paragraph, new_request_layout[1]);

        frame.set_cursor_position(Position::new(
            new_request_layout[1].x + input_cursor_position as u16 + 1,
            new_request_layout[1].y + 1
        ));
    }
}