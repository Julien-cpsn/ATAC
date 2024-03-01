use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::app::App;
use crate::app::app_states::AppState::{EditingRequestAuthBearerToken, SelectedRequest};

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

        let mut bearer_token_block = Block::new()
            .title("Bearer token")
            .borders(Borders::ALL);

        let mut should_color_blocks = false;
        let mut should_display_cursor = false;

        // Prevent from rendering the cursor while no input text has been selected
        match self.state {
            SelectedRequest => {
                should_color_blocks = true;
            },
            EditingRequestAuthBearerToken => {
                should_color_blocks = true;
                should_display_cursor = true;
            },
            _ => {}
        };

        let input_selected = self.auth_text_input_selection.selected;

        let input_cursor_position = match input_selected {
            0 if should_color_blocks => {
                bearer_token_block = bearer_token_block.yellow();
                self.auth_bearer_token_text_input.cursor_position as u16
            },
            _ => 0
        };

        if should_display_cursor {
            frame.set_cursor(
                bearer_token_auth_layout[input_selected].x + input_cursor_position + 1,
                bearer_token_auth_layout[input_selected].y + 1
            );
        }

        let bearer_token_line = self.add_color_to_env_keys(&self.auth_bearer_token_text_input.text);

        let bearer_token_paragraph = Paragraph::new(bearer_token_line).block(bearer_token_block);

        frame.render_widget(bearer_token_paragraph, bearer_token_auth_layout[0]);
    }
}