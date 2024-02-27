use tui_tree_widget::{TreeItem, TreeState};

#[derive(Default)]
pub struct StatefulTree<'a> {
    pub state: TreeState<usize>,
    pub items: Vec<TreeItem<'a, usize>>,
    pub selected: Option<(usize, usize)>
}

impl StatefulTree<'_> {
    pub fn up(&mut self) {
        self.state.key_up(&self.items);
    }

    pub fn down(&mut self) {
        self.state.key_down(&self.items);
    }

    pub fn set_selected(&mut self) {
        self.selected = Some((self.state.selected()[0], self.state.selected()[1]));
    }

    pub fn set_unselected(&mut self) {
        self.selected = None;
    }
}