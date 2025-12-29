#[derive(Default)]
pub struct ValidationPopup {
    pub state: bool,
}

impl ValidationPopup {
    pub fn change_state(&mut self) {
        self.state = !self.state
    }
}
