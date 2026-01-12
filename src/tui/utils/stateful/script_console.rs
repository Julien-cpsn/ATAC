use crate::tui::utils::stateful::text_input::TextInput;

pub struct ScriptConsole {
    pub pre_request_text_area: TextInput,
    pub post_request_text_area: TextInput,
    pub script_selection: u16,
}

impl ScriptConsole {
    pub fn change_selection(&mut self) {
        self.script_selection = match self.script_selection {
            0 => 1,
            1 => 0,
            _ => 0
        }
    }
}