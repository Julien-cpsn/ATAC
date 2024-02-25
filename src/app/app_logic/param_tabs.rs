use crate::app::app::App;
use crate::app::ui::param_tabs::param_tabs::RequestParamsTabs;

impl App<'_> {
    pub fn next_request_param_tab(&mut self) {
        self.request_param_tab = match self.request_param_tab {
            RequestParamsTabs::QueryParams => RequestParamsTabs::Auth,
            RequestParamsTabs::Auth => RequestParamsTabs::Headers,
            RequestParamsTabs::Headers => RequestParamsTabs::Body,
            RequestParamsTabs::Body => RequestParamsTabs::Cookies,
            RequestParamsTabs::Cookies => RequestParamsTabs::QueryParams
        };

        self.load_a_request_param_tab();
    }

    pub fn load_a_request_param_tab(&mut self) {
        match self.request_param_tab {
            RequestParamsTabs::QueryParams => self.load_request_query_params_tab(),
            RequestParamsTabs::Auth => self.load_request_auth_param_tab(),
            RequestParamsTabs::Headers => {}
            RequestParamsTabs::Body => self.load_request_body_param_tab(),
            RequestParamsTabs::Cookies => {}
        }
    }

    pub fn load_request_query_params_tab(&mut self) {
        self.update_params_selection();

        self.request_param_tab = RequestParamsTabs::QueryParams;
        self.update_inputs();
    }

    pub fn load_request_auth_param_tab(&mut self) {
        self.auth_text_input_selection.selected = 0;

        self.request_param_tab = RequestParamsTabs::Auth;
        self.update_inputs();
    }

    pub fn load_request_body_param_tab(&mut self) {
        self.request_param_tab = RequestParamsTabs::Body;
        self.update_inputs();
    }
}