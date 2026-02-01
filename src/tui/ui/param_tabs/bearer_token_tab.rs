use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;

use crate::app::app::App;
use crate::tui::app_states::AppState::EditingRequestAuthBearerToken;
use crate::tui::utils::stateful::text_input::SingleLineTextInput;

impl App<'_> {
    pub(super) fn render_bearer_token_tab(&mut self, frame: &mut Frame, area: Rect) {
        let bearer_token_auth_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(3),
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(4)
            .split(area);

        let should_display_cursor= matches!(&self.state, EditingRequestAuthBearerToken);

        self.auth_bearer_token_text_input.highlight_text = true;
        self.auth_bearer_token_text_input.highlight_block = true;
        self.auth_bearer_token_text_input.display_cursor = should_display_cursor;
        
        frame.render_widget(SingleLineTextInput(&mut self.auth_bearer_token_text_input), bearer_token_auth_layout[0]);
    }
}