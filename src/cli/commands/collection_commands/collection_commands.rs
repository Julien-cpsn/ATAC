use clap::Subcommand;
use crate::cli::commands::request_commands::send::SendCommand;

#[derive(clap::Args, Debug, Clone)]
pub struct CollectionCommand {
    #[command(subcommand)]
    pub collection_subcommand: CollectionSubcommand,

    /// Name of the environment to use
    #[arg(long, global = true)]
    pub env: Option<String>
}

#[derive(Subcommand, Debug, Clone)]
pub enum CollectionSubcommand {
    /// List all collections
    List {
        /// Also print request names
        #[arg(long)]
        request_names: bool
    },

    /// Describe a collection
    Info {
        /// Collection name
        collection_name: String,

        /// Also print request names
        #[arg(long)]
        without_request_names: bool
    },

    /// Create a new collection
    New {
        /// Collection name
        collection_name: String
    },

    /// Delete a collection
    Delete {
        /// Collection name
        /// e.g. my_collection, "my collection"
        collection_name: String,
    },

    /// Rename a collection
    Rename {
        /// e.g. my_collection, "my collection"
        collection_name: String,

        /// New collection name
        new_collection_name: String
    },
    
    /// Send a request
    Send {
        /// e.g. my_collection, "my collection"
        collection_name: String,

        #[clap(flatten)]
        subcommand: SendCommand
    },
}