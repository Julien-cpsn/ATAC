use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::app_states::AppState;

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

        let file_body_block = Block::new()
            .title("File path")
            .borders(Borders::ALL)
            .fg(THEME.read().others.selection_highlight_color);

        let mut should_display_cursor = false;

        if self.state == AppState::EditingRequestBodyFile {
            should_display_cursor = true;
        }

        let adjusted_input_length = file_body_layout[0].width as usize - 2;
        let (padded_text, input_cursor_position) = self.body_file_text_input.get_padded_text_and_cursor(adjusted_input_length);

        if should_display_cursor {
            frame.set_cursor_position(Position::new(
                file_body_layout[0].x + input_cursor_position as u16 + 1,
                file_body_layout[0].y + 1
            ));
        }

        let file_body_line = self.tui_add_color_to_env_keys(&padded_text);

        let file_body_paragraph = Paragraph::new(file_body_line)
            .block(file_body_block)
            .fg(THEME.read().others.selection_highlight_color);

        frame.render_widget(file_body_paragraph, file_body_layout[0]);
    }
}