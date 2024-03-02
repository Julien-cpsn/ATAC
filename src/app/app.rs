use std::fs::{File, OpenOptions};
use std::time::Duration;
use crossterm::terminal::{disable_raw_mode};
use ratatui::backend::Backend;
use ratatui::Terminal;
use throbber_widgets_tui::ThrobberState;
use tui_textarea::TextArea;
use crate::app::app_logic::new_request_popup::NewRequestPopup;
use crate::app::app_states::AppState;
use crate::app::files::config::Config;
use crate::app::ui::param_tabs::param_tabs::RequestParamsTabs;
use crate::app::ui::result_tabs::RequestResultTabs;
use crate::app::ui::views::RequestView;
use crate::app::startup::args::ARGS;
use crate::request::collection::Collection;
use crate::request::environment::Environment;
use crate::utils::stateful_scrollbar::StatefulScrollbar;
use crate::utils::stateful_custom_table::{StatefulCustomTable};
use crate::utils::stateful_tree::StatefulTree;
use crate::utils::text_input::TextInput;
use crate::utils::text_input_selection::TextInputSelection;
use crate::utils::validation_popup::ValidationPopup;

pub struct App<'a> {
    pub tick_rate: Duration,
    pub should_quit: bool,

    pub state: AppState,

    pub config: Config,

    pub log_file: Option<File>,

    pub environments: Vec<Environment>,
    pub selected_environment: usize,

    pub collections: Vec<Collection>,
    pub collections_tree: StatefulTree<'a>,

    pub request_view: RequestView,
    pub request_param_tab: RequestParamsTabs,
    pub request_result_tab: RequestResultTabs,

    pub new_collection_input: TextInput,
    pub new_request_popup: NewRequestPopup,

    pub delete_collection_popup: ValidationPopup,
    pub delete_request_popup: ValidationPopup,

    pub url_text_input: TextInput,

    pub query_params_table: StatefulCustomTable,

    pub auth_text_input_selection: TextInputSelection,
    pub auth_basic_username_text_input: TextInput,
    pub auth_basic_password_text_input: TextInput,
    pub auth_bearer_token_text_input: TextInput,

    pub headers_table: StatefulCustomTable,

    pub body_text_area: TextArea<'a>,

    pub result_throbber_state: ThrobberState,
    pub result_scrollbar: StatefulScrollbar
}

impl App<'_> {
    pub fn new<'a>() -> App<'a> {
        App {
            tick_rate: Duration::from_millis(250),
            should_quit: false,
            state: AppState::Normal,

            config: Config::default(),

            log_file: None,

            environments: vec![],
            selected_environment: 0,

            collections: vec![],
            collections_tree: StatefulTree::default(),

            request_view: RequestView::Normal,

            request_param_tab: RequestParamsTabs::QueryParams,
            request_result_tab: RequestResultTabs::Body,

            new_collection_input: TextInput::default(),
            new_request_popup: NewRequestPopup {
                selected_collection: 0,
                max_selection: 0,
                text_input: TextInput::default(),
            },

            delete_collection_popup: ValidationPopup::default(),
            delete_request_popup: ValidationPopup::default(),
            
            url_text_input: TextInput::default(),

            query_params_table: StatefulCustomTable::default(),

            auth_text_input_selection: TextInputSelection::default(),
            auth_basic_username_text_input: TextInput::default(),
            auth_basic_password_text_input: TextInput::default(),
            auth_bearer_token_text_input: TextInput::default(),

            headers_table: StatefulCustomTable::default(),

            body_text_area: TextArea::default(),

            result_throbber_state: ThrobberState::default(),
            result_scrollbar: StatefulScrollbar::default(),
        }
    }

    pub async fn run(&mut self, mut terminal: Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.clear()?;

        while !self.should_quit {
            self.draw(&mut terminal)?;
            self.handle_events().await;
        }

        Ok(())
    }

    pub fn chain_hook(&mut self) -> &mut Self {
        let original_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic| {
            disable_raw_mode().unwrap();
            original_hook(panic);
        }));

        self
    }
}