use ratatui::Frame;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::utils::centered_rect::centered_rect;
use crate::tui::utils::stateful::text_input::SingleLineTextInput;

impl App<'_> {
    pub fn render_exporting_response_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Export response body file path")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color)
            .bg(THEME.read().ui.main_background_color);

        let area = centered_rect(60, 3, frame.area());
        let exporting_response_area = popup_block.inner(area);

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        
        self.export_response_input.display_cursor = true;
        
        frame.render_widget(SingleLineTextInput(&mut self.export_response_input), exporting_response_area);
    }
}
