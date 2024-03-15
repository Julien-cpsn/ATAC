use crate::app::app::App;
use crate::request::request::KeyValue;

impl App<'_> {
    /// Reset selection if headers are provided, either set it to none
    pub fn update_headers_selection(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read().unwrap();

        match !selected_request.headers.is_empty() {
            true => {
                self.headers_table.selection = Some((0, 0));
                self.headers_table.left_state.select(Some(0));
                self.headers_table.right_state.select(Some(0));
            },
            false => {
                self.headers_table.selection = None;
                self.headers_table.left_state.select(None);
                self.headers_table.right_state.select(None);
            }
        }
    }

    pub fn modify_request_header(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            let selection = self.headers_table.selection.unwrap();
            let input_text = &self.headers_table.selection_text_input.text;

            match selection {
                (_, 0) => selected_request.headers[selection.0].data.0 = input_text.clone(),
                (_, 1) => selected_request.headers[selection.0].data.1 = input_text.clone(),
                (_, _) => {}
            };
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    pub fn create_new_header(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            selected_request.headers.push(KeyValue {
                enabled: true,
                data: (String::from("header"), String::from("value"))
            });
        }

        self.save_collection_to_file(selected_request_index.0);
        self.update_headers_selection();
        self.update_inputs();
    }

    pub fn delete_header(&mut self) {
        if self.headers_table.selection.is_none() {
            return;
        }

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            let selection = self.headers_table.selection.unwrap();
            selected_request.headers.remove(selection.0);
        }

        self.save_collection_to_file(selected_request_index.0);
        self.update_headers_selection();
        self.update_inputs();
    }

    pub fn toggle_header(&mut self) {
        if self.headers_table.rows.is_empty() {
            return;
        }

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write().unwrap();

            let row = self.headers_table.selection.unwrap().0;
            selected_request.headers[row].enabled = !selected_request.headers[row].enabled;
        }

        self.save_collection_to_file(selected_request_index.0);
        self.update_inputs();
    }
}