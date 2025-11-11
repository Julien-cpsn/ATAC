#[derive(Default)]
pub struct ValidationPopup {
    pub state: bool,
    pub message: String,
}

impl ValidationPopup {
    pub fn change_state(&mut self) {
        self.state = !self.state
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }
}
