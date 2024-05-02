use std::sync::Arc;
use parking_lot::RwLock;
use tui_textarea::TextArea;
use crate::utils::vim_emulation::Vim;

#[derive(Default)]
pub struct ScriptConsole<'a> {
    pub console_output: Arc<RwLock<Option<String>>>,
    pub pre_request_text_area: TextArea<'a>,
    pub post_request_text_area: TextArea<'a>,
    pub script_selection: u16,
    pub vim_emulation: Vim
}

impl ScriptConsole<'_> {
    pub fn change_selection(&mut self) {
        self.script_selection = match self.script_selection {
            0 => 1,
            1 => 0,
            _ => 0
        }
    }
}