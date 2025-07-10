use crate::models::settings::Setting;

#[derive(Default)]
pub struct SettingsPopup {
    pub settings: Vec<(String, Setting)>,
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

    pub fn toggle_setting_left(&mut self) {
        match self.settings[self.selection].1 {
            Setting::Bool(_) => self.settings[self.selection].1 = Setting::Bool(false),
            Setting::U32(u32) => match u32 > 100 {
                true => self.settings[self.selection].1 = Setting::U32(u32 - 100),
                false => self.settings[self.selection].1 = Setting::U32(100),
            }
        }
    }

    pub fn toggle_setting_right(&mut self) {
        match self.settings[self.selection].1 {
            Setting::Bool(_) => self.settings[self.selection].1 = Setting::Bool(true),
            Setting::U32(u32) => match u32 < 100000 {
                true => self.settings[self.selection].1 = Setting::U32(u32 + 100),
                false => self.settings[self.selection].1 = Setting::U32(100000),
            }
        }
    }
}
