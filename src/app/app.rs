use std::fs::{File, OpenOptions};
use ratatui::backend::Backend;
use ratatui::Terminal;
use tui_textarea::TextArea;
use crate::app::app_states::AppState;
use crate::app::request_ui::param_tabs::RequestParamsTabs;
use crate::app::request_ui::result_tabs::RequestResultTabs;
use crate::app::request_ui::views::RequestView;
use crate::request::request::{Request};
use crate::utils::stateful_list::StatefulList;
use crate::utils::stateful_scrollbar::StatefulScrollbar;
use crate::utils::stateful_custom_table::{StatefulCustomTable};
use crate::utils::text_input::TextInput;
use crate::utils::text_input_selection::TextInputSelection;

pub struct App<'a> {
    pub should_quit: bool,
    pub state: AppState,

    pub log_file: File,

    pub collection: StatefulList<Request>,

    pub request_view: RequestView,
    pub request_param_tab: RequestParamsTabs,
    pub request_result_tab: RequestResultTabs,

    pub new_request_input: TextInput,

    pub url_text_input: TextInput,

    pub request_param_table: StatefulCustomTable,

    pub auth_text_input_selection: TextInputSelection,
    pub auth_basic_username_text_input: TextInput,
    pub auth_basic_password_text_input: TextInput,
    pub auth_bearer_token_text_input: TextInput,

    pub body_text_area: TextArea<'a>,

    pub result_scrollbar: StatefulScrollbar
}

impl App<'_> {
    pub fn new<'a>(requests: Vec<Request>) -> App<'a> {
        App {
            should_quit: false,
            state: AppState::Normal,

            log_file: OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open("atac.log")
                .expect("Could not open file"),

            collection: StatefulList {
                state: Default::default(),
                items: requests,
                selected: None,
                last_selected: None,
            },

            request_view: RequestView::Normal,

            request_param_tab: RequestParamsTabs::Params,
            request_result_tab: RequestResultTabs::Body,

            new_request_input: TextInput::default(),

            url_text_input: TextInput::default(),

            request_param_table: StatefulCustomTable::default(),

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