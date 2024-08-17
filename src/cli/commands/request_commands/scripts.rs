use clap::Subcommand;
use crate::models::scripts::ScriptType;

#[derive(Subcommand, Debug, Clone)]
pub enum ScriptsCommand {
    /// Print the current pre- or post-request script
    Get {
        /// Pre or post
        script_type: ScriptType
    },
    /// Set a pre- or post-request script
    Set {
        /// Pre or post
        script_type: ScriptType,

        /// Script to set, leave empty to set it to none
        script: Option<String>
    }
}