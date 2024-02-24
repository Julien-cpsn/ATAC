mod app;
mod request;
mod utils;

use std::io::{stdout, Result};
use crossterm::{ExecutableCommand};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
pub use ratatui::backend::{Backend};
use ratatui::backend::CrosstermBackend;
use ratatui::{Terminal};
use crate::app::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    App::new()
        .chain_hook()
        .startup()
        .run(terminal).await?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
