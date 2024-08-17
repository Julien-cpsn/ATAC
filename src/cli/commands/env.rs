use clap::Subcommand;
use crate::cli::commands::key::KeyCommand;

#[derive(clap::Args, Debug, Clone)]
pub struct EnvCommand {
    #[command(subcommand)]
    pub env_subcommand: EnvSubcommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum EnvSubcommand {
    /// Describe an environment
    Info {
        /// e.g. my_env (from the file .env.my_env)
        env_name: String,
    },

    /// Add, get or set a key/value pair
    Key {
        /// e.g. my_env (from the file .env.my_env)
        env_name: String,
        
        #[command(subcommand)]
        subcommand: KeyCommand
    }
}