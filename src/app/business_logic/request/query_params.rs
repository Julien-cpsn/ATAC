use tracing::info;

use crate::app::app::App;
use crate::app::business_logic::key_value::find_key;
use crate::models::request::KeyValue;

impl App<'_> {
    pub fn find_query_param(&mut self, collection_index: usize, request_index: usize, key: &str) -> anyhow::Result<usize> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));
        let selected_request = local_selected_request.read();
        
        find_key(&selected_request.params, key)
    }

    pub fn modify_request_query_param(&mut self, collection_index: usize, request_index: usize, value: String, column: usize, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            let query_param_type = match column {
                0 => "key",
                1 => "value",
                _ => ""
            };
            
            info!("Query param {query_param_type} set to \"{value}\"");

            match column {
                0 => selected_request.params[row].data.0 = value.clone(),
                1 => selected_request.params[row].data.1 = value.clone(),
                _ => {}
            };
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }


    pub fn create_new_query_param(&mut self, collection_index: usize, request_index: usize, key: String, value: String) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            info!("Key \"{key}\" with value \"{value}\" added to the query params");

            selected_request.params.push(KeyValue {
                enabled: true,
                data: (key, value)
            });
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn delete_query_param(&mut self, collection_index: usize, request_index: usize, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            info!("Query param deleted");

            selected_request.params.remove(row);
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn toggle_query_param(&mut self, collection_index: usize, request_index: usize, state: Option<bool>, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            let new_state = match state {
                None => {
                    let state = !selected_request.params[row].enabled;
                    // Better user feedback
                    println!("{state}");
                    state
                },
                Some(state) => state
            };

            info!("Query param state set to \"{new_state}\"");

            selected_request.params[row].enabled = new_state;
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn duplicate_query_param(&mut self, collection_index: usize, request_index: usize, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            info!("Query param duplicated");
            
            let query_param = selected_request.params[row].clone();
            selected_request.params.insert(row, query_param);
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }
}