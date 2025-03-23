use clap::Subcommand;

use crate::cli::commands::key_value::KeyValueCommand;
use crate::cli::commands::request_commands::auth::AuthCommand;
use crate::cli::commands::request_commands::body::BodySubcommand;
use crate::cli::commands::request_commands::method::MethodCommand;
use crate::cli::commands::request_commands::new::NewRequestCommand;
use crate::cli::commands::request_commands::scripts::ScriptsCommand;
use crate::cli::commands::request_commands::send::SendCommand;
use crate::cli::commands::request_commands::setting::SettingsCommand;
use crate::cli::commands::request_commands::url::UrlCommand;
use crate::cli::utils::arguments_validators::collection_slash_request_validator;
use crate::models::export::ExportFormat;

#[derive(clap::Args, Debug, Clone)]
pub struct RequestCommand {
    #[command(subcommand)]
    pub request_subcommand: RequestSubcommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RequestSubcommand {
    /// Describe a request
    Info {
        /// Request to describe e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),
    },

    /// Create a request
    New {
        /// Request to create, e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),

        #[clap(flatten)]
        subcommand: NewRequestCommand
    },

    /// Delete a request
    Delete {
        /// Request to delete, e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),
    },

    /// Rename a request
    Rename {
        /// Request to delete, e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),

        /// New request name
        new_request_name: String
    },
    
    /// Get or set a request URL
    Url {
        /// e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),

        #[command(subcommand)]
        subcommand: UrlCommand
    },
    
    /// Get or set a request method
    Method {
        /// e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),

        #[command(subcommand)]
        subcommand: MethodCommand
    },

    /// Get, set, add, delete, rename or toggle a query param
    Params {
        /// e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),

        #[command(subcommand)]
        subcommand: KeyValueCommand
    },

    /// Get or set a request auth method
    Auth {
        /// e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),

        #[command(subcommand)]
        subcommand: AuthCommand
    },

    /// Get, set, add, delete, rename or toggle a header
    Header {
        /// e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),

        #[command(subcommand)]
        subcommand: KeyValueCommand
    },

    /// Get or set a request body
    Body {
        /// e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),

        #[command(subcommand)]
        subcommand: BodySubcommand
    },

    /// Get or set pre- and post-request scripts
    Scripts {
        /// e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),

        #[clap(subcommand)]
        subcommand: ScriptsCommand
    },

    /// Send a request
    Send {
        /// Request to send e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),

        #[clap(flatten)]
        subcommand: SendCommand
    },

    /// Get or set a request setting
    Settings {
        /// e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),

        #[command(subcommand)]
        subcommand: SettingsCommand
    },
    
    /// Export a request to another programming language
    Export {
        /// e.g. my_collection/my_request
        #[arg(value_parser = collection_slash_request_validator)]
        collection_slash_request: (String, String),

        /// Language to export to
        format: ExportFormat
    }
}