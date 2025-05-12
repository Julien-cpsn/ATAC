use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::utils::centered_rect::centered_rect;
use ratatui::layout::Margin;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear};
use ratatui::Frame;
use ratatui::prelude::Line;
use crate::tui::app_states::AppState::EditingEnvVariable;

impl App<'_> {
    pub fn render_env_editor_popup(&mut self, frame: &mut Frame) {
        let local_env = self.get_selected_env_as_local().unwrap();
        let env = local_env.read();

        let popup_block = Block::default()
            .title(format!("Editing {}", env.name))
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color)
            .bg(THEME.read().ui.secondary_background_color);

        let area = centered_rect(120, 25, frame.area());

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);

        let env_variables_editor_layout = area.inner(Margin::new(1, 1));

        let no_selection_lines = vec![
            Line::default(),
            Line::from("No environment variable").fg(THEME.read().ui.font_color),
            Line::from("(Add one with n)").fg(THEME.read().ui.secondary_foreground_color)
        ];

        self.render_custom_table(
            frame,
            env_variables_editor_layout,
            &self.env_editor_table,
            no_selection_lines,
            EditingEnvVariable,
            "Key",
            "Value"
        )
    }
}
