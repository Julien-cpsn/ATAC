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
use crate::request::method::Method;
use crate::request::request::Request;
use crate::utils::stateful_custom_table::Param;

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let requests = vec![
        Request {
            name: String::from("Check headers"),
            url: String::from("https://httpbin.org/headers"),
            ..Default::default()
        },
        Request {
            name: String::from("Test Bearer"),
            url: String::from("https://httpbin.org/bearer"),
            ..Default::default()
        },
        Request {
            name: String::from("Test Query Params"),
            url: String::from("https://httpbin.org/get"),
            params: vec![
                Param { enabled: true, data: ("test".to_string(), "3".to_string()) }
            ],
            ..Default::default()
        },
        Request {
            name: String::from("Test Post"),
            url: String::from("https://httpbin.org/post"),
            method: Method::POST,
            ..Default::default()
        },
        Request {
            name: String::from("Test Put"),
            url: String::from("https://httpbin.org/put"),
            method: Method::PUT,
            ..Default::default()
        },
        Request {
            name: String::from("Test Delete"),
            url: String::from("https://httpbin.org/delete"),
            method: Method::DELETE,
            ..Default::default()
        },
        Request {
            name: String::from("Test Patch"),
            url: String::from("https://httpbin.org/patch"),
            method: Method::PATCH,
            ..Default::default()
        },
        Request {
            name: String::from("Rust Homepage"),
            url: String::from("https://www.rust-lang.org"),
            ..Default::default()
        },
        Request {
            name: String::from("Google fr"),
            url: String::from("https://www.google.fr/"),
            ..Default::default()
        },
    ];

    App::new(requests).run(terminal).await?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
