use clap::Subcommand;
use crate::cli::commands::request_commands::send::SendCommand;
use crate::cli::utils::arguments_validators::collection_and_request_validator;

#[derive(clap::Args, Debug, Clone)]
pub struct RequestCommand {
    /// e.g. my_collection/my_request, my_collection
    #[arg(value_parser = collection_and_request_validator)]
    pub collection_and_request: (String, String),

    #[command(subcommand)]
    pub request_subcommand: Option<RequestSubcommand>,

    /// Name of the environment to use
    #[arg(long, global = true)]
    pub env: Option<String>
}

#[derive(Subcommand, Debug, Clone)]
pub enum RequestSubcommand {
    /// Send a request
    Send(SendCommand),
}