use crate::app::app::App;
use crate::cli::args::Command;
use crate::cli::args::Command::*;
use crate::cli::cli_logic::completions::generate_completions;
use crate::cli::commands::collection_commands::collection_commands::{CollectionCommand, CollectionSubcommand};
use crate::cli::commands::import::ImportType;
use crate::cli::commands::request_commands::method::MethodCommand;
use crate::cli::commands::request_commands::request_commands::{RequestCommand, RequestSubcommand};
use crate::panic_error;

impl App<'_> {
    pub async fn handle_command(&mut self, command: Command) {
        let result = match &command {
            Collection(collection_command) => self.handle_collection_command(collection_command).await,
            
            Request(request_command) => self.handle_request_command(request_command).await,
            
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
        if let Some(environment_name) = &collection_command.env {
            let environment_index = self.find_environment(&environment_name)?;
            self.selected_environment = environment_index;
        }

        match &collection_command.collection_subcommand {
            CollectionSubcommand::Info { collection_name, without_request_names: with_request_names } => self.describe_collection(collection_name, *with_request_names),
            CollectionSubcommand::List { request_names: with_request_names } => self.list_collections(*with_request_names),
            CollectionSubcommand::New { collection_name } => self.new_collection(collection_name.clone()),
            CollectionSubcommand::Delete { collection_name } => self.cli_delete_collection(collection_name),
            CollectionSubcommand::Rename { collection_name, new_collection_name } => self.cli_rename_collection(collection_name, new_collection_name.clone()),
            CollectionSubcommand::Send { collection_name, subcommand } => self.send_collection_command(collection_name, subcommand).await,
        }
    }

    async fn handle_request_command(&mut self, request_command: &RequestCommand) -> anyhow::Result<()> {
        // Since all the request commands need the collection_and_request argument, it's preferable to parse it from here
        let (collection_index, request_index) = match &request_command.request_subcommand {
            RequestSubcommand::Info { collection_and_request } | RequestSubcommand::Send { collection_and_request, .. } | RequestSubcommand::Method { collection_and_request, .. } => self.find_collection_and_request(&collection_and_request.0, &collection_and_request.1)?
        };

        if let Some(environment_name) = &request_command.env {
            let environment_index = self.find_environment(&environment_name)?;
            self.selected_environment = environment_index;
        }

        match &request_command.request_subcommand {
            RequestSubcommand::Info { .. } => self.describe_request(collection_index, request_index),
            RequestSubcommand::Method { subcommand, .. } => match subcommand {
                MethodCommand::Get => self.print_request_method(collection_index, request_index),
                MethodCommand::Set { new_method } => self.modify_request_method(collection_index, request_index, new_method.clone())
            },
            RequestSubcommand::Send { subcommand, .. } => self.send_request_command(collection_index, request_index, subcommand).await
        }
    }
}
