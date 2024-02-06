use strum::{Display, EnumIter, FromRepr};
use crate::app::app::App;

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
    Body,
    #[strum(to_string = "Cookies")]
    Cookies
}

impl App<'_> {
    pub fn next_request_param_tab(&mut self) {
        self.request_param_tab = match self.request_param_tab {
            RequestParamsTabs::Params => RequestParamsTabs::Auth,
            RequestParamsTabs::Auth => RequestParamsTabs::Headers,
            RequestParamsTabs::Headers => RequestParamsTabs::Body,
            RequestParamsTabs::Body => RequestParamsTabs::Cookies,
            RequestParamsTabs::Cookies => RequestParamsTabs::Params
        };
    }
}