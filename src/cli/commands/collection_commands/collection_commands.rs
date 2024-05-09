use clap::Subcommand;
use crate::cli::commands::request_commands::send::SendCommand;
use crate::cli::utils::arguments_validators::collection_validator;

#[derive(clap::Args, Debug, Clone)]
pub struct CollectionCommand {
    /// e.g. my_collection/my_request, my_collection
    #[arg(value_parser = collection_validator)]
    pub collection: String,

    #[command(subcommand)]
    pub collection_subcommand: Option<CollectionSubcommand>,

    /// Name of the environment to use
    #[arg(long, global = true)]
    pub env: Option<String>
}

#[derive(Subcommand, Debug, Clone)]
pub enum CollectionSubcommand {
    /// Rename a collection
    Rename {
        /// New collection name
        new_collection_name: String
    },
    
    /// Send a request
    Send(SendCommand),
}