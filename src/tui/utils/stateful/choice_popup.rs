#[derive(Default)]
pub struct ChoicePopup {
    pub choices: Vec<String>,
    pub selection: usize
}

impl ChoicePopup {
    pub fn next(&mut self) {
        if self.selection + 1 < self.choices.len() {
            self.selection += 1;
        }
        else {
            self.selection = 0;
        }
    }

    pub fn previous(&mut self) {
        if self.selection as isize - 1 >= 0 {
            self.selection -= 1;
        }
        else {
            self.selection = self.choices.len() - 1;
        }
    }
}
