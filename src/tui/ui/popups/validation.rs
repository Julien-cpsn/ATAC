use ratatui::Frame;
use ratatui::prelude::Style;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::utils::centered_rect::centered_rect;

impl App<'_> {
    pub fn render_success_popup(&mut self, frame: &mut Frame, message: String) {
        let popup_block = Block::default()
            .title("Success")
            .borders(Borders::ALL)
            .style(Style::default().bg(THEME.read().ui.main_background_color));

        let area = centered_rect(60, 3, frame.area());

        let message_paragraph = Paragraph::new(message.as_str())
            .block(Block::default().borders(Borders::NONE));

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        frame.render_widget(message_paragraph, area);
    }

    pub fn render_error_popup(&mut self, frame: &mut Frame, message: String) {
        let popup_block = Block::default()
            .title("Error")
            .borders(Borders::ALL)
            .style(Style::default().bg(THEME.read().ui.main_background_color));

        let area = centered_rect(60, 3, frame.area());

        let message_paragraph = Paragraph::new(message.as_str())
            .block(Block::default().borders(Borders::NONE));

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        frame.render_widget(message_paragraph, area);
    }
}
