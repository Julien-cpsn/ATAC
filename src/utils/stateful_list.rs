use ratatui::widgets::ListState;

#[allow(dead_code)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub selected: Option<usize>,
    pub last_selected: Option<usize>,
}

#[allow(dead_code)]
impl<T> StatefulList<T> {
    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn select(&mut self) {
        self.selected = match self.state.selected() {
            Some(i) => Some(i),
            None => None
        };
    }

    pub fn unselect(&mut self) {
        /*
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        *self.state.offset_mut() = offset;*/
        self.state.select(None);
        self.selected = None;
    }
}
