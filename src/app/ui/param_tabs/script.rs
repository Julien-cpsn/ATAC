use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::prelude::{Color, Style};
use ratatui::style::Color::Yellow;
use ratatui::widgets::{Block, Borders};

use crate::app::app::App;

impl App<'_> {
    pub(super) fn render_request_script(&mut self, frame: &mut Frame, area: Rect) {
        let scripts_layout = Layout::new(
            Vertical,
            vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]
        )
            .split(area);

        self.script_console.pre_request_text_area.set_line_number_style(Style::new().fg(Color::DarkGray));
        self.script_console.post_request_text_area.set_line_number_style(Style::new().fg(Color::DarkGray));

        let pre_request_script_text_area = &mut self.script_console.pre_request_text_area;
        let post_request_script_text_area = &mut self.script_console.post_request_text_area;

        let title = match self.script_console.script_selection {
            0 => {
                pre_request_script_text_area.set_style(Style::new().fg(Yellow));
                post_request_script_text_area.set_style(Style::new());

                " Pre-request "
            },
            1 => {
                pre_request_script_text_area.set_style(Style::new());
                post_request_script_text_area.set_style(Style::new().fg(Yellow));

                " Post-request "
            }
            _ => ""
        };

        pre_request_script_text_area.set_block(
            Block::default()
                .borders(Borders::BOTTOM)
                .title_bottom(title)
                .title_alignment(Alignment::Center)
        );

        frame.render_widget(pre_request_script_text_area.widget(), scripts_layout[0]);
        frame.render_widget(post_request_script_text_area.widget(), scripts_layout[1]);
    }
}