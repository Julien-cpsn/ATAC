use std::env;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap::builder::Styles;
use clap_complete::Generator;
use lazy_static::lazy_static;
use nestify::nest;

use crate::cli::completions::CompletionsCommand;
use crate::cli::import::ImportCommand;
use crate::cli::send::SendCommand;
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

nest! {
    #[derive(Subcommand, Debug, Clone)]
    pub enum Command {
        /// Send a request
        Send(SendCommand),

        /// Import a collection or request from other file formats (Postman v2.1.0, cURL)
        Import(ImportCommand),
        
        /// Create a completion file
        Completions(CompletionsCommand)
    }
}

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
            Some(arg_directory) => arg_directory,
            // If no directory was provided with the CLI
            None => match env::var("ATAC_MAIN_DIR") {
                // If the ATAC_MAIN_DIR environment variable exists
                Ok(env_directory) => PathBuf::from(env_directory),
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