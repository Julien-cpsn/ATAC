use std::io::stdout;
use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use tracing::trace;
use crate::app::app::App;

impl App<'_> {
    pub fn prepare_terminal(&mut self) -> &mut Self {
        trace!("Preparing terminal...");
        
        enable_raw_mode().unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();
        
        trace!("Terminal OK");

        self
    }
}