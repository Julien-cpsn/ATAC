use clap::Subcommand;
use crate::models::method::Method;

#[derive(Subcommand, Debug, Clone)]
pub enum MethodCommand {
    /// Print the current request method
    Get,
    /// Set the request method
    Set {
        /// Method (GET, POST, PUT, PATCH, DELETE, HEAD, OPTION)
        new_method: Method
    }
}