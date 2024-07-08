extern crate core;

use std::fmt::Display;
use std::io::{Result, stdout};
use std::process::exit;

use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::style::Stylize;
use ratatui::crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
pub use ratatui::backend::Backend;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::app::app::App;
use crate::app::startup::startup::AppMode;

mod app;
mod models;
mod cli;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    
    let mut app = App::new();
    let app_mode = app.startup();

    match app_mode {
        AppMode::CLI(app, command) => {
            app
                .handle_command(command)
                .await;
        },
        AppMode::TUI(app) => {
            app
                .prepare_terminal()
                .chain_hook()
                .run(terminal).await?;

            stdout().execute(LeaveAlternateScreen)?;
            disable_raw_mode()?;
        }
    }

    Ok(())
}

pub fn panic_error<T>(message: T) -> ! where T: Display {
    println!("{error}:\n\t{message}", error = "Error".red().bold());
    exit(1);
}
