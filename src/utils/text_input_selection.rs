#[derive(Default)]
pub struct TextInputSelection {
    pub selected: usize,
    pub max_selection: usize,
}

impl TextInputSelection {
    pub fn previous(&mut self) {
        if self.selected as i16 - 1 >= 0 {
            self.selected -= 1;
        }
        else {
            self.selected = self.max_selection - 1;
        }
    }

    pub fn next(&mut self) {
        if self.selected + 1 < self.max_selection {
            self.selected += 1;
        }
        else {
            self.selected = 0;
        }
    }
}