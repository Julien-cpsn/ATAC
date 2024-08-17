use clap::Subcommand;
use crate::cli::commands::key::KeyCommand;

#[derive(Subcommand, Debug, Clone)]
pub enum KeyValueCommand {
    #[command(flatten)]
    Key(KeyCommand),
    
    /// Toggle a key value pair
    Toggle {
        /// Key to get the value
        key: String,
        
        /// Choose the state to apply 
        state: Option<bool>
    },

    /// Print all the key value pairs
    All
}