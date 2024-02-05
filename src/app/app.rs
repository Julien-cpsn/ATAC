use ratatui::backend::Backend;
use ratatui::Terminal;
use reqwest::Method;
use strum::Display;
use tui_textarea::TextArea;
use crate::app::tabs::tabs::RequestTabs;
use crate::request::request::Request;
use crate::utils::stateful_list::StatefulList;
use crate::utils::stateful_scrollbar::StatefulScrollbar;
use crate::utils::text_input::TextInput;

pub struct App<'a> {
    pub should_quit: bool,
    pub state: AppState,

    pub collection: StatefulList<Request<'a>>,

    pub request_tab: RequestTabs,

    pub new_request_input: TextInput,
    pub url_text_input: TextInput,
    pub body_text_area: TextArea<'a>,

    pub result_scrollbar: StatefulScrollbar
}

#[derive(Default, Display)]
pub enum AppState {
    #[default]
    #[strum(to_string = "Main menu")]
    Normal,
    #[strum(to_string = "Editing request URL")]
    EditingUrl,
    #[strum(to_string = "Creating new request")]
    CreatingNewRequest,
    #[strum(to_string = "Editing request body")]
    EditingBody
}

impl App<'_> {
    pub fn new<'a>() -> App<'a> {
        let items = vec![
            Request {
                name: "Add User",
                url: "http://127.0.0.1:8080/api/add_user",
                method: Method::POST,
                body: Some(String::from(r#"{
    "json": 32
}"#)),
                result: None,
            },
            Request {
                name: "Get User",
                url: "http://127.0.0.1:8080/api/get_user",
                method: Method::GET,
                body: None,
                result: None,
            },
            Request {
                name: "Rust Homepage",
                url: "https://www.rust-lang.org",
                method: Method::GET,
                body: None,
                result: None,
            },
        ];

        App {
            should_quit: false,
            state: AppState::Normal,
            collection: StatefulList {
                state: Default::default(),
                items,
                selected: None,
                last_selected: None,
            },
            request_tab: Default::default(),
            new_request_input: TextInput {
                text: String::new(),
                cursor_position: 0,
            },
            url_text_input: TextInput {
                text: String::new(),
                cursor_position: 0,
            },
            body_text_area: TextArea::default(),
            result_scrollbar: StatefulScrollbar {
                scroll: 0,
                max_scroll: 0,
                state: Default::default(),
            }
        }
    }

    pub async fn run(&mut self, mut terminal: Terminal<impl Backend>) -> std::io::Result<()> {
        while !self.should_quit {
            self.draw(&mut terminal)?;
            self.should_quit = self.handle_events().await?;
        }

        Ok(())
    }
}