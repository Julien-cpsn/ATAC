use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::app::App;

impl<'a> App<'a> {
    pub(super) fn render_environments(&mut self, frame: &mut Frame, rect: Rect) {
        let current_environment = match self.environments.get(self.selected_environment) {
            None => "None",
            Some(env) => &env.name
        };

        let current_environment_paragraph = Paragraph::new(current_environment)
            .block(
                Block::default()
                    .title("Environment")
                    .borders(Borders::ALL)
                    .dark_gray()
            );

        frame.render_widget(current_environment_paragraph, rect)
    }
}