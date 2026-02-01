use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;

use crate::app::app::App;
use crate::tui::app_states::AppState::{EditingRequestAuthBasicPassword, EditingRequestAuthBasicUsername, SelectedRequest};
use crate::tui::utils::stateful::text_input::SingleLineTextInput;

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

        let mut should_color_blocks = false;
        let mut should_display_cursor = false;

        // Prevent from rendering the cursor while no input text has been selected
        match self.state {
            SelectedRequest => {
                should_color_blocks = true;
            },
            EditingRequestAuthBasicUsername | EditingRequestAuthBasicPassword => {
                should_color_blocks = true;
                should_display_cursor = true;
            },
            _ => {}
        };

        let mut highlight_username = false;
        let mut display_username_cursor = false;
        let mut highlight_password = false;
        let mut display_password_cursor = false;

        let input_selected = self.auth_text_input_selection.selected;

        match input_selected {
            0 if should_color_blocks => {
                highlight_username = true;
                display_username_cursor = should_display_cursor;
            },
            1 if should_color_blocks => {
                highlight_password = true;
                display_password_cursor = should_display_cursor;
            },
            _ => {}
        };

        self.auth_basic_username_text_input.highlight_text = highlight_username;
        self.auth_basic_username_text_input.highlight_block = highlight_username;
        self.auth_basic_username_text_input.display_cursor = display_username_cursor;
        self.auth_basic_password_text_input.highlight_text = highlight_password;
        self.auth_basic_password_text_input.highlight_block = highlight_password;
        self.auth_basic_password_text_input.display_cursor = display_password_cursor;

        frame.render_widget(SingleLineTextInput(&mut self.auth_basic_username_text_input), basic_auth_layout[0]);
        frame.render_widget(SingleLineTextInput(&mut self.auth_basic_password_text_input), basic_auth_layout[1]);
    }
}