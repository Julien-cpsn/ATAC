use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem};
use crate::app::app::App;

impl App<'_> {
    pub(super) fn render_collection(&mut self, frame: &mut Frame, rect: Rect) {
        let items: Vec<ListItem> = self.collection.items
            .iter()
            .map(|request| {
                request.to_list_item()
            })
            .collect();

        let list = List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .block(
                Block::default()
                    .title("Collection")
                    .borders(Borders::ALL)
            );

        frame.render_stateful_widget(
            list,
            rect,
            &mut self.collection.state
        );
    }
}