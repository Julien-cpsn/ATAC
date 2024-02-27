use std::io::stdout;
use crossterm::ExecutableCommand;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use crate::app::app::App;

impl App<'_> {
    pub fn prepare_terminal(&mut self) -> &mut Self {
        enable_raw_mode().unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();

        self
    }
}