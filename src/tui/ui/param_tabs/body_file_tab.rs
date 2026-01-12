use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;

use crate::app::app::App;
use crate::tui::app_states::AppState::EditingRequestBodyFile;
use crate::tui::utils::stateful::text_input::SingleLineTextInput;

impl App<'_> {
    pub(super) fn render_file_body_tab(&mut self, frame: &mut Frame, area: Rect) {
        let file_body_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(3),
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(4)
            .split(area);

        let should_display_cursor= matches!(&self.state, EditingRequestBodyFile);

        self.body_file_text_input.highlight_text = true;
        self.body_file_text_input.highlight_block = true;
        self.body_file_text_input.display_cursor = should_display_cursor;
        
        frame.render_widget(SingleLineTextInput(&mut self.body_file_text_input), file_body_layout[0]);
    }
}