use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::app::App;
use crate::app::files::theme::THEME;

impl<'a> App<'a> {
    pub(super) fn render_environments(&mut self, frame: &mut Frame, rect: Rect) {
        let local_env = self.get_selected_env_as_local().unwrap();
        let env = local_env.read();

        let current_environment = env.name.clone();

        drop(env);

        let current_environment_paragraph = Paragraph::new(current_environment)
            .block(
                Block::default()
                    .title("Environment")
                    .borders(Borders::ALL)
                    .style(Style::new().fg(THEME.read().ui.secondary_foreground_color))
            );

        frame.render_widget(current_environment_paragraph, rect)
    }
}