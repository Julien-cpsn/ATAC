use ratatui::Frame;
use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::app::App;
use crate::app::app_states::AppState::{EditingRequestAuthPassword, EditingRequestAuthUsername, SelectedRequest};

impl App<'_> {
    pub(super) fn render_basic_auth_tab(&mut self, frame: &mut Frame, area: Rect) {
        let basic_auth_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(3),
                Constraint::Length(3),
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(4)
            .split(area);

        let mut username_block = Block::new()
            .title("Username")
            .borders(Borders::ALL);


        let mut password_block = Block::new()
            .title("Password")
            .borders(Borders::ALL);


        let mut should_color_blocks = false;
        let mut should_display_cursor = false;

        // Prevent from rendering the cursor while no input text has been selected
        match self.state {
            SelectedRequest => {
                should_color_blocks = true;
            },
            EditingRequestAuthUsername | EditingRequestAuthPassword => {
                should_color_blocks = true;
                should_display_cursor = true;
            },
            _ => {}
        };

        let input_selected = self.auth_text_input_selection.selected;

        let input_cursor_position = match input_selected {
            0 if should_color_blocks => {
                username_block = username_block.yellow();
                self.auth_basic_username_text_input.cursor_position as u16
            },
            1 if should_color_blocks => {
                password_block = password_block.yellow();
                self.auth_basic_password_text_input.cursor_position as u16
            },
            _ => 0
        };

        if should_display_cursor {
            frame.set_cursor(
                basic_auth_layout[input_selected].x + input_cursor_position + 1,
                basic_auth_layout[input_selected].y + 1
            );
        }

        let username_line = self.add_color_to_env_keys(&self.auth_basic_username_text_input.text);
        let password_line = self.add_color_to_env_keys(&self.auth_basic_password_text_input.text);

        let username_paragraph = Paragraph::new(username_line).block(username_block);
        let password_paragraph = Paragraph::new(password_line).block(password_block);

        frame.render_widget(username_paragraph, basic_auth_layout[0]);
        frame.render_widget(password_paragraph, basic_auth_layout[1]);
    }
}