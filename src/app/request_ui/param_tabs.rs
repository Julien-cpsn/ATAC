use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum RequestParamsTabs {
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

pub fn next_request_tab(current_tab: RequestParamsTabs) -> RequestParamsTabs {
    match current_tab {
        RequestParamsTabs::Params => RequestParamsTabs::Auth,
        RequestParamsTabs::Auth => RequestParamsTabs::Headers,
        RequestParamsTabs::Headers => RequestParamsTabs::Body,
        RequestParamsTabs::Body => RequestParamsTabs::Params
    }
}