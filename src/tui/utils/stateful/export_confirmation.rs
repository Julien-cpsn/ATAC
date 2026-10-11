#[derive(Default)]
pub struct ExportConfirmation {
    pub message: String,
}

impl ExportConfirmation {
    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }
}
