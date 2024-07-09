use std::env;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap::builder::Styles;
use lazy_static::lazy_static;

use crate::cli::commands::collection_commands::collection_commands::CollectionCommand;
use crate::cli::commands::completions::CompletionsCommand;
use crate::cli::commands::import::ImportCommand;
use crate::cli::commands::request_commands::request_commands::RequestCommand;
use crate::app::files::utils::expand_tilde;
use crate::panic_error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, styles = Styles::styled())]
pub struct Args {
    /// Main application directory, containing JSON collections files, the atac.toml config file and the atac.log file
    #[arg(short, long)]
    pub directory: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,

    /// Avoid saving data to the collection and environment files
    #[arg(global = true, long, default_value_t = false)]
    pub dry_run: bool,
}

/*
* atac
*   - collection
*       - list (all collections)
*       - info
*       - new
*       - rename
*       - delete
*       - send
*           - "collection name"
*   - request
*       - list (all "collections/request")
*       - info
*       - new
*       - rename
*       - delete
*       - send
*       - method
*       - params
*       - auth
*       - headers
*       - body
*       - scripts
*       - settings
*           - "collection name/request name"
*  - import
*       - postman
*       - curl
*  - completions
*       - bash, powershell, fish
*/

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Collection commands
    Collection(CollectionCommand),

    /// Request commands
    Request(RequestCommand),

    /// List all collections
    List {
        /// Also print request names
        #[arg(long)]
        request_names: bool
    },

    /// Create a new collection
    New {
        /// Collection name
        collection_name: String
    },

    /// Delete a collection
    Delete {
        /// Collection name
        collection_name: String
    },

    /// Import a collection or request from other file formats (Postman v2.1.0, cURL)
    Import(ImportCommand),

    /// Create a completion file
    Completions(CompletionsCommand),
}

#[derive(Debug)]
pub struct ParsedArgs {
    pub directory: PathBuf,
    pub command: Option<Command>,
    pub should_save: bool
}

impl ParsedArgs {
    pub fn in_command(&self) -> bool {
        return self.command.is_some();
    }
}

lazy_static! {
    pub static ref ARGS: ParsedArgs = {
        let args = Args::parse();
        
        let directory = match args.directory {
            // If a directory was provided with a CLI argument
            Some(arg_directory) => expand_tilde(arg_directory),
            // If no directory was provided with the CLI
            None => match env::var("ATAC_MAIN_DIR") {
                // If the ATAC_MAIN_DIR environment variable exists
                Ok(env_directory) => expand_tilde(PathBuf::from(env_directory)),
                Err(_) => panic_error("No directory provided, provide one either with `--directory <dir>` or via the environment variable `ATAC_MAIN_DIR`")
            }
        };

        ParsedArgs {
            directory,
            command: args.command,
            should_save: !args.dry_run
        }
    };
}
