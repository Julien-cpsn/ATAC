use std::env;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use crate::{panic_error};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Main application directory, containing JSON collections files, the atac.toml config file and the atac.log file
    #[arg(short, long)]
    pub directory: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,

    /// Avoid saving data to the collection file
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Command {
    /// Used to import a collection file such as Postman, or a file containing a curl
    Import(ImportArgs),
}

#[derive(Debug, clap::Args, PartialEq)]
pub struct ImportArgs {
    /// A file to import, only Postman v2.1 JSON collection for now
    pub path: PathBuf,

    /// Max depth at which import should stop creating nested collections and only get the deeper requests
    #[arg(long)]
    pub max_depth: Option<u16>,
}

pub struct ParsedArgs {
    pub directory: PathBuf,
    pub is_directory_from_env: bool,
    pub command: Option<Command>,
    pub should_save: bool
}

lazy_static! {
    pub static ref ARGS: ParsedArgs = {
        let args = Args::parse();

        let (directory, is_directory_from_env) = match args.directory {
            // If a directory was provided with a CLI argument
            Some(arg_directory) => (arg_directory, false),
            // If no directory was provided with the CLI
            None => match env::var("ATAC_MAIN_DIR") {
                // If the ATAC_MAIN_DIR environment variable exists
                Ok(env_directory) => (PathBuf::from(env_directory), true),
                Err(_) => panic_error("No directory provided, provide one either with `--directory <dir>` or via the environment variable `ATAC_MAIN_DIR`")
            }
        };

        ParsedArgs {
            directory,
            is_directory_from_env,
            command: args.command,
            should_save: !args.dry_run
        }
    };
}