use ratatui::prelude::Span;
use ratatui::style::Stylize;
use ratatui::text::Line;
use strum::Display;
use crate::app::app::App;
use crate::app::app_states::AppState::*;
use crate::app::ui::param_tabs::param_tabs::RequestParamsTabs;
use crate::request::auth::Auth;
use crate::request::body::ContentType;

#[derive(Copy, Clone, PartialEq, Default, Display)]
pub enum AppState {
    #[default]
    #[strum(to_string = "Main menu")]
    Normal,

    /* Cookies */
    
    #[strum(to_string = "Displaying cookies")]
    DisplayingCookies,

    #[strum(to_string = "Editing cookies")]
    EditingCookies,
    
    /* Collections */
    
    #[strum(to_string = "Creating new collection")]
    CreatingNewCollection,

    #[strum(to_string = "Creating new request")]
    CreatingNewRequest,

    #[strum(to_string = "Deleting collection")]
    DeletingCollection,

    #[strum(to_string = "Deleting request")]
    DeletingRequest,

    #[strum(to_string = "Renaming collection")]
    RenamingCollection,

    #[strum(to_string = "Renaming request")]
    RenamingRequest,

    /* Request */

    #[strum(to_string = "Request menu")]
    SelectedRequest,
    
    #[strum(to_string = "Editing request URL")]
    EditingRequestUrl,

    #[strum(to_string = "Editing request param")]
    EditingRequestParam,

    #[strum(to_string = "Editing request auth username")]
    EditingRequestAuthUsername,

    #[strum(to_string = "Editing request auth password")]
    EditingRequestAuthPassword,

    #[strum(to_string = "Editing request auth bearer token")]
    EditingRequestAuthBearerToken,

    #[strum(to_string = "Editing request header")]
    EditingRequestHeader,

    #[strum(to_string = "Editing request body")]
    EditingRequestBodyTable,

    #[strum(to_string = "Editing request body")]
    EditingRequestBodyString,

    #[strum(to_string = "Editing request settings")]
    EditingRequestSettings
}

const TEXT_INPUT_KEYS: &str = "Esc Enter ← → copy paste";
const TEXT_AREA_INPUT_KEYS: &str = "Esc Enter Tab ^(s)ave ↑ ↓ ← → copy paste";
const VALIDATION_KEYS: &str = "Esc Enter ← →";
const TABLE_KEYS: &str = "↑ ↓ ← → Enter (n) (d) (t)";
const FULL_TABLE_KEYS: &str = "↑ ↓ ← → Enter (n)ew (d)elete (t)oggle";

