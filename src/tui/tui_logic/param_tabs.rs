use crate::app::app::App;
use crate::tui::ui::param_tabs::param_tabs::RequestParamsTabs;

impl App<'_> {
    pub fn tui_next_request_param_tab(&mut self) {
        self.request_param_tab = match self.request_param_tab {
            RequestParamsTabs::QueryParams => RequestParamsTabs::Auth,
            RequestParamsTabs::Auth => RequestParamsTabs::Headers,
            RequestParamsTabs::Headers => RequestParamsTabs::Body,
            RequestParamsTabs::Body => RequestParamsTabs::Scripts,
            RequestParamsTabs::Scripts => RequestParamsTabs::QueryParams
        };

        self.tui_load_a_request_param_tab();
    }

    pub fn tui_load_a_request_param_tab(&mut self) {
        match self.request_param_tab {
            RequestParamsTabs::QueryParams => self.tui_load_request_query_params_tab(),
            RequestParamsTabs::Auth => self.tui_load_request_auth_param_tab(),
            RequestParamsTabs::Headers => self.tui_load_request_headers_tab(),
            RequestParamsTabs::Body => self.tui_load_request_body_param_tab(),
            RequestParamsTabs::Scripts => {}
        }
    }

    pub fn tui_load_request_query_params_tab(&mut self) {
        self.tui_update_query_params_selection();

        self.request_param_tab = RequestParamsTabs::QueryParams;
        self.update_inputs();
    }

    pub fn tui_load_request_auth_param_tab(&mut self) {
        self.auth_text_input_selection.selected = 0;

        self.request_param_tab = RequestParamsTabs::Auth;
        self.update_inputs();
    }

    pub fn tui_load_request_headers_tab(&mut self) {
        self.tui_update_headers_selection();

        self.request_param_tab = RequestParamsTabs::Headers;
        self.update_inputs();
    }

    pub fn tui_load_request_body_param_tab(&mut self) {
        self.request_param_tab = RequestParamsTabs::Body;
        self.update_inputs();
    }
}