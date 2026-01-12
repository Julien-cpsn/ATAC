use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::app_states::AppState::{EditingRequestAuthJwtSecret, EditingRequestAuthJwtPayload, SelectedRequest};
use crate::tui::utils::syntax_highlighting::JSON_SYNTAX_REF;
use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use crate::tui::utils::stateful::text_input::{MultiLineTextInput, SingleLineTextInput};

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

        let mut should_color_blocks = false;
        let mut should_display_cursor = false;

        // Prevent from rendering the cursor while no input text has been selected
        match self.state {
            SelectedRequest => {
                should_color_blocks = true;
            },
            EditingRequestAuthJwtSecret | EditingRequestAuthJwtPayload => {
                should_color_blocks = true;
                should_display_cursor = true;
            },
            _ => {}
        };
        
        let mut algorithm_paragraph = Paragraph::new(algorithm.to_string())
            .fg(THEME.read().ui.font_color);
        let mut secret_type_paragraph = Paragraph::new(secret_type.to_string())
            .fg(THEME.read().ui.font_color);

        let mut highlight_secret = false;
        let mut display_secret_cursor = false;
        let mut highlight_payload = false;
        let mut display_payload_cursor = false;

        let input_selected = self.auth_text_input_selection.selected;

        match input_selected {
            0 if should_color_blocks => {
                algorithm_block = algorithm_block.fg(THEME.read().others.selection_highlight_color);
                algorithm_paragraph = algorithm_paragraph.fg(THEME.read().others.selection_highlight_color);
            },
            1 if should_color_blocks => {
                secret_type_block = secret_type_block.fg(THEME.read().others.selection_highlight_color);
                secret_type_paragraph = secret_type_paragraph.fg(THEME.read().others.selection_highlight_color);
            },
            2 if should_color_blocks => {
                highlight_secret = true;
                display_secret_cursor = should_display_cursor;
            },
            3 if should_color_blocks => {
                highlight_payload = true;
                display_payload_cursor = should_display_cursor;
            },
            _ => {}
        }


        algorithm_paragraph = algorithm_paragraph.block(algorithm_block);
        secret_type_paragraph = secret_type_paragraph.block(secret_type_block);


        self.auth_jwt_secret_text_input.block_title = Some(format!("Secret ({})", algorithm.get_helper()));
        self.auth_jwt_secret_text_input.highlight_text = highlight_secret;
        self.auth_jwt_secret_text_input.highlight_block = highlight_secret;
        self.auth_jwt_secret_text_input.display_cursor = display_secret_cursor;

        self.auth_jwt_payload_text_area.highlight_text = highlight_payload;
        self.auth_jwt_payload_text_area.highlight_block = highlight_payload;
        self.auth_jwt_payload_text_area.display_cursor = display_payload_cursor;
        
        frame.render_widget(algorithm_paragraph, jwt_token_auth_layout[0]);
        frame.render_widget(secret_type_paragraph, jwt_token_auth_layout[1]);
        frame.render_widget(SingleLineTextInput(&mut self.auth_jwt_secret_text_input), jwt_token_auth_layout[2]);
        frame.render_widget(MultiLineTextInput(&mut self.auth_jwt_payload_text_area, JSON_SYNTAX_REF.clone()), jwt_token_auth_layout[3]);
    }
}
