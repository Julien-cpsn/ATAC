use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum RequestTabs {
    #[default]
    #[strum(to_string = "Params")]
    Params,
    #[strum(to_string = "Auth")]
    Auth,
    #[strum(to_string = "Headers")]
    Headers,
    #[strum(to_string = "Body")]
    Body
}

#[warn(dead_code)]
pub fn next_request_tab(current_tab: RequestTabs) -> RequestTabs {
    match current_tab {
        RequestTabs::Params => RequestTabs::Auth,
        RequestTabs::Auth => RequestTabs::Headers,
        RequestTabs::Headers => RequestTabs::Body,
        RequestTabs::Body => RequestTabs::Params
    }
}
