use tracing::{info};

use crate::app::app::App;
use crate::cli::commands::request_commands::setting::RequestSettingName;

impl App<'_> {
    pub fn cli_print_request_settings(&mut self, collection_index: usize, request_index: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.write();

            for (setting, state) in selected_request.settings.to_vec() {
                println!("{setting}: {state}");
            }
        }

        Ok(())
    }
    
    pub fn cli_modify_request_setting(&mut self, collection_index: usize, request_index: usize, setting_name: &RequestSettingName, new_state: &bool) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            info!("Setting \"{}\" set to \"{}\"", setting_name, new_state);

            match setting_name {
                RequestSettingName::Proxy => selected_request.settings.use_config_proxy = *new_state,
                RequestSettingName::Redirects => selected_request.settings.allow_redirects= *new_state,
                RequestSettingName::Cookies => selected_request.settings.store_received_cookies= *new_state,
                RequestSettingName::Pretty => selected_request.settings.pretty_print_response_content= *new_state,
            };
        }

        self.save_collection_to_file(collection_index);

        Ok(())
    }
    
    pub fn cli_print_request_setting(&mut self, collection_index: usize, request_index: usize, setting_name: &RequestSettingName) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.write();
            
            let setting = match setting_name {
                RequestSettingName::Proxy => selected_request.settings.use_config_proxy,
                RequestSettingName::Redirects => selected_request.settings.allow_redirects,
                RequestSettingName::Cookies => selected_request.settings.store_received_cookies,
                RequestSettingName::Pretty => selected_request.settings.pretty_print_response_content,
            };
            
            println!("{setting}")
        }
        
        Ok(())
    }
}