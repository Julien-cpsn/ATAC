use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::app_states::AppState::{EditingRequestAuthJwtPayload, EditingRequestAuthJwtSecret, EditingRequestAuthJwtAlgorithm, SelectedRequest};

impl App<'_> {
    pub(super) fn render_jwt_token_tab(&mut self, frame: &mut Frame, area: Rect) {
        let jwt_token_auth_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(4)
            .split(area);

        let mut algorithm_block = Block::new()
            .title("algorithm")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut secret_block = Block::new()
            .title("secret")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut payload_block = Block::new()
            .title("payload")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut should_color_blocks = false;
        let mut should_display_cursor = false;

        // Prevent from rendering the cursor while no input text has been selected
        match self.state {
            SelectedRequest => {
                should_color_blocks = true;
            },
            EditingRequestAuthJwtAlgorithm | EditingRequestAuthJwtSecret | EditingRequestAuthJwtPayload => {
                should_color_blocks = true;
                should_display_cursor = true;
            },
            _ => {}
        };

        let algorithm_adjusted_input_length = jwt_token_auth_layout[0].width as usize - 2;
        let (algorithm_padded_text, algorithm_input_cursor_position) = self.auth_jwt_algorithm_text_input.get_padded_text_and_cursor(algorithm_adjusted_input_length);
        
        let secret_adjusted_input_length = jwt_token_auth_layout[1].width as usize - 2;
        let (secret_padded_text, secret_input_cursor_position) = self.auth_jwt_secret_text_input.get_padded_text_and_cursor(secret_adjusted_input_length);

        let payload_adjusted_input_length = jwt_token_auth_layout[2].width as usize - 2;
        let (payload_padded_text, payload_input_cursor_position) = self.auth_jwt_payload_text_input.get_padded_text_and_cursor(payload_adjusted_input_length);

        let algorithm_line = self.tui_add_color_to_env_keys(&algorithm_padded_text);
        let secret_line = self.tui_add_color_to_env_keys(&secret_padded_text);
        let payload_line = self.tui_add_color_to_env_keys(&payload_padded_text);

        let mut algorithm_paragraph = Paragraph::new(algorithm_line)
            .fg(THEME.read().ui.font_color);
        let mut secret_paragraph = Paragraph::new(secret_line)
            .fg(THEME.read().ui.font_color);
        let mut payload_paragraph = Paragraph::new(payload_line)
            .fg(THEME.read().ui.font_color);

        let input_selected = self.auth_text_input_selection.selected;

        let input_cursor_position = match input_selected {
            0 if should_color_blocks => {
                algorithm_block = algorithm_block.fg(THEME.read().others.selection_highlight_color);
                algorithm_paragraph = algorithm_paragraph.fg(THEME.read().others.selection_highlight_color);
                
                algorithm_input_cursor_position as u16
            },
            1 if should_color_blocks => {
                secret_block = secret_block.fg(THEME.read().others.selection_highlight_color);
                secret_paragraph = secret_paragraph.fg(THEME.read().others.selection_highlight_color);

                secret_input_cursor_position as u16
            },
            2 if should_color_blocks => {
                payload_block = payload_block.fg(THEME.read().others.selection_highlight_color);
                payload_paragraph = payload_paragraph.fg(THEME.read().others.selection_highlight_color);

                payload_input_cursor_position as u16
            },
            _ => 0
        };

        if should_display_cursor {
            frame.set_cursor_position(Position::new(
                jwt_token_auth_layout[input_selected].x + input_cursor_position + 1,
                jwt_token_auth_layout[input_selected].y + 1
            ));
        }

        algorithm_paragraph = algorithm_paragraph.block(algorithm_block);
        secret_paragraph = secret_paragraph.block(secret_block);
        payload_paragraph = payload_paragraph.block(payload_block);

        frame.render_widget(algorithm_paragraph, jwt_token_auth_layout[0]);
        frame.render_widget(secret_paragraph, jwt_token_auth_layout[1]);
        frame.render_widget(payload_paragraph, jwt_token_auth_layout[2]);
    }
}
