use clap::Subcommand;
use crate::models::protocol::http::method::Method;

#[derive(Subcommand, Debug, Clone)]
pub enum MethodCommand {
    /// Print the current request method
    Get,
    
    /// Set the request method
    Set {
        /// Method (GET, POST, QUERY, PUT, PATCH, DELETE, OPTIONS, HEAD, TRACE, CONNECT)
        new_method: Method
    }
}