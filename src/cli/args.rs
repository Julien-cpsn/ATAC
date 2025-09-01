use std::{env, fs};
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap::builder::Styles;
use clap_verbosity_flag::Verbosity;
use directories::ProjectDirs;
use lazy_static::lazy_static;
use regex::Regex;
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

    /// Will only allow collection files that matches the regex filter
    #[arg(long, global = true)]
    pub filter: Option<Regex>,

    /// Run TUI after command
    #[arg(long, global = true, default_value_t = false)]
    pub tui: bool,

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
      - openapi
 - completions
      - bash, powershell, fish, zsh
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

    /// Import a collection or a request from other file formats (Postman v2.1.0, cURL, OpenAPI)
    Import(ImportCommand),

    /// Create a completion file
    Completions(CompletionsCommand),

    /// Generate ATAC man page
    Man(ManCommand),
}

lazy_static! {
    pub static ref ARGS: GlobalArgs = {
        let args = Args::parse();
        
        let (directory, should_parse_directory) = match &args.command {
            // CLI
            Some(command) => match command.clone() {
                // Commands that take an output dir
                Command::Completions(CompletionsCommand { output_directory, .. }) | Command::Man(ManCommand { output_directory, .. }) => (output_directory, false),
                // Commands that use no dir at all
                Command::Try(_) => (None, false),
                // Commands that use the app dir
                _ => (Some(choose_app_directory(args.directory)), true)
            },
            // TUI
            None => (Some(choose_app_directory(args.directory)), true) 
        };

        GlobalArgs {
            directory,
            command: args.command,
            collection_filter: args.filter,
            should_run_tui: args.tui,
            should_save: !args.dry_run,
            should_parse_directory,
            verbosity: args.verbose,
            ansi_log: !args.no_ansi_log
        }
    };
}

fn choose_app_directory(path_buf: Option<PathBuf>) -> PathBuf {
    match path_buf {
        // If a directory was provided with the CLI argument
        Some(directory) => expand_tilde(directory),
        
        // If no directory was provided with the CLI
        None => match env::var("ATAC_MAIN_DIR") {
            // If the ATAC_MAIN_DIR environment variable exists
            Ok(env_directory) => expand_tilde(PathBuf::from(env_directory)),
            
            // No ATAC_MAIN_DIR env variable
            Err(_) => match ProjectDirs::from("com", "Julien-cpsn", "ATAC") {
                Some(project_dir) => {
                    let config_dir = project_dir.config_dir();
                    
                    if !config_dir.exists() {
                        fs::create_dir_all(config_dir).expect(&format!("Could not recursively create folder \"{}\"", config_dir.display()));
                    }
                    
                    config_dir.to_path_buf()
                },
                None => panic_error("No directory provided, provide one either with `--directory <dir>` or via the environment variable `ATAC_MAIN_DIR`")
            }
        }
    }
}

#[derive(Debug)]
pub struct GlobalArgs {
    pub directory: Option<PathBuf>,
    pub command: Option<Command>,
    pub collection_filter: Option<Regex>,
    pub should_run_tui: bool,
    pub should_save: bool,
    pub should_parse_directory: bool,
    pub verbosity: Verbosity,
    pub ansi_log: bool
}
