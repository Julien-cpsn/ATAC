use strum::Display;

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
    #[strum(to_string = "Editing request body")]
    EditingRequestBody
}

pub fn get_available_keys(app_state: AppState) -> String {
    match app_state {
        AppState::Normal => String::from("q ↑ ↓ ← → n d"),
        AppState::SelectedRequest => String::from("Esc Tab ^(U)rl ^(B)ody"),
        AppState::EditingRequestUrl => String::from("Esc Enter ← → copy paste"),
        AppState::CreatingNewRequest => String::from("Esc Enter ← → copy paste"),
        AppState::EditingRequestBody => String::from("Esc Enter Tab ^(s)ave ↑ ↓ ← → copy paste")
    }
}