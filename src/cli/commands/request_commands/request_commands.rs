use clap::Subcommand;
use crate::cli::commands::request_commands::method::MethodCommand;
use crate::cli::commands::request_commands::send::SendCommand;
use crate::cli::utils::arguments_validators::collection_and_request_validator;

#[derive(clap::Args, Debug, Clone)]
pub struct RequestCommand {
    #[command(subcommand)]
    pub request_subcommand: RequestSubcommand,

    /// Name of the environment to use
    #[arg(long, global = true)]
    pub env: Option<String>
}

#[derive(Subcommand, Debug, Clone)]
pub enum RequestSubcommand {
    /// Describe a request
    Info {
        /// e.g. my_collection/my_request, my_collection
        #[arg(value_parser = collection_and_request_validator)]
        collection_and_request: (String, String),
    },

    /// Get or set a request method
    Method {
        /// e.g. my_collection/my_request, my_collection
        #[arg(value_parser = collection_and_request_validator)]
        collection_and_request: (String, String),

        #[command(subcommand)]
        subcommand: MethodCommand
    },

    /// Send a request
    Send {
        /// e.g. my_collection/my_request, my_collection
        #[arg(value_parser = collection_and_request_validator)]
        collection_and_request: (String, String),

        #[clap(flatten)]
        subcommand: SendCommand
    },
}