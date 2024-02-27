mod app;
mod request;
mod utils;

use std::io::{stdout, Result};
use crossterm::{ExecutableCommand};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
pub use ratatui::backend::{Backend};
use ratatui::backend::CrosstermBackend;
use ratatui::{Terminal};
use crate::app::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    App::new()
        .startup()
        .prepare_terminal()
        .chain_hook()
        .run(terminal).await?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
