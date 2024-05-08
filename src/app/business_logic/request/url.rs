use regex::Regex;
use crate::app::app::App;
use crate::models::request::KeyValue;

impl App<'_> {
    pub fn modify_request_url(&mut self, url: String, collection_index: usize, request_index: usize) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));
        
        {
            let mut selected_request = local_selected_request.write();

            let url_parts = url.split_once("?");

            let final_url: String;
            let query_params: &str;

            if let Some((url, found_query_params)) = url_parts {
                final_url = url.to_string();
                query_params = found_query_params;
            } else {
                final_url = url;
                query_params = "";
            }


            let mut new_params_to_add: Vec<KeyValue> = vec![];
            let mut existing_params_found_indexes: Vec<usize> = vec![];

            let query_params_pattern = Regex::new(r"(&?([^=]+)=([^&]+))").unwrap();

            for (_, [_, param_name, value]) in query_params_pattern.captures_iter(query_params).map(|c| c.extract()) {
                let mut url_param_found = false;

                for (index, existing_param) in selected_request.params.iter_mut().enumerate() {
                    if param_name == existing_param.data.0 && existing_param.enabled {
                        existing_param.data.1 = value.to_string();
                        url_param_found = true;
                        existing_params_found_indexes.push(index);
                    }
                }

                if !url_param_found {
                    let new_param = KeyValue {
                        enabled: true,
                        data: (param_name.to_string(), value.to_string()),
                    };

                    new_params_to_add.push(new_param);
                }
            }

            let param_indexes = selected_request.params.len();

            for param_index in 0..param_indexes {
                if !existing_params_found_indexes.contains(&param_index) {
                    selected_request.params.remove(param_index);
                }
            }

            for new_param in new_params_to_add {
                selected_request.params.push(new_param);
            }

            selected_request.url = final_url;
        }

        self.save_collection_to_file(collection_index);
    }
}