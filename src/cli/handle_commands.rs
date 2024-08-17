use crate::app::app::App;
use crate::cli::args::Command;
use crate::cli::args::Command::*;
use crate::cli::cli_logic::completions::generate_completions;
use crate::cli::cli_logic::man::generate_man_page;
use crate::cli::commands::collection_commands::collection_commands::{CollectionCommand, CollectionSubcommand};
use crate::cli::commands::env::{EnvCommand, EnvSubcommand};
use crate::cli::commands::key::KeyCommand;
use crate::cli::commands::import::ImportType;
use crate::cli::commands::key_value::KeyValueCommand;
use crate::cli::commands::request_commands::auth::{AuthCommand};
use crate::cli::commands::request_commands::body::BodySubcommand;
use crate::cli::commands::request_commands::method::MethodCommand;
use crate::cli::commands::request_commands::request_commands::{RequestCommand, RequestSubcommand};
use crate::cli::commands::request_commands::scripts::ScriptsCommand;
use crate::cli::commands::request_commands::setting::SettingsCommand;
use crate::cli::commands::request_commands::url::UrlCommand;
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

            Try(try_command) => self.try_request(&try_command.new_request_command, &try_command.send_command).await,

            Env(env_command) => self.handle_env_commands(env_command),

            Completions(completions_command) => generate_completions(completions_command),
            
            Man(man_command) => generate_man_page(man_command)
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
            CollectionSubcommand::Send { collection_name, subcommand } => self.cli_send_collection(collection_name, subcommand).await,
        }
    }

    async fn handle_request_command(&mut self, request_command: &RequestCommand) -> anyhow::Result<()> {
        // Since all the request commands need the collection_slash_request argument, it's preferable to parse it from here
        let (collection_index, request_index) = match &request_command.request_subcommand {
            RequestSubcommand::Info { collection_slash_request } | RequestSubcommand::Delete { collection_slash_request }| RequestSubcommand::Rename { collection_slash_request, .. } | RequestSubcommand::Url { collection_slash_request, .. } | RequestSubcommand::Method { collection_slash_request, .. } | RequestSubcommand::Params { collection_slash_request, .. } | RequestSubcommand::Auth { collection_slash_request, .. } | RequestSubcommand::Header { collection_slash_request, .. } | RequestSubcommand::Body { collection_slash_request, .. } | RequestSubcommand::Scripts { collection_slash_request, .. } | RequestSubcommand::Send { collection_slash_request, .. } | RequestSubcommand::Settings { collection_slash_request, .. } => self.find_collection_slash_request(&collection_slash_request.0, &collection_slash_request.1)?,
            // Specific case
            RequestSubcommand::New { collection_slash_request, subcommand } => return self.cli_new_request(collection_slash_request.clone(), subcommand.clone()),
        };

        match &request_command.request_subcommand {
            RequestSubcommand::Info { .. } => self.cli_describe_request(collection_index, request_index),
            RequestSubcommand::Delete { .. } => self.delete_request(collection_index, request_index),
            RequestSubcommand::Rename { new_request_name, .. } => self.rename_request(collection_index, request_index, new_request_name.clone()),
            RequestSubcommand::New { .. } => panic!("Should not happen"),
            RequestSubcommand::Url { subcommand, .. } => match subcommand {
                UrlCommand::Get => self.cli_print_request_url(collection_index, request_index),
                UrlCommand::Set { new_url } => self.modify_request_url(collection_index, request_index, new_url.clone())
            },
            RequestSubcommand::Method { subcommand, .. } => match subcommand {
                MethodCommand::Get => self.cli_print_request_method(collection_index, request_index),
                MethodCommand::Set { new_method } => self.modify_request_method(collection_index, request_index, new_method.clone())
            },
            RequestSubcommand::Params { subcommand, .. } => {
                let key = match subcommand {
                    KeyValueCommand::Key(key_command) => match key_command {
                        // Specific case
                        KeyCommand::Add { key, value } => return self.create_new_query_param(collection_index, request_index, key.clone(), value.clone()),
                        // Otherwise, get the key
                        KeyCommand::Get { key } | KeyCommand::Set { key, .. } | KeyCommand::Delete { key } | KeyCommand::Rename { key, .. } => key
                    },
                    KeyValueCommand::Toggle { key, .. } => key,
                    KeyValueCommand::All => return self.cli_print_query_params(collection_index, request_index),
                };

                let query_param_index = self.find_query_param(collection_index, request_index, key)?;

                match subcommand {
                    KeyValueCommand::Key(key_command) => match key_command {
                        KeyCommand::Get { .. } => self.cli_print_query_param(collection_index, request_index, query_param_index),
                        KeyCommand::Set { value, .. } => self.modify_request_query_param(collection_index, request_index, value.clone(), 1, query_param_index),
                        KeyCommand::Delete { .. } => self.delete_query_param(collection_index, request_index, query_param_index),
                        KeyCommand::Rename { new_key, .. } => self.modify_request_query_param(collection_index, request_index, new_key.clone(), 0, query_param_index),
                        _ => panic!("Should not happen")
                    },
                    KeyValueCommand::Toggle { state, .. } => self.toggle_query_param(collection_index, request_index, state.clone(), query_param_index),
                    _ => panic!("Should not happen")
                }
            },
            RequestSubcommand::Auth { subcommand, .. } => match subcommand {
                AuthCommand::Get => self.cli_print_request_auth(collection_index, request_index),
                AuthCommand::Set { auth_method } => self.modify_request_auth(collection_index, request_index, auth_method.to_owned())
            },
            RequestSubcommand::Header { subcommand, .. } => {
                let key = match subcommand {
                    KeyValueCommand::Key(key_command) => match key_command {
                        // Specific case
                        KeyCommand::Add { key, value } => return self.create_new_header(collection_index, request_index, key.clone(), value.clone()),
                        // Otherwise, get the key
                        KeyCommand::Get { key } | KeyCommand::Set { key, .. } | KeyCommand::Delete { key } | KeyCommand::Rename { key, .. } => key
                    },
                    KeyValueCommand::Toggle { key, .. } => key,
                    KeyValueCommand::All => return self.cli_print_headers(collection_index, request_index)
                };

                let header_index = self.find_header(collection_index, request_index, key)?;

                match subcommand {
                    KeyValueCommand::Key(key_command) => match key_command {
                        KeyCommand::Get { .. } => self.cli_print_header(collection_index, request_index, header_index),
                        KeyCommand::Set { value, .. } => self.modify_request_header(collection_index, request_index, value.clone(), 1, header_index),
                        KeyCommand::Delete { .. } => self.delete_header(collection_index, request_index, header_index),
                        KeyCommand::Rename { new_key, .. } => self.modify_request_header(collection_index, request_index, new_key.clone(), 0, header_index),
                        _ => panic!("Should not happen")
                    },
                    KeyValueCommand::Toggle { state, .. } => self.toggle_header(collection_index, request_index, state.clone(), header_index),
                    _ => panic!("Should not happen")
                }
            },
            RequestSubcommand::Body { subcommand, .. } => match subcommand {
                BodySubcommand::Get => self.cli_print_request_body(collection_index, request_index),
                BodySubcommand::Set { content_type } => self.modify_request_content_type(collection_index, request_index, content_type.to_content_type()),
                BodySubcommand::Key { subcommand } => {
                    let key = match subcommand {
                        KeyValueCommand::Key(key_command) => match key_command {
                            // Specific case
                            KeyCommand::Add { key, value } => return self.create_new_form_data(collection_index, request_index, key.clone(), value.clone()),
                            // Otherwise, get the key
                            KeyCommand::Get { key } | KeyCommand::Set { key, .. } | KeyCommand::Delete { key } | KeyCommand::Rename { key, .. } => key
                        },
                        KeyValueCommand::Toggle { key, .. } => key,
                        KeyValueCommand::All => return self.cli_print_all_form_data(collection_index, request_index)
                    };

                    let form_data_index = self.find_form_data(collection_index, request_index, key)?;
                    
                    match subcommand {
                        KeyValueCommand::Key(key_command) => match key_command {
                            KeyCommand::Get { .. } => self.cli_print_form_data(collection_index, request_index, form_data_index),
                            KeyCommand::Set { value, .. } => self.modify_request_form_data(collection_index, request_index, value.clone(), 1, form_data_index),
                            KeyCommand::Add { value, .. } => self.create_new_form_data(collection_index, request_index, key.clone(), value.clone()),
                            KeyCommand::Delete { .. } => self.delete_form_data(collection_index, request_index, form_data_index),
                            KeyCommand::Rename { new_key, .. } => self.modify_request_form_data(collection_index, request_index, new_key.clone(), 0, form_data_index),
                        },
                        KeyValueCommand::Toggle { state, .. } => self.toggle_form_data(collection_index, request_index, state.clone(), form_data_index),
                        _ => panic!("Should not happen")
                    }
                }
            },
            RequestSubcommand::Scripts { subcommand, .. } => match subcommand {
                ScriptsCommand::Get { script_type } => self.cli_print_request_script(collection_index, request_index, script_type),
                ScriptsCommand::Set { script_type, script } => self.modify_request_script(collection_index, request_index, script_type, script.clone())
            },
            RequestSubcommand::Send { subcommand, .. } => self.cli_send_request(collection_index, request_index, subcommand).await,
            RequestSubcommand::Settings { subcommand, .. } => match subcommand {
                SettingsCommand::All => self.cli_print_request_settings(collection_index, request_index),
                SettingsCommand::Get { setting_name } => self.cli_print_request_setting(collection_index, request_index, setting_name),
                SettingsCommand::Set { setting_name, new_state: new_status } => self.cli_modify_request_setting(collection_index, request_index, setting_name, new_status),
            }
        }
    }

    fn handle_env_commands(&mut self, env_command: &EnvCommand) -> anyhow::Result<()> {
        let env_index = match &env_command.env_subcommand {
            EnvSubcommand::Info { env_name } | EnvSubcommand::Key { env_name, .. }=> self.find_environment(env_name)?
        };

        match &env_command.env_subcommand {
            EnvSubcommand::Info { .. } => self.cli_describe_env(env_index),
            EnvSubcommand::Key { subcommand, .. } => match subcommand {
                KeyCommand::Get { key } => self.get_env_value(env_index, key),
                KeyCommand::Set { key, value } => self.set_env_value(env_index, key, value.clone()),
                KeyCommand::Add { key, value } => self.add_env_value(env_index, key.clone(), value.clone()),
                KeyCommand::Delete { key } => self.delete_env_key(env_index, key),
                KeyCommand::Rename { key, new_key } => self.rename_env_key(env_index, key, new_key),
            }
        }
    }
}
