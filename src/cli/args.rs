use std::env;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap::builder::Styles;
use clap_verbosity_flag::{InfoLevel, Verbosity};
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

    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

/*
* atac
*   - collection
*       - list (all collections)
*       - info
*           - "collection name"
*       - new
*           - "collection name"
*       - delete
*           - "collection name"
*       - rename
*           - "collection name" "new collection name"
*       - send (all request from collection)
*           - "collection name"
*   - request
*       - info
*           - "collection/request"
*       - new
*           - "collection/request"
*       - delete
*           - "collection/request"
*       - rename
*           - "collection/request" "new request name"
*       - send
*           - "collection/request"
*       - url
*           - "collection/request"
*               - get
*               - set "new url"
*       - method
*           - get
*           - set
                - GET, POST, PUT, PATCH, DELETE, HEAD, OPTION
*       - params
*       - auth
*       - headers
*       - body
*       - scripts
*       - settings
*           - "collection name/request name"
*   - env
*       - new
*       - delete
*           "env name"
*       - key
*           - add
*           - set
*           - delete
*               "key", "value"
*   - import
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

        // TODO: add env log with args.verbose.log_level_filter();
        
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
