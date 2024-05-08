#[derive(Default)]
pub struct SettingsPopup {
    pub settings: Vec<(String, bool)>,
    pub selection: usize,
}

impl SettingsPopup {
    pub fn next(&mut self) {
        if self.selection + 1 < self.settings.len() {
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
            self.selection = self.settings.len() - 1;
        }
    }

    pub fn toggle_setting(&mut self) {
        self.settings[self.selection].1 = !self.settings[self.selection].1
    }
}
