use ratatui::prelude::Stylize;
use ratatui::widgets::{Block, Borders, Clear};
use ratatui::Frame;

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::utils::centered_rect::centered_rect;
use crate::tui::utils::stateful::text_input::SingleLineTextInput;

impl App<'_> {
    pub fn render_renaming_collection_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Enter the new collection name")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color)
            .bg(THEME.read().ui.main_background_color);


        let area = centered_rect(50, 3, frame.area());
        let renaming_collection_area = popup_block.inner(area);

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);

        self.rename_collection_input.display_cursor = true;
       
        frame.render_widget(SingleLineTextInput(&mut self.rename_collection_input), renaming_collection_area);
    }
}