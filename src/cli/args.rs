use std::env;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap::builder::Styles;
use clap_verbosity_flag::Verbosity;
use lazy_static::lazy_static;

use crate::cli::commands::collection_commands::collection_commands::CollectionCommand;
use crate::cli::commands::completions::CompletionsCommand;
use crate::cli::commands::import::ImportCommand;
use crate::cli::commands::request_commands::request_commands::RequestCommand;
use crate::app::files::utils::expand_tilde;
use crate::cli::commands::env::EnvCommand;
use crate::cli::commands::man::ManCommand;
use crate::cli::commands::try_command::TryCommand;
use crate::panic_error;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = r#"ATAC is Arguably a Terminal API Client.
It is based on well-known clients such as Postman, Insomnia, or even Bruno, but inside your terminal without any specific graphical environment needed.
The philosophy of ATAC is to be free, account-less, and offline for now and forever."#,
    styles = Styles::styled()
)]
pub struct Args {
    /// Main application directory, containing JSON/YAML collections files, the atac.toml config file and the atac.log file
    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    pub directory: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,

    /// Avoid saving data to the collection and environment files
    #[arg(long, global = true, default_value_t = false, display_order = 99)]
    pub dry_run: bool,

    /// Avoid using ANSI format for log file/output
    #[arg(long, global = true, default_value_t = false)]
    pub no_ansi_log: bool,
    
    #[command(flatten)]
    pub verbose: Verbosity
}

/*
atac
  - collection
      - list
      - info
      - new
      - delete
      - rename
      - send (all requests from the collection)
  - request
      - info
      - new
      - delete
      - rename
      - url
      - method
      - params
      - auth
      - headers
      - body
      - scripts
      - send
      - settings
  - try
  - env
      - info
      - key
          - get
          - add
          - set
          - delete
          - rename
  - import
      - postman
      - curl
 - completions
      - bash, powershell, fish
 - man
*/

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Collection commands
    Collection(CollectionCommand),

    /// Request commands
    Request(RequestCommand),

    /// One-shot request sender
    Try(TryCommand),
    
    /// Environment commands
    Env(EnvCommand),

    /// Import a collection or a request from other file formats (Postman v2.1.0, cURL)
    Import(ImportCommand),

    /// Create a completion file
    Completions(CompletionsCommand),

    /// Generate ATAC man page
    Man(ManCommand),
}

#[derive(Debug)]
pub struct GlobalArgs {
    pub directory: PathBuf,
    pub command: Option<Command>,
    pub should_save: bool,
    pub verbosity: Verbosity,
    pub ansi_log: bool
}

lazy_static! {
    pub static ref ARGS: GlobalArgs = {
        let args = Args::parse();

        let completion_directory = if let Some(Command::Completions(ref completion_args)) = args.command {
            completion_args.output_directory.clone().or_else(|| {
                panic_error("No directory provided.");
            })
        } else {
            None
        };

        let directory = match args.directory.or(completion_directory) {
            // If a directory was provided with a CLI argument
            Some(arg_directory) => expand_tilde(arg_directory),
            // If no directory was provided with the CLI
            None => match env::var("ATAC_MAIN_DIR") {
                // If the ATAC_MAIN_DIR environment variable exists
                Ok(env_directory) => expand_tilde(PathBuf::from(env_directory)),
                Err(_) => panic_error("No directory provided, provide one either with `--directory <dir>` or via the environment variable `ATAC_MAIN_DIR`")
            }
        };

        GlobalArgs {
            directory,
            command: args.command,
            should_save: !args.dry_run,
            verbosity: args.verbose,
            ansi_log: !args.no_ansi_log
        }
    };
}
