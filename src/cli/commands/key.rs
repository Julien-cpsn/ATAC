use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum KeyCommand {
    /// Get the value paired to a key
    Get {
        /// Key to get the value
        key: String,
    },
    /// Set the value paired to a key
    Set {
        /// Key to set the value
        key: String,
        
        /// Value to set
        value: String
    },
    /// Add a key value pair
    Add {
        /// Key to add
        key: String,

        /// Value to set
        value: String
    },
    /// Delete a key
    Delete {
        /// Key to delete
        key: String,
    },
    /// Rename a key
    Rename {
        /// Key to rename
        key: String,

        /// New key name
        new_key: String
    }
}