impl App<'_> {
    pub fn get_state_line(&self) -> Line {
        match self.state {
            Normal | CreatingNewCollection | CreatingNewRequest => Line::from(self.state.to_string().white().on_dark_gray()),
            DisplayingCookies | EditingCookies => Line::from(self.state.to_string().white().on_dark_gray()),
            DeletingCollection | RenamingCollection => {
                let collection_index = self.collections_tree.state.selected()[0];
                let collection_name = &self.collections[collection_index].name;

                Line::from(vec![
                    Span::raw("Collection > ").dark_gray(),
                    Span::raw(format!("{} > ", collection_name)).dark_gray(),
                    Span::raw(self.state.to_string()).white().on_dark_gray()
                ])
            },
            DeletingRequest | RenamingRequest => {
                let selected_request_index = &self.collections_tree.state.selected();
                let selected_request = &self.collections[selected_request_index[0]].requests[selected_request_index[1]].read().unwrap();

                Line::from(vec![
                    Span::raw("Request > ").dark_gray(),
                    Span::raw(format!("{} > ", selected_request.name)).dark_gray(),
                    Span::raw(self.state.to_string()).white().on_dark_gray()
                ])
            },
            SelectedRequest |
            EditingRequestUrl |
            EditingRequestParam |
            EditingRequestAuthUsername | EditingRequestAuthPassword | EditingRequestAuthBearerToken  |
            EditingRequestHeader |
            EditingRequestBodyTable | EditingRequestBodyString |
            EditingRequestSettings 
            => {
                let local_selected_request = self.get_selected_request_as_local();
                let selected_request = local_selected_request.read().unwrap();

                if self.state == SelectedRequest {
                    Line::from(vec![
                        Span::raw("Request > ").dark_gray(),
                        Span::raw(selected_request.name.clone()).white().on_dark_gray()
                    ])
                }
                else {
                    Line::from(vec![
                        Span::raw("Request > ").dark_gray(),
                        Span::raw(format!("{} > ", selected_request.name)).dark_gray(),
                        Span::raw(self.state.to_string()).white().on_dark_gray()
                    ])
                }
            }
        }
    }

    pub fn get_available_keys(&self) -> String {
        match self.state {
            Normal => {
                let mut base_keys = String::from("q or ^c ↑ ↓ ← → Enter (h)elp (c) (r) (d) (n)");

                if !self.environments.is_empty() {
                    base_keys += " (e)";
                }

                base_keys
            },

            /* Cookies */
            
            DisplayingCookies => format!("Esc {FULL_TABLE_KEYS}"),

            EditingCookies => String::from(TEXT_INPUT_KEYS),

            /* Collections */
            
            CreatingNewCollection => String::from(TEXT_INPUT_KEYS),

            CreatingNewRequest => format!("{TEXT_INPUT_KEYS} ↑ ↓"),

            DeletingCollection => String::from(VALIDATION_KEYS),

            DeletingRequest => String::from(VALIDATION_KEYS),

            RenamingCollection => String::from(TEXT_INPUT_KEYS),

            RenamingRequest => String::from(TEXT_INPUT_KEYS),

            /* Request */

            SelectedRequest => {
                let local_selected_request = self.get_selected_request_as_local();
                let selected_request = local_selected_request.read().unwrap();

                let mut base_keys = String::from("Esc ^Enter ^TAB (h)elp (u) (m) ^(a) ^(b) (s)");

                if !self.environments.is_empty() {
                    base_keys += " (e)";
                }

                let additional_keys = match self.request_param_tab {
                    RequestParamsTabs::QueryParams => match selected_request.params.is_empty() {
                        true => Some("(n)"),
                        false => Some(TABLE_KEYS),
                    },
                    RequestParamsTabs::Auth => match selected_request.auth {
                        Auth::NoAuth => None,
                        Auth::BasicAuth(_, _) => Some("↑ ↓ Enter"),
                        Auth::BearerToken(_) => Some("Enter"),
                    },
                    RequestParamsTabs::Headers => match selected_request.headers.is_empty() {
                        true => Some("(n)"),
                        false => Some(TABLE_KEYS)
                    },
                    RequestParamsTabs::Body => match selected_request.body {
                        ContentType::NoBody => None,
                        ContentType::Multipart(_) | ContentType::Form(_) => match selected_request.body.get_form().unwrap().is_empty() {
                            true => Some("(n)"),
                            false => Some(TABLE_KEYS)
                        },
                        ContentType::Raw(_) | ContentType::Json(_) | ContentType::Xml(_) | ContentType::Html(_) => Some("Enter"),
                    },
                };

                if let Some(additional_keys_str) = additional_keys {
                    base_keys += &format!(" | {additional_keys_str}");
                }

                base_keys
            },
            
            EditingRequestUrl => String::from(TEXT_INPUT_KEYS),

            EditingRequestParam => String::from(TEXT_INPUT_KEYS),

            EditingRequestAuthUsername | EditingRequestAuthPassword | EditingRequestAuthBearerToken => String::from(TEXT_INPUT_KEYS),

            EditingRequestHeader => String::from(TEXT_INPUT_KEYS),

            EditingRequestBodyTable => String::from(TEXT_INPUT_KEYS),

            EditingRequestBodyString => String::from(TEXT_AREA_INPUT_KEYS),

            EditingRequestSettings => String::from("Esc Enter ↑ ↓ ← →"),
        }
    }

    pub fn get_full_available_keys(&self) -> String {
        match self.state {
            Normal => {
                let mut base_keys = String::from("(q)uit or ^c ↑ ↓ ← → Enter (c)ollection (r)equest (d)elete re(n)ame");

                if !self.environments.is_empty() {
                    base_keys += " (e)nv";
                }

                base_keys
            },

            /* Cookies */
            
            DisplayingCookies => format!("Esc {FULL_TABLE_KEYS}"),

            EditingCookies => String::from(TEXT_INPUT_KEYS),

            /* Collections */
            
            CreatingNewCollection => String::from(TEXT_INPUT_KEYS),

            CreatingNewRequest => format!("{TEXT_INPUT_KEYS} ↑ ↓"),

            DeletingCollection => String::from(VALIDATION_KEYS),

            DeletingRequest => String::from(VALIDATION_KEYS),

            RenamingCollection => String::from(TEXT_INPUT_KEYS),

            RenamingRequest => String::from(TEXT_INPUT_KEYS),

            /* Request */

            SelectedRequest => {
                let local_selected_request = self.get_selected_request_as_local();
                let selected_request = local_selected_request.read().unwrap();

                let mut base_keys = String::from("Esc ^Enter ^TAB (u)rl (m)ethod ^(a)uth ^(b)ody (s)ettings");

                if !self.environments.is_empty() {
                    base_keys += " (e)nv";
                }

                let additional_keys = match self.request_param_tab {
                    RequestParamsTabs::QueryParams => match selected_request.params.is_empty() {
                        true => Some("(n)ew param"),
                        false => Some(FULL_TABLE_KEYS),
                    },
                    RequestParamsTabs::Auth => match selected_request.auth {
                        Auth::NoAuth => None,
                        Auth::BasicAuth(_, _) => Some("↑ ↓ Enter"),
                        Auth::BearerToken(_) => Some("Enter"),
                    },
                    RequestParamsTabs::Headers => match selected_request.headers.is_empty() {
                        true => Some("(n)ew header"),
                        false => Some(FULL_TABLE_KEYS)
                    },
                    RequestParamsTabs::Body => match selected_request.body {
                        ContentType::NoBody => None,
                        ContentType::Multipart(_) | ContentType::Form(_) => match selected_request.body.get_form().unwrap().is_empty() {
                            true => Some("(n)"),
                            false => Some(FULL_TABLE_KEYS)
                        }
                        ContentType::Raw(_) | ContentType::Json(_) | ContentType::Xml(_) | ContentType::Html(_) => Some("Enter"),
                    },
                };

                if let Some(additional_keys_str) = additional_keys {
                    base_keys += &format!(" | {additional_keys_str}");
                }

                base_keys
            },
            
            EditingRequestUrl => String::from(TEXT_INPUT_KEYS),

            EditingRequestParam => String::from(TEXT_INPUT_KEYS),

            EditingRequestAuthUsername | EditingRequestAuthPassword | EditingRequestAuthBearerToken => String::from(TEXT_INPUT_KEYS),

            EditingRequestHeader => String::from(TEXT_INPUT_KEYS),

            EditingRequestBodyTable => String::from(TEXT_INPUT_KEYS),
            
            EditingRequestBodyString => String::from(TEXT_AREA_INPUT_KEYS),

            EditingRequestSettings => String::from("Esc Enter ↑ ↓ ← →"),
        }
    }
}