use crate::app::app::App;
use crate::models::request::KeyValue;

impl App<'_> {
    pub fn modify_request_query_param(&mut self, input_text: String, column: usize, row: usize, collection_index: usize, request_index: usize) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            match column {
                0 => selected_request.params[row].data.0 = input_text.clone(),
                1 => selected_request.params[row].data.1 = input_text.clone(),
                _ => {}
            };
        }

        self.save_collection_to_file(collection_index);
    }


    pub fn create_new_query_param(&mut self, key: String, value: String, collection_index: usize, request_index: usize) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            selected_request.params.push(KeyValue {
                enabled: true,
                data: (key, value)
            });
        }

        self.save_collection_to_file(collection_index);
    }

    pub fn delete_query_param(&mut self, row: usize, collection_index: usize, request_index: usize) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();
            selected_request.params.remove(row);
        }

        self.save_collection_to_file(collection_index);
    }

    pub fn toggle_query_param(&mut self, row: usize, collection_index: usize, request_index: usize) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();
            selected_request.params[row].enabled = !selected_request.params[row].enabled;
        }

        self.save_collection_to_file(collection_index);
    }
}