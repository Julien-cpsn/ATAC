use strum::Display;
use crate::app::app::App;
use crate::app::app_states::AppState::*;
use crate::app::request_ui::param_tabs::RequestParamsTabs;
use crate::request::auth::Auth;
use crate::request::body::ContentType;

#[derive(Copy, Clone, PartialEq, Default, Display)]
pub enum AppState {
    #[default]
    #[strum(to_string = "Main menu")]
    Normal,

    #[strum(to_string = "Request menu")]
    SelectedRequest,

    #[strum(to_string = "Editing request URL")]
    EditingRequestUrl,

    #[strum(to_string = "Creating new request")]
    CreatingNewRequest,

    #[strum(to_string = "Editing request auth username")]
    EditingRequestAuthUsername,

    #[strum(to_string = "Editing request auth password")]
    EditingRequestAuthPassword,

    #[strum(to_string = "Editing request body")]
    EditingRequestBody
}

impl App<'_> {
    pub fn get_available_keys(&self) -> String {
        match self.state {
            Normal => String::from("q or ^c ↑ ↓ ← → n d"),

            SelectedRequest => {
                let selected_request_index = self.collection.selected.unwrap();
                let selected_request = &self.collection.items[selected_request_index];

                let mut base_keys = String::from("Esc Space Tab ^(U)rl ^(B)ody");

                let additional_keys = match self.request_param_tab {
                    RequestParamsTabs::Params => None,
                    RequestParamsTabs::Auth => match selected_request.auth {
                        Auth::NoAuth => None,
                        Auth::BasicAuth(_, _) => Some("↑ ↓ Enter"),
                        Auth::BearerToken(_) => Some("Enter"),
                    },
                    RequestParamsTabs::Headers => None,
                    RequestParamsTabs::Body => match selected_request.body {
                        ContentType::NoBody => None,
                        ContentType::Raw(_) | ContentType::JSON(_) | ContentType::XML(_) | ContentType::HTML(_) => Some("Enter"),
                    },
                    RequestParamsTabs::Cookies => None,
                };

                if let Some(additional_keys_str) = additional_keys {
                    base_keys += &format!(" | {additional_keys_str}");
                }

                base_keys
            },

            CreatingNewRequest => String::from("Esc Enter ← → copy paste"),

            EditingRequestUrl => String::from("Esc Enter ← → copy paste"),

            EditingRequestAuthUsername | EditingRequestAuthPassword => String::from("Esc Enter ← → copy paste"),

            EditingRequestBody => String::from("Esc Enter Tab ^(s)ave ↑ ↓ ← → copy paste"),
        }
    }
}