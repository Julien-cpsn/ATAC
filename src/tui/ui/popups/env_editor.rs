use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::app_states::AppState::EditingEnvVariable;
use crate::tui::utils::centered_rect::centered_rect;
use ratatui::layout::Margin;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear};
use ratatui::Frame;
use crate::tui::tui_logic::utils::key_value_vec_to_items_list;

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

        self.env_editor_table.is_editing = matches!(self.state, EditingEnvVariable);

        let mut rows = key_value_vec_to_items_list(&self.get_selected_env_as_local(), &self.env_editor_table.rows);

        frame.render_stateful_widget(&mut self.env_editor_table, env_variables_editor_layout, &mut rows);
    }
}
