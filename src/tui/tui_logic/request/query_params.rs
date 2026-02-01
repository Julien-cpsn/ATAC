use crate::app::app::App;

impl App<'_> {
    /// Reset selection of if params are provided, either set it to none
    pub fn tui_update_query_params_selection(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        match selected_request.params.is_empty() {
            false => self.query_params_table.update_selection(Some((0, 0))),
            true => self.query_params_table.update_selection(None)
        }
    }

    pub fn tui_modify_request_query_param(&mut self) {
        let input_text = self.query_params_table.selection_text_input.to_string();
        let selected_request_index = &self.collections_tree.selected.unwrap();

        let selection = self.query_params_table.selection.unwrap();

        match self.modify_request_query_param(selected_request_index.0, selected_request_index.1, input_text, selection.1, selection.0) {
            Ok(_) => {}
            Err(_) => return
        }

        self.select_request_state();
    }


    pub fn tui_create_new_query_param(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();

        match self.create_new_query_param(selected_request_index.0, selected_request_index.1, String::from("param"), String::from("value")) {
            Ok(_) => {}
            Err(_) => return
        }

        self.tui_update_query_params_selection();
        self.update_inputs();
    }

    pub fn tui_delete_query_param(&mut self) {
        if self.query_params_table.rows.is_empty() || self.query_params_table.selection.is_none() {
            return;
        }

        let selection = self.query_params_table.selection.unwrap();
        let selected_request_index = &self.collections_tree.selected.unwrap();

        match self.delete_query_param(selected_request_index.0, selected_request_index.1, selection.0) {
            Ok(_) => {}
            Err(_) => return
        }

        self.tui_update_query_params_selection();
        self.update_inputs();
    }

    pub fn tui_toggle_query_param(&mut self) {
        if self.query_params_table.rows.is_empty() || self.query_params_table.selection.is_none() {
            return;
        }

        let row = self.query_params_table.selection.unwrap().0;
        let selected_request_index = &self.collections_tree.selected.unwrap();

        match self.toggle_query_param(selected_request_index.0, selected_request_index.1, None, row) {
            Ok(_) => {}
            Err(_) => return
        }

        self.update_inputs();
    }

    pub fn tui_duplicate_query_param(&mut self) {
        if self.query_params_table.rows.is_empty() || self.query_params_table.selection.is_none() {
            return;
        }

        let row = self.query_params_table.selection.unwrap().0;
        let selected_request_index = &self.collections_tree.selected.unwrap();

        match self.duplicate_query_param(selected_request_index.0, selected_request_index.1, row) {
            Ok(_) => {}
            Err(_) => return
        }

        self.update_inputs();
    }
}