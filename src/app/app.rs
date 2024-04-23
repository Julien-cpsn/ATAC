use std::fs::File;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crossterm::terminal::disable_raw_mode;
use ratatui::backend::Backend;
use ratatui::Terminal;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use throbber_widgets_tui::ThrobberState;
use tui_textarea::TextArea;

use crate::app::app_logic::new_request_popup::NewRequestPopup;
use crate::app::app_states::AppState;
use crate::app::files::config::Config;
use crate::app::ui::param_tabs::param_tabs::RequestParamsTabs;
use crate::app::ui::result_tabs::RequestResultTabs;
use crate::app::ui::views::RequestView;
use crate::request::collection::Collection;
use crate::request::environment::Environment;
use crate::utils::choice_popup::ChoicePopup;
use crate::utils::cookies_popup::CookiesPopup;
use crate::utils::help_popup::HelpPopup;
use crate::utils::settings_popup::SettingsPopup;
use crate::utils::stateful_custom_table::StatefulCustomTable;
use crate::utils::stateful_scrollbar::StatefulScrollbar;
use crate::utils::stateful_tree::StatefulTree;
use crate::utils::syntax_highlighting::SyntaxHighlighting;
use crate::utils::text_input::TextInput;
use crate::utils::text_input_selection::TextInputSelection;
use crate::utils::validation_popup::ValidationPopup;
use crate::utils::vim_emulation::{Vim, VimMode};

pub struct App<'a> {
    pub tick_rate: Duration,
    pub should_quit: bool,
    pub should_display_help: bool,

    pub state: AppState,

    pub config: Config,

    pub log_file: Option<File>,

    /* Help */

    pub help_popup: HelpPopup,

    /* Environments */
    
    pub environments: Vec<Environment>,
    pub selected_environment: usize,

    /* Cookies */
    
    pub cookies_popup: CookiesPopup,
    
    /* Collections */
    
    pub collections: Vec<Collection>,
    pub collections_tree: StatefulTree<'a>,

    pub request_view: RequestView,
    pub request_param_tab: RequestParamsTabs,
    pub request_result_tab: RequestResultTabs,

    pub creation_popup: ChoicePopup,

    pub new_collection_input: TextInput,
    pub rename_collection_input: TextInput,
    pub new_request_popup: NewRequestPopup,
    pub rename_request_input: TextInput,

    pub delete_collection_popup: ValidationPopup,
    pub delete_request_popup: ValidationPopup,

    /* Request */

    pub url_text_input: TextInput,

    pub query_params_table: StatefulCustomTable,

    pub auth_text_input_selection: TextInputSelection,
    pub auth_basic_username_text_input: TextInput,
    pub auth_basic_password_text_input: TextInput,
    pub auth_bearer_token_text_input: TextInput,

    pub headers_table: StatefulCustomTable,

    pub body_file_text_input: TextInput,
    pub body_form_table: StatefulCustomTable,
    pub body_text_area: TextArea<'a>,
    pub body_text_area_vim_emulation: Vim,

    pub request_settings_popup: SettingsPopup,

    pub result_throbber_state: ThrobberState,
    pub result_vertical_scrollbar: StatefulScrollbar,
    pub result_horizontal_scrollbar: StatefulScrollbar,

    /* Others */
    
    pub syntax_highlighting: SyntaxHighlighting
}

impl App<'_> {
    pub fn new<'a>() -> App<'a> {
        App {
            tick_rate: Duration::from_millis(250),
            should_quit: false,
            should_display_help: false,
            
            state: AppState::Normal,

            config: Config::default(),

            log_file: None,

            /* Help */

            help_popup: HelpPopup::default(),

            /* Environments */

            environments: vec![],
            selected_environment: 0,

            /* Cookies */

            cookies_popup: CookiesPopup::default(),
            
            /* Collections */
            
            collections: vec![],
            collections_tree: StatefulTree::default(),

            request_view: RequestView::Normal,

            request_param_tab: RequestParamsTabs::QueryParams,
            request_result_tab: RequestResultTabs::Body,

            creation_popup: ChoicePopup {
              choices: vec![String::from("Collection"), String::from("Request")],
              selection: 0
            },
            
            new_collection_input: TextInput::default(),
            rename_collection_input: TextInput::default(),
            new_request_popup: NewRequestPopup::default(),
            rename_request_input: TextInput::default(),

            delete_collection_popup: ValidationPopup::default(),
            delete_request_popup: ValidationPopup::default(),
            
            /* Request */
            
            url_text_input: TextInput::default(),

            query_params_table: StatefulCustomTable::default(),

            auth_text_input_selection: TextInputSelection::default(),
            auth_basic_username_text_input: TextInput::default(),
            auth_basic_password_text_input: TextInput::default(),
            auth_bearer_token_text_input: TextInput::default(),

            headers_table: StatefulCustomTable::default(),

            body_file_text_input: TextInput::default(),
            body_form_table: StatefulCustomTable::default(),
            body_text_area: TextArea::default(),
            body_text_area_vim_emulation: Vim::new(VimMode::Normal),


            request_settings_popup: SettingsPopup::default(),
            
            result_throbber_state: ThrobberState::default(),
            result_vertical_scrollbar: StatefulScrollbar::default(),
            result_horizontal_scrollbar: StatefulScrollbar::default(),

            /* Others */

            syntax_highlighting: SyntaxHighlighting {
                syntax_set: SyntaxSet::load_defaults_newlines(),
                theme_set: ThemeSet::load_defaults(),
                last_highlighted: Arc::new(RwLock::new(None)),
            },
        }
    }

    pub async fn run(&mut self, mut terminal: Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.clear()?;

        while !self.should_quit {
            self.update_current_available_events();
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