use ratatui::prelude::Span;
use ratatui::style::Stylize;
use ratatui::text::Line;
use strum::Display;
use crate::app::app::App;
use crate::app::app_states::AppState::*;
use crate::app::ui::param_tabs::RequestParamsTabs;
use crate::request::auth::Auth;
use crate::request::body::ContentType;

#[derive(Copy, Clone, PartialEq, Default, Display)]
pub enum AppState {
    #[default]
    #[strum(to_string = "Main menu")]
    Normal,

    #[strum(to_string = "Request menu")]
    SelectedRequest,

    #[strum(to_string = "Creating new request")]
    CreatingNewRequest,

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

    #[strum(to_string = "Editing request body")]
    EditingRequestBody
}

const TEXT_INPUT_KEYS: &str = "Esc Enter ← → copy paste";

impl App<'_> {
    pub fn get_state_line(&self) -> Line {
        match self.state {
            Normal | CreatingNewRequest => Line::from(self.state.to_string().white().on_dark_gray()),
            _ => {
                let selected_request_index = self.collection.selected.unwrap();
                let selected_request = &self.collection.items[selected_request_index];

                if self.state == SelectedRequest {
                    Line::from(vec![
                        Span::raw("Request > ").dark_gray(),
                        Span::raw(&selected_request.name).white().on_dark_gray()
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
            Normal => String::from("(q)uit or ^c ↑ ↓ ← → or Enter (n)ew (d)elete"),

            SelectedRequest => {
                let selected_request_index = self.collection.selected.unwrap();
                let selected_request = &self.collection.items[selected_request_index];

                let mut base_keys = String::from("Esc ^Enter ^TAB (u)rl (m)ethod ^(p)arams ^(a)uth ^(b)ody");

                let additional_keys = match self.request_param_tab {
                    RequestParamsTabs::Params => match selected_request.params.is_empty() {
                        true => None,
                        false => Some("↑ ↓ ← → Enter")
                    },
                    RequestParamsTabs::Auth => match selected_request.auth {
                        Auth::NoAuth => None,
                        Auth::BasicAuth(_, _) => Some("↑ ↓ Enter"),
                        Auth::BearerToken(_) => Some("Enter"),
                    },
                    RequestParamsTabs::Headers => None,
                    RequestParamsTabs::Body => match selected_request.body {
                        ContentType::NoBody => None,
                        ContentType::Raw(_) | ContentType::Json(_) | ContentType::Xml(_) | ContentType::Html(_) => Some("Enter"),
                    },
                    RequestParamsTabs::Cookies => None,
                };

                if let Some(additional_keys_str) = additional_keys {
                    base_keys += &format!(" | {additional_keys_str}");
                }

                base_keys
            },

            CreatingNewRequest => String::from(TEXT_INPUT_KEYS),

            EditingRequestUrl => String::from(TEXT_INPUT_KEYS),

            EditingRequestParam => String::from(TEXT_INPUT_KEYS),

            EditingRequestAuthUsername | EditingRequestAuthPassword | EditingRequestAuthBearerToken => String::from(TEXT_INPUT_KEYS),

            EditingRequestBody => String::from("Esc Enter Tab ^(s)ave ↑ ↓ ← → copy paste"),
        }
    }
}