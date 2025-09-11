#[derive(Default)]
pub struct ChoicePopup<T> {
    pub choices: Vec<T>,
    pub selection: usize
}

impl<T> ChoicePopup<T> {
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

    pub fn get_selection(&self) -> &T {
        &self.choices[self.selection]
    }
}
