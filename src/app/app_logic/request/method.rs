use crate::app::app::App;
use crate::request::method::next_method;

impl App<'_> {
    pub fn modify_request_method(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            let next_method = next_method(&selected_request.method);
            selected_request.method = next_method;
        }

        self.save_collection_to_file(selected_request_index.0);
    }
}