use crate::cli::commands::request_commands::new::NewRequestCommand;
use crate::cli::commands::request_commands::send::SendCommand;

#[derive(clap::Args, Debug, Clone)]
pub struct TryCommand {
    #[command(flatten)]
    pub new_request_command: NewRequestCommand,

    #[command(flatten)]
    pub send_command: SendCommand
}