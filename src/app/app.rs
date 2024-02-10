use ratatui::backend::Backend;
use ratatui::Terminal;
use reqwest::Method;
use tui_textarea::TextArea;
use crate::app::app_states::AppState;
use crate::app::request_ui::param_tabs::RequestParamsTabs;
use crate::app::request_ui::result_tabs::RequestResultTabs;
use crate::app::request_ui::views::RequestView;
use crate::request::body::ContentType::{JSON};
use crate::request::request::{Request};
use crate::utils::stateful_list::StatefulList;
use crate::utils::stateful_scrollbar::StatefulScrollbar;
use crate::utils::text_input::TextInput;
use crate::utils::text_input_selection::TextInputSelection;

pub struct App<'a> {
    pub should_quit: bool,
    pub state: AppState,

    pub collection: StatefulList<Request<'a>>,

    pub request_view: RequestView,
    pub request_param_tab: RequestParamsTabs,
    pub request_result_tab: RequestResultTabs,

    pub new_request_input: TextInput,

    pub url_text_input: TextInput,

    pub auth_text_input_selection: TextInputSelection,
    pub auth_basic_username_text_input: TextInput,
    pub auth_basic_password_text_input: TextInput,
    pub auth_bearer_token_text_input: TextInput,

    pub body_text_area: TextArea<'a>,

    pub result_scrollbar: StatefulScrollbar
}

impl App<'_> {
    pub fn new<'a>() -> App<'a> {
        let items = vec![
            Request {
                name: "Check headers",
                url: "https://httpbin.org/headers",
                method: Method::GET,
                body: JSON(String::from(
r#"{
    "json": 32
}"#
                )),
                ..Default::default()
            },
            Request {
                name: "Get User",
                url: "http://127.0.0.1:8080/api/get_user",
                method: Method::GET,
                ..Default::default()
            },
            Request {
                name: "Rust Homepage",
                url: "https://www.rust-lang.org",
                method: Method::GET,
                ..Default::default()
            },
            Request {
                name: "Google fr",
                url: "https://www.google.fr/",
                method: Method::GET,
                ..Default::default()
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

            request_view: RequestView::Normal,

            request_param_tab: RequestParamsTabs::Params,
            request_result_tab: RequestResultTabs::Body,

            new_request_input: TextInput::default(),

            url_text_input: TextInput::default(),

            auth_text_input_selection: TextInputSelection::default(),
            auth_basic_username_text_input: TextInput::default(),
            auth_basic_password_text_input: TextInput::default(),
            auth_bearer_token_text_input: TextInput::default(),

            body_text_area: TextArea::default(),

            result_scrollbar: StatefulScrollbar::default()
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