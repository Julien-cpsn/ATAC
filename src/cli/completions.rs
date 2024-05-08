#[derive(clap::Args, Debug, Clone)]
pub struct CompletionsCommand {
    /// The shell type. E.g. Bash, Powershell, Fish
    pub shell: String,
}