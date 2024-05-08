use crate::app::app::App;
use crate::cli::args::Command;
use crate::cli::cli_logic::completions::generate_completions;
use crate::cli::import::ImportType;
use crate::panic_error;

impl App<'_> {
    pub async fn handle_command(&mut self, command: Command) {
        let result = match command {
            Command::Import(import_command) => match import_command.import_type {
                ImportType::Postman(postman_import) => self.import_postman_collection(postman_import),
                ImportType::Curl(curl_import) => self.import_curl_file(curl_import)
            },
            Command::Send(send_command) => self.send_request_command(send_command).await,
            Command::Completions(completions_command) => generate_completions(completions_command)
        };

        if let Err(error) = result {
            panic_error(error.to_string());
        }
    }
}
