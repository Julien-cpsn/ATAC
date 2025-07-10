use anyhow::anyhow;
use tracing::{info};

use crate::app::app::App;
use crate::cli::commands::request_commands::setting::RequestSettingName;
use crate::models::settings::Setting;

impl App<'_> {
    pub fn cli_print_request_settings(&mut self, collection_index: usize, request_index: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.read();

            for (setting, state) in selected_request.settings.to_vec() {
                println!("{setting}: {state}");
            }
        }

        Ok(())
    }
    
    pub fn cli_modify_request_setting(&mut self, collection_index: usize, request_index: usize, setting_name: &RequestSettingName, new_value: &Setting) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            info!("Setting \"{}\" set to \"{}\"", setting_name, new_value);

            match new_value {
                Setting::Bool(bool) => match setting_name {
                    RequestSettingName::Proxy => selected_request.settings.use_config_proxy = Setting::Bool(*bool),
                    RequestSettingName::Redirects => selected_request.settings.allow_redirects = Setting::Bool(*bool),
                    RequestSettingName::Cookies => selected_request.settings.store_received_cookies = Setting::Bool(*bool),
                    RequestSettingName::Pretty => selected_request.settings.pretty_print_response_content = Setting::Bool(*bool),
                    _ => return Err(anyhow!(format!("The setting \"{}\" only takes positive int values", setting_name)))
                },
                Setting::U32(u32) => match setting_name {
                    RequestSettingName::Timeout => selected_request.settings.timeout = Setting::U32(*u32),
                    _ => return Err(anyhow!(format!("The setting \"{}\" only takes boolean values", setting_name)))
                }
            }
        }

        self.save_collection_to_file(collection_index);

        Ok(())
    }
    
    pub fn cli_print_request_setting(&mut self, collection_index: usize, request_index: usize, setting_name: &RequestSettingName) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.read();
            
            let setting = match setting_name {
                RequestSettingName::Proxy => &selected_request.settings.use_config_proxy,
                RequestSettingName::Timeout => &selected_request.settings.timeout,
                RequestSettingName::Redirects => &selected_request.settings.allow_redirects,
                RequestSettingName::Cookies => &selected_request.settings.store_received_cookies,
                RequestSettingName::Pretty => &selected_request.settings.pretty_print_response_content,
            };
            
            println!("{setting}")
        }
        
        Ok(())
    }
}