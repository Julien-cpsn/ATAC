use std::io::{Result, stdout};

use ratatui::prelude::*;
use tui_textarea::TextArea;
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen};
use ratatui::crossterm::ExecutableCommand;
use edit::{Builder, edit_with_builder};


pub fn run(terminal: &mut Terminal<impl Backend>, default_text: String, file_extention: &str) -> Result<String> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    let mut file = Builder::new();
    let file = file.suffix(file_extention);
    let content = edit_with_builder(default_text, file);

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    terminal.clear()?;
    Ok(content?)
}

pub fn run_and_replace_textarea(terminal: &mut Terminal<impl Backend>, content: &mut TextArea, file_extention: &str) -> Result<()> {
    let file = run(terminal, content.lines().join("\n"), file_extention)?;

    *content = file.lines().collect();

    Ok(())
}

