use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::app_states::AppState::{EditingRequestAuthJwtSecret, SelectedRequest};
use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::prelude::Style;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

impl App<'_> {
    pub(super) fn render_jwt_token_tab(&mut self, frame: &mut Frame, area: Rect) {
        let jwt_token_auth_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(3),
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(4)
            .split(area);

        let (algorithm, secret_type) = {
            let local_selected_request = self.get_selected_request_as_local();
            let selected_request = local_selected_request.read();
            let jwt_token = selected_request.auth.get_jwt();

            (jwt_token.algorithm.clone(), jwt_token.secret_type.clone())
        };

        let mut algorithm_block = Block::new()
            .title("Algorithm ← →")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut secret_type_block = Block::new()
            .title("Secret type ← →")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut secret_block = Block::new()
            .title(format!("Secret ({})", algorithm.get_helper()))
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut payload_block = Block::new()
            .title("Payload")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut should_color_blocks = false;
        let mut should_display_cursor = false;

        // Prevent from rendering the cursor while no input text has been selected
        match self.state {
            SelectedRequest => {
                should_color_blocks = true;
            },
            EditingRequestAuthJwtSecret => {
                should_color_blocks = true;
                should_display_cursor = true;
            },
            _ => {}
        };
        
        let secret_adjusted_input_length = jwt_token_auth_layout[2].width as usize - 2;
        let (secret_padded_text, secret_input_cursor_position) = self.auth_jwt_secret_text_input.get_padded_text_and_cursor(secret_adjusted_input_length);

        let secret_line = self.tui_add_color_to_env_keys(&secret_padded_text);

        let mut algorithm_paragraph = Paragraph::new(algorithm.to_string())
            .fg(THEME.read().ui.font_color);
        let mut secret_type_paragraph = Paragraph::new(secret_type.to_string())
            .fg(THEME.read().ui.font_color);
        let mut secret_paragraph = Paragraph::new(secret_line)
            .fg(THEME.read().ui.font_color);

        let mut auth_jwt_payload_text_area_style = Style::new().fg(THEME.read().ui.font_color);

        let input_selected = self.auth_text_input_selection.selected;

        let input_cursor_position = match input_selected {
            0 if should_color_blocks => {
                algorithm_block = algorithm_block.fg(THEME.read().others.selection_highlight_color);
                algorithm_paragraph = algorithm_paragraph.fg(THEME.read().others.selection_highlight_color);
                
                0
            },
            1 if should_color_blocks => {
                secret_type_block = secret_type_block.fg(THEME.read().others.selection_highlight_color);
                secret_type_paragraph = secret_type_paragraph.fg(THEME.read().others.selection_highlight_color);

                0
            },
            2 if should_color_blocks => {
                secret_block = secret_block.fg(THEME.read().others.selection_highlight_color);
                secret_paragraph = secret_paragraph.fg(THEME.read().others.selection_highlight_color);

                secret_input_cursor_position
            },
            3 if should_color_blocks => {
                payload_block = payload_block.fg(THEME.read().others.selection_highlight_color);
                auth_jwt_payload_text_area_style = Style::new().fg(THEME.read().others.selection_highlight_color);

                0
            },
            _ => 0
        };

        if should_display_cursor {
            frame.set_cursor_position(Position::new(
                jwt_token_auth_layout[input_selected].x + input_cursor_position as u16 + 1,
                jwt_token_auth_layout[input_selected].y + 1
            ));
        }

        algorithm_paragraph = algorithm_paragraph.block(algorithm_block);
        secret_type_paragraph = secret_type_paragraph.block(secret_type_block);
        secret_paragraph = secret_paragraph.block(secret_block);

        frame.render_widget(algorithm_paragraph, jwt_token_auth_layout[0]);
        frame.render_widget(secret_type_paragraph, jwt_token_auth_layout[1]);
        frame.render_widget(secret_paragraph, jwt_token_auth_layout[2]);

        self.auth_jwt_payload_text_area.set_block(payload_block);
        self.auth_jwt_payload_text_area.set_style(auth_jwt_payload_text_area_style);
        self.auth_jwt_payload_text_area.set_line_number_style(Style::new().fg(THEME.read().ui.secondary_foreground_color));

        frame.render_widget(&self.auth_jwt_payload_text_area, jwt_token_auth_layout[3]);
    }
}
