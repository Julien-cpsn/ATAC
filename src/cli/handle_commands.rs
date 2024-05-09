use crate::app::app::App;
use crate::cli::args::Command;
use crate::cli::args::Command::*;
use crate::cli::cli_logic::completions::generate_completions;
use crate::cli::commands::collection_commands::collection_commands::{CollectionCommand, CollectionSubcommand};
use crate::cli::commands::import::ImportType;
use crate::cli::commands::request_commands::request_commands::{RequestCommand, RequestSubcommand};
use crate::panic_error;

impl App<'_> {
    pub async fn handle_command(&mut self, command: Command) {
        let result = match &command {
            Collection(collection_command) => self.handle_collection_command(collection_command).await,
            
            Request(request_command) => self.handle_request_command(request_command).await,
            
            List { request_names } => self.list_collections(*request_names),
            
            New { collection_name } => self.new_collection(collection_name.clone()), 
            
            Delete { collection_name } => self.cli_delete_collection(collection_name),
            
            Import(import_command) => match &import_command.import_type {
                ImportType::Postman(postman_import) => self.import_postman_collection(postman_import),
                ImportType::Curl(curl_import) => self.import_curl_file(curl_import)
            },
            
            Completions(completions_command) => generate_completions(completions_command),
        };

        if let Err(error) = result {
            panic_error(error.to_string());
        }
    }

    async fn handle_collection_command(&mut self, collection_command: &CollectionCommand) -> anyhow::Result<()> {
        let collection_index = self.find_collection(&collection_command.collection)?;

        if let Some(environment_name) = &collection_command.env {
            let environment_index = self.find_environment(&environment_name)?;
            self.selected_environment = environment_index;
        }

        match &collection_command.collection_subcommand {
            None => self.describe_collection(collection_index),
            Some(collection_subcommand) => match collection_subcommand {
                CollectionSubcommand::Rename { new_collection_name } => self.rename_collection(new_collection_name.clone(), collection_index),
                CollectionSubcommand::Send(send_command) => self.send_collection_command(collection_index, send_command).await,
            }
        }
    }

    async fn handle_request_command(&mut self, request_command: &RequestCommand) -> anyhow::Result<()> {
        let (collection_index, request_index) = self.find_collection_and_request(&request_command.collection_and_request.0, &request_command.collection_and_request.1)?;

        if let Some(environment_name) = &request_command.env {
            let environment_index = self.find_environment(&environment_name)?;
            self.selected_environment = environment_index;
        }

        match &request_command.request_subcommand {
            None => self.describe_request(collection_index, request_index),
            Some(request_subcommand) => match request_subcommand {
                RequestSubcommand::Send(send_command) => self.send_request_command(collection_index, request_index, send_command).await
            }
        }
    }
}
