use std::path::PathBuf;

#[derive(clap::Args, Debug, Clone)]
pub struct CompletionsCommand {
    /// The shell type. E.g. Bash, Powershell, Fish, Zsh
    pub shell: String,

    /// The path of the directory where to generate the file. If empty, will generate the file in the current folder
    #[clap(value_hint = clap::ValueHint::FilePath)]
    pub output_directory: Option<PathBuf>,
}