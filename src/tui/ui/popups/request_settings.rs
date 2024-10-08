use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::utils::centered_rect::centered_rect;

impl App<'_> {
    pub fn render_request_settings_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Request settings")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color)
            .bg(THEME.read().ui.main_background_color);

        let settings_number = self.request_settings_popup.settings.len() as u16;

        let area = centered_rect(50, 2 + 1 + 2 * settings_number, frame.area());

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);

        let request_settings_layout = Layout::new(
            Horizontal,
            vec![
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(1)
            .split(area);

        let mut constraints: Vec<Constraint> = vec![Constraint::Length(1)];

        for _ in 0..settings_number {
            constraints.push(Constraint::Length(2))
        }

        let settings_names_layout = Layout::new(
            Vertical,
            constraints.clone()
        )
            .horizontal_margin(1)
            .split(request_settings_layout[0]);

        let settings_values_layout = Layout::new(
            Vertical,
            constraints
        )
            .split(request_settings_layout[1]);

        // Render settings
        for (index, (setting_name, setting_value)) in self.request_settings_popup.settings.iter().enumerate() {
            let setting_name_paragraph = Paragraph::new(setting_name.to_string()).fg(THEME.read().ui.font_color).centered();
            let mut setting_value_paragraph = Paragraph::new(setting_value.to_string()).fg(THEME.read().ui.font_color).centered();

            if index == self.request_settings_popup.selection {
                setting_value_paragraph = setting_value_paragraph.fg(THEME.read().others.selection_highlight_color)
            }

            frame.render_widget(setting_name_paragraph, settings_names_layout[index + 1]);
            frame.render_widget(setting_value_paragraph, settings_values_layout[index + 1]);
        }
    }
}