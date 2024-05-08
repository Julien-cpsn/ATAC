use crate::app::app::App;

impl App<'_> {
    /// Reset selection if headers are provided, either set it to none
    pub fn update_headers_selection(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        match selected_request.headers.is_empty() {
            false => {
                self.headers_table.selection = Some((0, 0));
                self.headers_table.left_state.select(Some(0));
                self.headers_table.right_state.select(Some(0));
            },
            true => {
                self.headers_table.selection = None;
                self.headers_table.left_state.select(None);
                self.headers_table.right_state.select(None);
            }
        }
    }

    pub fn tui_modify_request_header(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();

        let selection = self.headers_table.selection.unwrap();
        let input_text = self.headers_table.selection_text_input.text.clone();

        self.modify_request_header(input_text, selection.1, selection.0, selected_request_index.0, selected_request_index.1);

        self.select_request_state();
    }

    pub fn tui_create_new_header(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();

        self.create_new_header(String::from("header"), String::from("value"), selected_request_index.0, selected_request_index.1);

        self.update_headers_selection();
        self.update_inputs();
    }

    pub fn tui_delete_header(&mut self) {
        if self.headers_table.rows.is_empty() || self.headers_table.selection.is_none() {
            return;
        }

        let selection = self.headers_table.selection.unwrap();
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.delete_header(selection.0, selected_request_index.0, selected_request_index.1);
        
        self.update_headers_selection();
        self.update_inputs();
    }

    pub fn tui_toggle_header(&mut self) {
        if self.headers_table.rows.is_empty() || self.headers_table.selection.is_none() {
            return;
        }

        let row = self.headers_table.selection.unwrap().0;
        let selected_request_index = &self.collections_tree.selected.unwrap();
        
        self.toggle_header(row, selected_request_index.0, selected_request_index.1);
        
        self.update_inputs();
    }
}