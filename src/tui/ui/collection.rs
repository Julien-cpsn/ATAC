use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Modifier, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders};
use tui_tree_widget::{Tree, TreeItem};
use rayon::prelude::*;

use crate::app::app::App;
use crate::app::files::theme::THEME;

impl<'a> App<'a> {
    pub(super) fn render_collections(&mut self, frame: &mut Frame, rect: Rect) {
        let items: Vec<TreeItem<'a, usize>> = self.collections
            .par_iter()
            .enumerate()
            .map(|(collection_index, request)| {
                request.to_tree_item(collection_index)
            })
            .collect();

        
        let tree_items = self.collections_tree.items.clone();
        
        let collections_tree = Tree::new(&tree_items)
            .unwrap()
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .node_closed_symbol("")
            .node_no_children_symbol("")
            .block(
                Block::default()
                    .title("Collections")
                    .borders(Borders::ALL)
                    .fg(THEME.read().ui.main_foreground_color)
            );

        self.collections_tree.items = items;

        frame.render_stateful_widget(
            collections_tree,
            rect,
            &mut self.collections_tree.state
        );
    }
}