use std::ops::Deref;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::prelude::{Style, Stylize};
use ratatui::widgets::{Block, Borders};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::app_states::AppState;

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

        self.script_console.pre_request_text_area.set_line_number_style(Style::new().fg(THEME.read().ui.secondary_foreground_color));
        self.script_console.post_request_text_area.set_line_number_style(Style::new().fg(THEME.read().ui.secondary_foreground_color));

        let pre_request_script_text_area = &mut self.script_console.pre_request_text_area;
        let post_request_script_text_area = &mut self.script_console.post_request_text_area;

        let title = match self.script_console.script_selection {
            0 => " Pre-request ",
            1 => " Post-request ",
            _ => ""
        };
        
        if self.state == AppState::SelectedRequest {
            match self.script_console.script_selection {
                0 => {
                    pre_request_script_text_area.set_style(Style::new().fg(THEME.read().others.selection_highlight_color));
                    post_request_script_text_area.set_style(Style::new().fg(THEME.read().ui.font_color));
                },
                1 => {
                    pre_request_script_text_area.set_style(Style::new().fg(THEME.read().ui.font_color));
                    post_request_script_text_area.set_style(Style::new().fg(THEME.read().others.selection_highlight_color));
                }
                _ => {}
            };
        }
        else {
            pre_request_script_text_area.set_style(Style::new().fg(THEME.read().ui.font_color));
            post_request_script_text_area.set_style(Style::new().fg(THEME.read().ui.font_color));
        }

        pre_request_script_text_area.set_block(
            Block::default()
                .borders(Borders::BOTTOM)
                .title_bottom(title)
                .title_alignment(Alignment::Center)
                .fg(THEME.read().ui.main_foreground_color)
        );

        frame.render_widget(pre_request_script_text_area.deref(), scripts_layout[0]);
        frame.render_widget(post_request_script_text_area.deref(), scripts_layout[1]);
    }
}