use crate::app::app::App;
use crate::app::app_states::AppState;
use crate::app::ui::param_tabs::RequestParamsTabs;
use crate::request::body::ContentType;

impl App<'_> {
    pub fn normal_state(&mut self) {
        self.state = AppState::Normal;
    }

    pub fn create_new_request_state(&mut self) {
        self.new_request_popup.max_selection = self.collections.len();
        self.state = AppState::CreatingNewRequest;
    }

    pub fn select_request_state(&mut self) {
        self.state = AppState::SelectedRequest;
        self.update_inputs();
    }

    pub fn edit_request_url_state(&mut self) {
        self.state = AppState::EditingRequestUrl;
    }

    pub fn edit_request_param_state(&mut self) {
        self.state = AppState::EditingRequestParam;
        self.update_inputs();
    }

    pub fn edit_request_auth_username_state(&mut self) {
        self.state = AppState::EditingRequestAuthUsername;
    }

    pub fn edit_request_auth_password_state(&mut self) {
        self.state = AppState::EditingRequestAuthPassword;
    }

    pub fn edit_request_auth_bearer_token_state(&mut self) {
        self.state = AppState::EditingRequestAuthBearerToken;
    }

    pub fn edit_request_body_state(&mut self) {
        self.request_param_tab = RequestParamsTabs::Body;

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let selected_request = &self.collections[selected_request_index.0].requests[selected_request_index.1];

        match &selected_request.body {
            ContentType::NoBody => {},
            ContentType::Raw(_) | ContentType::Json(_) | ContentType::Xml(_) | ContentType::Html(_) => {
                self.state = AppState::EditingRequestBody;
            }
        }
    }
}