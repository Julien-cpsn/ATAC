use ratatui::Frame;
use ratatui::layout::Direction::Horizontal;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Style;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::utils::centered_rect::centered_rect;
impl App<'_> {
    pub fn render_deleting_collection_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Confirm delete collection")
            .borders(Borders::ALL)
            .style(Style::default().bg(THEME.read().ui.main_background_color));

        let area = centered_rect(30, 3, frame.area());

        let deleting_request_layout = Layout::new(
            Horizontal,
            vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(1)
            .split(area);

        let mut no_paragraph = Paragraph::new("no").centered();
        let mut yes_paragraph = Paragraph::new("yes").centered();

        match self.delete_collection_popup.state {
            false => no_paragraph = no_paragraph.fg(THEME.read().others.selection_highlight_color).bold(),
            true => yes_paragraph = yes_paragraph.fg(THEME.read().others.selection_highlight_color).bold(),
        }

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        frame.render_widget(no_paragraph, deleting_request_layout[0]);
        frame.render_widget(yes_paragraph, deleting_request_layout[1]);
    }
}