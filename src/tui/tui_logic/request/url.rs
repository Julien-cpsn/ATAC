use crate::app::app::App;

impl App<'_> {
    pub fn tui_modify_request_url(&mut self) {
        let input_text = self.url_text_input.to_string();

        let selected_request_index = &self.collections_tree.selected.unwrap();

        match self.modify_request_url(selected_request_index.0, selected_request_index.1, input_text) {
            Ok(_) => {}
            Err(_) => return
        }

        // In case new params were inputted or deleted
        self.tui_update_query_params_selection();
        self.select_request_state();
    }
}