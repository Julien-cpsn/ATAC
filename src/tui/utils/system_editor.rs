use std::io::{Result, stdout};

use ratatui::prelude::*;
use tui_textarea::TextArea;
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen};
use ratatui::crossterm::ExecutableCommand;
use edit::{Builder, edit_with_builder};

use crate::tui::utils::stateful::text_input::TextInput;


pub fn run(terminal: &mut Terminal<impl Backend>, default_text: &str, file_extension: &str) -> Result<String> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    let mut file = Builder::new();
    let file = file.suffix(file_extension);
    let content = edit_with_builder(default_text, file);

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    terminal.clear()?;
    Ok(content?)
}

pub fn run_and_replace_textarea(terminal: &mut Terminal<impl Backend>, content: &mut TextArea, file_extension: &str) -> Result<()> {
    let text = &content.lines().join("\n");
    let file = run(terminal, &text, file_extension)?;

    *content = file.lines().collect();

    Ok(())
}

pub fn run_and_replace_textinput(terminal: &mut Terminal<impl Backend>, content: &mut TextInput, file_extension: &str) -> Result<()> {
    let file = run(terminal, &content.text, file_extension)?;

    content.set_input(file);

    Ok(())
}
