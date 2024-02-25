use crate::app::app::App;
use crate::app::app_states::AppState;
use crate::app::ui::param_tabs::param_tabs::RequestParamsTabs;
use crate::request::body::ContentType;

impl App<'_> {
    pub fn normal_state(&mut self) {
        self.state = AppState::Normal;
    }

    pub fn create_new_collection_state(&mut self) {
        self.state = AppState::CreatingNewCollection;
    }

    pub fn create_new_request_state(&mut self) {
        let collections_length = self.collections.len();

        // Cannot create a request if there is no collection
        if collections_length == 0 {
            return;
        }

        self.new_request_popup.max_selection = collections_length;
        self.state = AppState::CreatingNewRequest;
    }

    pub fn delete_collection_state(&mut self) {
        self.delete_collection_popup.state = false;
        self.state = AppState::DeletingCollection;
    }

    pub fn delete_request_state(&mut self) {
        self.delete_request_popup.state = false;
        self.state = AppState::DeletingRequest;
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

        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read().unwrap();

        match &selected_request.body {
            ContentType::NoBody => {},
            ContentType::Raw(_) | ContentType::Json(_) | ContentType::Xml(_) | ContentType::Html(_) => {
                self.state = AppState::EditingRequestBody;
            }
        }
    }
}