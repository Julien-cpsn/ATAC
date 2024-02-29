use crate::app::app::App;

impl App<'_> {
    pub fn next_environment(&mut self) {
        if self.selected_environment + 1 < self.environments.len() {
            self.selected_environment += 1;
        }
        else {
            self.selected_environment = 0;
        }
    }
}