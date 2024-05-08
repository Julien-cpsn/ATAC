use crate::tui::utils::stateful::text_input::TextInput;

#[derive(Default)]
pub struct NewRequestPopup {
    pub selected_collection: usize,
    pub max_selection: usize,
    pub text_input: TextInput
}

impl NewRequestPopup {
    pub fn next_collection(&mut self) {
        if self.selected_collection + 1 < self.max_selection {
            self.selected_collection += 1;
        }
        else {
            self.selected_collection = 0;
        }
    }

    pub fn previous_collection(&mut self) {
        if self.selected_collection as isize - 1 >= 0 {
            self.selected_collection -= 1;
        }
        else {
            self.selected_collection = self.max_selection - 1;
        }
    }
}