use tracing::info;

use crate::app::app::App;
use crate::app::business_logic::key_value::find_key;
use crate::models::request::KeyValue;

impl App<'_> {
    pub fn find_header(&mut self, collection_index: usize, request_index: usize, key: &str) -> anyhow::Result<usize> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));
        let selected_request = local_selected_request.read();

        find_key(&selected_request.headers, key)
    }
    
    pub fn modify_request_header(&mut self, collection_index: usize, request_index: usize, value: String, column: usize, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            let header_type = match column {
                0 => "key",
                1 => "value",
                _ => ""
            };
            
            info!("Header {header_type} set to \"{value}\"");

            match column {
                0 => selected_request.headers[row].data.0 = value.clone(),
                1 => selected_request.headers[row].data.1 = value.clone(),
                _ => {}
            };
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn create_new_header(&mut self, collection_index: usize, request_index: usize, key: String, value: String) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            info!("Key \"{key}\" with value \"{value}\" added to the headers");

            selected_request.headers.push(KeyValue {
                enabled: true,
                data: (key, value)
            });
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn delete_header(&mut self, collection_index: usize, request_index: usize, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            info!("Header deleted");
            
            selected_request.headers.remove(row);
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn toggle_header(&mut self, collection_index: usize, request_index: usize, state: Option<bool>, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));
        
        {
            let mut selected_request = local_selected_request.write();
            
            let new_state = match state {
                None => {
                    let state = !selected_request.headers[row].enabled;
                    // Better user feedback
                    println!("{state}");
                    state
                },
                Some(state) => state
            };

            info!("Header state set to \"{new_state}\"");

            selected_request.headers[row].enabled = new_state;
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn duplicate_header(&mut self, collection_index: usize, request_index: usize, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            info!("Header duplicated");
            
            let header = selected_request.headers[row].clone();
            selected_request.headers.insert(row, header);
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }
}