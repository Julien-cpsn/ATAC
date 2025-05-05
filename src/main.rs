extern crate core;

use std::fmt::Display;
use std::io::{Result, stdout};
use std::process::exit;

use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::style::Stylize;
use ratatui::crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
pub use ratatui::backend::Backend;

use crate::app::app::App;
use crate::app::startup::startup::AppMode;
use crate::cli::args::ARGS;

mod app;
mod models;
mod cli;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
  
    let mut app = App::new();
    let app_mode = app.startup();

    let should_run_tui = match app_mode {
        AppMode::CLI(command) => {
            app
                .handle_command(command)
                .await;
            
            ARGS.should_run_tui
        },
        AppMode::TUI => true,
    };
    
    if should_run_tui {
        run_tui(&mut app).await?
    }

    Ok(())
}

async fn run_tui<'a>(app: &mut App<'a>) -> Result<()> {
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    app
        .prepare_terminal()
        .chain_hook()
        .run(terminal).await?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    
    Ok(())
}

pub fn panic_error<T>(message: T) -> ! where T: Display {
    println!("{error}:\n\t{message}", error = "Error".red().bold());
    exit(1);
}
