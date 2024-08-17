use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum UrlCommand {
    /// Print the current request URL
    Get,
    /// Set the request URL
    Set {
        #[clap(value_hint = clap::ValueHint::Url)]
        new_url: String
    }
}