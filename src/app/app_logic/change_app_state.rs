use crate::app::app::App;
use crate::app::app_states::AppState;
use crate::app::request_ui::param_tabs::RequestParamsTabs;

impl App<'_> {
    pub fn normal_state(&mut self) {
        self.state = AppState::Normal;
    }

    pub fn create_new_request_state(&mut self) {
        self.state = AppState::CreatingNewRequest;
    }

    pub fn select_request_state(&mut self) {
        self.state = AppState::SelectedRequest;
    }

    pub fn edit_request_url_state(&mut self) {
        self.state = AppState::EditingRequestUrl;
    }

    pub fn edit_request_body_state(&mut self) {
        self.request_param_tab = RequestParamsTabs::Body;

        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        if selected_request.body.is_some() {
            self.state = AppState::EditingRequestBody;
        }
    }
}