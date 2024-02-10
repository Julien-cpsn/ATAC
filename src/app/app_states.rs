use strum::Display;
use crate::app::app_states::AppState::*;

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

    #[strum(to_string = "Editing request auth")]
    EditingRequestAuth,

    #[strum(to_string = "Editing request auth username")]
    EditingRequestAuthUsername,

    #[strum(to_string = "Editing request auth password")]
    EditingRequestAuthPassword,

    #[strum(to_string = "Editing request body")]
    EditingRequestBody
}

pub fn get_available_keys(app_state: AppState) -> String {
    match app_state {
        Normal => String::from("q or ^c ↑ ↓ ← → n d"),

        SelectedRequest => String::from("Esc Tab ^(U)rl ^(B)ody"),
        CreatingNewRequest => String::from("Esc Enter ← → copy paste"),

        EditingRequestUrl => String::from("Esc Enter ← → copy paste"),

        EditingRequestAuth =>String::from("Esc Enter ↑ ↓"),
        EditingRequestAuthUsername => String::from("Esc Enter ← → copy paste"),
        EditingRequestAuthPassword => String::from("Esc Enter ← → copy paste"),

        EditingRequestBody => String::from("Esc Enter Tab ^(s)ave ↑ ↓ ← → copy paste"),
    }
}