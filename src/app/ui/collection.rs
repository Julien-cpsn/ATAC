use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::{Block, Borders};
use tui_tree_widget::{Tree, TreeItem};
use crate::app::app::App;

impl<'a> App<'a> {
    pub(super) fn render_collection(&mut self, frame: &mut Frame, rect: Rect) {
        let items: Vec<TreeItem<'a, usize>> = self.collections
            .iter()
            .enumerate()
            .map(|(collection_index, request)| {
                request.to_tree_item(collection_index)
            })
            .collect();

        let collections_tree = Tree::new(self.collections_tree.items.clone())
            .unwrap()
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .node_closed_symbol("")
            .node_no_children_symbol("")
            .block(
                Block::default()
                    .title("Collections")
                    .borders(Borders::ALL)
            );

        self.collections_tree.items = items;

        frame.render_stateful_widget(
            collections_tree,
            rect,
            &mut self.collections_tree.state
        );
    }
}