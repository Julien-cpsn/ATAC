use clap::Subcommand;
use crate::models::auth::Auth;

#[derive(Subcommand, Debug, Clone)]
pub enum AuthCommand {
    /// Print the current request auth method
    Get,
    /// Set the request auth method
    Set {
        #[command(subcommand)]
        auth_method: Auth
    }
}