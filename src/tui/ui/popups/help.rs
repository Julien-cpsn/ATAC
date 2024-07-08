use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::Line;
use ratatui::style::Color::{DarkGray, Gray};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::app::App;
use crate::tui::app_states::event_available_keys_to_spans;
use crate::tui::utils::centered_rect::centered_rect;
use crate::tui::utils::colors::DARK_BLACK;


const NB_LINES: usize = 9;
const LINE_LENGTH: usize = 2;
const LEFT_MAX:usize = NB_LINES-1;
const MIDDLE_MAX:usize = 2*NB_LINES-1;
const RIGHT_MIN:usize = 2*NB_LINES;
const RIGHT_MAX:usize = 3*NB_LINES-1;

impl App<'_> {
    pub fn render_help_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .borders(Borders::ALL)
            .white()
            .bg(*DARK_BLACK);

        let area = centered_rect(110, 26, frame.size());

        frame.set_cursor(0, 0);
        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);

        let help_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Fill(1),
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(1)
            .split(area);

        let title_paragraph = Paragraph::new(self.help_popup.selection.to_string().bold().underlined()).centered();
        frame.render_widget(title_paragraph, help_layout[1]);

        let help_keys_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
        )
            .vertical_margin(2)
            .horizontal_margin(4)
            .split(help_layout[2]);


        let lines = [Constraint::Length(1); LINE_LENGTH*NB_LINES];

        let left_layout = Layout::new(Vertical, lines.clone()).split(help_keys_layout[0]);
        let middle_layout = Layout::new(Vertical, lines.clone()).split(help_keys_layout[1]);
        let right_layout = Layout::new(Vertical, lines.clone()).split(help_keys_layout[2]);

        let events = &self.help_popup.selection.get_available_events(self.request_view, self.request_param_tab);
        let keys = event_available_keys_to_spans(events, Gray, DarkGray, false);

        for i in 0..3 * NB_LINES {
            if i >= keys.len() {
                break;
            }

            let keys_line = Line::from(keys[i].clone());
            let keys_paragraph = Paragraph::new(keys_line);

            match i {
                0..=LEFT_MAX => {
                    frame.render_widget(keys_paragraph, left_layout[i*LINE_LENGTH]);
                },
                NB_LINES..=MIDDLE_MAX => {
                    frame.render_widget(keys_paragraph, middle_layout[(i-NB_LINES)*LINE_LENGTH]);
                },
                RIGHT_MIN..=RIGHT_MAX => {
                    frame.render_widget(keys_paragraph, right_layout[(i-2*NB_LINES)*LINE_LENGTH]);
                },
                _ => {}
            }
        }
    }
}