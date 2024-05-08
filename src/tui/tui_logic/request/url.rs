use crate::app::app::App;

impl App<'_> {
    pub fn tui_modify_request_url(&mut self) {
        let input_text = self.url_text_input.text.clone();

        let selected_request_index = &self.collections_tree.selected.unwrap();

        self.modify_request_url(input_text, selected_request_index.0, selected_request_index.1);

        // In case new params were inputted or deleted
        self.update_query_params_selection();
        self.select_request_state();
    }
}