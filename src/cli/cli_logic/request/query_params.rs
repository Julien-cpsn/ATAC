use crate::app::app::App;
use crate::app::business_logic::key_value::print_key_value_vector;

impl App<'_> {
    pub fn cli_print_query_params(&mut self, collection_index: usize, request_index: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.read();
            print_key_value_vector(&selected_request.params, None);
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn cli_print_query_param(&mut self, collection_index: usize, request_index: usize, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.read();

            let value = &selected_request.params[row].data.1;

            println!("{value}")
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }
}