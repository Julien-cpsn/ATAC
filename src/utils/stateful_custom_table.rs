use ratatui::widgets::{ListState};
use crate::utils::text_input::TextInput;

#[derive(Default)]
pub struct StatefulCustomTable {
    pub left_state: ListState,
    pub right_state: ListState,
    /// (x, y)
    pub selection: Option<(usize, usize)>,
    pub rows: Vec<CustomTableItem>,
    pub param_selection_text_input: TextInput,
}

#[derive(Default, Clone)]
pub struct CustomTableItem {
    pub enabled: bool,
    pub data: (String, String)
}

impl StatefulCustomTable {
    fn decrement_x(&self, i: usize) -> usize {
        if i == 0 {
            self.rows.len() - 1
        } else {
            i - 1
        }
    }

    fn increment_x(&self, i: usize) -> usize {
        if i >= self.rows.len() - 1 {
            0
        } else {
            i + 1
        }
    }

    pub fn change_y(&mut self) {
        if self.rows.is_empty() || self.selection.is_none() {
            return;
        }

        match self.selection.unwrap() {
            (x, 0) => self.selection = Some((x, 1)),
            (x, 1) => self.selection = Some((x, 0)),
            (x, _) => self.selection = Some((x, 0))
        }

        let x = self.selection.unwrap().0;

        self.right_state.select(Some(x));
        self.left_state.select(Some(x));
    }

    pub fn up(&mut self) {
        if self.rows.is_empty() || self.selection.is_none() {
            return;
        }

        let x = match self.selection.unwrap() {
            (_, 0) => match self.left_state.selected() {
                None => 0,
                Some(i) => self.decrement_x(i)
            },
            (_, 1) => match self.right_state.selected() {
                None => 0,
                Some(i) => self.decrement_x(i)
            },
            (_, _) => 0
        };

        self.left_state.select(Some(x));
        self.right_state.select(Some(x));

        match self.selection.unwrap() {
            (_, y) => self.selection = Some((x, y))
        }
    }

    pub fn down(&mut self) {
        if self.rows.is_empty() || self.selection.is_none() {
            return;
        }

        let x = match self.selection.unwrap() {
            (_, 0) => match self.left_state.selected() {
                None => 0,
                Some(i) => self.increment_x(i)
            },
            (_, 1) => match self.right_state.selected() {
                None => 0,
                Some(i) => self.increment_x(i)
            },
            (_, _) => 0
        };

        self.left_state.select(Some(x));
        self.right_state.select(Some(x));

        match self.selection.unwrap() {
            (_, y) => self.selection = Some((x, y))
        }
    }

    pub fn is_selected(&self) -> bool {
        return self.selection.is_some();
    }
}