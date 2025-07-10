use clap::{Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use strum::Display;
use crate::models::settings::Setting;

#[derive(Subcommand, Debug, Clone)]
pub enum SettingsCommand {
    /// Print all the request settings
    All,
    /// Print the current request setting
    Get {
        /// Setting name to get status
        setting_name: RequestSettingName
    },
    /// Set the request method
    Set {
        /// Setting name to set status
        setting_name: RequestSettingName,

        /// New value to apply to the setting
        new_value: Setting
    }
}

#[derive(Debug, Copy, Clone, ValueEnum, Display, Serialize, Deserialize)]
pub enum RequestSettingName {
    /// Use config proxy
    Proxy,
    /// Allow redirects
    Redirects,
    /// Timeout (ms)
    Timeout,
    /// Store received cookies
    Cookies,
    /// Pretty print response content
    Pretty
}