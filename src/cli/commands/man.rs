use std::path::PathBuf;

#[derive(clap::Args, Debug, Clone)]
pub struct ManCommand {
    /// The path of the directory where to generate the file.
    /// On most linux distros it's /usr/share/man/man1/.
    /// If empty, will generate the file in the current folder
    #[clap(value_hint = clap::ValueHint::FilePath)]
    pub output_directory: Option<PathBuf>,
}