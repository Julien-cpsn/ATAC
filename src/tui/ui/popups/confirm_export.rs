use ratatui::Frame;
use ratatui::prelude::Stylize;
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::utils::centered_rect::centered_rect;

impl App<'_> {
    pub fn render_success_popup(&mut self, frame: &mut Frame, message: String) {
        self.render_confirm_popup(frame, message, true);
    }

    pub fn render_error_popup(&mut self, frame: &mut Frame, message: String) {
        self.render_confirm_popup(frame, message, false);
    }

    pub fn render_confirm_popup(&mut self, frame: &mut Frame, message: String, success: bool) {
        let popup_block = Block::default()
            .title(if success {"Success"} else {"Error"})
            .borders(Borders::ALL)
            .fg(if success { Color::Green } else { Color::Red })//THEME.read().ui.main_foreground_color)
            .bg(THEME.read().ui.main_background_color);
        
        let area = centered_rect(60, 3, frame.area());
        let confirm_message_area = popup_block.inner(area);

        let message_paragraph = Paragraph::new(message.as_str()).fg(THEME.read().ui.font_color);

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        frame.render_widget(message_paragraph, confirm_message_area);
    }
}
