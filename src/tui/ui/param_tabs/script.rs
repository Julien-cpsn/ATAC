use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::app_states::AppState::{EditingPostRequestScript, EditingPreRequestScript};
use crate::tui::utils::stateful::text_input::MultiLineTextInput;
use crate::tui::utils::syntax_highlighting::JS_SYNTAX_REF;

impl App<'_> {
    pub(super) fn render_request_script(&mut self, frame: &mut Frame, area: Rect) {
        let scripts_layout = Layout::new(
            Vertical,
            vec![
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50)
            ]
        )
            .split(area);

        let mut highlight_pre_request_script = false;
        let display_pre_request_script_cursor = matches!(&self.state, EditingPreRequestScript);
        let mut highlight_post_request_script = false;
        let display_post_request_script_cursor = matches!(&self.state, EditingPostRequestScript);

        let title = match self.script_console.script_selection {
            0 => {
                highlight_pre_request_script = true;

                " ↑ Pre-request "
            },
            1 => {
                highlight_post_request_script = true;

                " Post-request ↓ "
            },
            _ => ""
        };

        let inter_script_block = Block::default()
            .borders(Borders::BOTTOM)
            .title_bottom(title)
            .title_alignment(Alignment::Center)
            .fg(THEME.read().ui.main_foreground_color);

        self.script_console.pre_request_text_area.highlight_text = highlight_pre_request_script;
        self.script_console.pre_request_text_area.display_cursor = display_pre_request_script_cursor;
        self.script_console.post_request_text_area.highlight_text = highlight_post_request_script;
        self.script_console.post_request_text_area.display_cursor = display_post_request_script_cursor;
        
        frame.render_widget(MultiLineTextInput(&mut self.script_console.pre_request_text_area, JS_SYNTAX_REF.clone()), scripts_layout[0]);
        frame.render_widget(inter_script_block, scripts_layout[1]);
        frame.render_widget(MultiLineTextInput(&mut self.script_console.post_request_text_area, JS_SYNTAX_REF.clone()), scripts_layout[2]);
    }
}