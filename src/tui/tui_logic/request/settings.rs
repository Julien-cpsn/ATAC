use crate::app::app::App;

impl App<'_> {
    pub fn modify_request_settings(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            selected_request.settings.update_from_vec(&self.request_settings_popup.settings)
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }
}