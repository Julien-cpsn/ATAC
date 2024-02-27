use std::path::PathBuf;
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Main application directory, containing JSON collections files, the atac.toml config file and the atac.log file
    #[arg(short, long)]
    pub directory: PathBuf,

    #[command(subcommand)]
    pub command: Option<Command>,

    /// Avoid saving data to the collection file
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Used to import a collection file such as Postman
    Import(ImportArgs)
}

#[derive(Debug, clap::Args)]
pub struct ImportArgs {
    /// A file to import, only Postman v2.1 JSON collection for now
    pub path: PathBuf,

    /// Max depth at which import should stop creating nested collections and only get the deeper requests
    #[arg(long)]
    pub max_depth: Option<u16>,
}

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}