use ratatui::Frame;
use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::app_states::AppState::{SelectedRequest, EditingRequestAuthBasicUsername, EditingRequestAuthBasicPassword};

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
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);


        let mut password_block = Block::new()
            .title("Password")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);


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

        let username_adjusted_input_length = basic_auth_layout[0].width as usize - 2;
        let (username_padded_text, username_input_cursor_position) = self.auth_basic_username_text_input.get_padded_text_and_cursor(username_adjusted_input_length);
        
        let password_adjusted_input_length = basic_auth_layout[1].width as usize - 2;
        let (password_padded_text, password_input_cursor_position) = self.auth_basic_password_text_input.get_padded_text_and_cursor(password_adjusted_input_length);

        let username_line = self.tui_add_color_to_env_keys(&username_padded_text);
        let password_line = self.tui_add_color_to_env_keys(&password_padded_text);

        let mut username_paragraph = Paragraph::new(username_line)
            .fg(THEME.read().ui.font_color);
        let mut password_paragraph = Paragraph::new(password_line)
            .fg(THEME.read().ui.font_color);

        let input_selected = self.auth_text_input_selection.selected;

        let input_cursor_position = match input_selected {
            0 if should_color_blocks => {
                username_block = username_block.fg(THEME.read().others.selection_highlight_color);
                username_paragraph = username_paragraph.fg(THEME.read().others.selection_highlight_color);
                
                username_input_cursor_position as u16
            },
            1 if should_color_blocks => {
                password_block = password_block.fg(THEME.read().others.selection_highlight_color);
                password_paragraph = password_paragraph.fg(THEME.read().others.selection_highlight_color);

                password_input_cursor_position as u16
            },
            _ => 0
        };

        if should_display_cursor {
            frame.set_cursor_position(Position::new(
                basic_auth_layout[input_selected].x + input_cursor_position + 1,
                basic_auth_layout[input_selected].y + 1
            ));
        }

        username_paragraph = username_paragraph.block(username_block);
        password_paragraph = password_paragraph.block(password_block);

        frame.render_widget(username_paragraph, basic_auth_layout[0]);
        frame.render_widget(password_paragraph, basic_auth_layout[1]);
    }
}