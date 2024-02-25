use std::path::PathBuf;
use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Main application directory, containing JSON collections files, the atac.toml config file and the atac.log file
    #[arg(short, long)]
    pub directory: PathBuf,

    /// Avoid saving data to the collection file
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// A file to import, only Postman v2.1 JSON collection for now
    #[arg(short, long)]
    pub import: Option<PathBuf>,
}

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}