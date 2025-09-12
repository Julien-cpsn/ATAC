use std::fs;
use std::sync::Arc;
use anyhow::anyhow;
use indexmap::IndexMap;
use parking_lot::RwLock;
use serde::Deserialize;
use thiserror::Error;
use uuid::Uuid;
use crate::app::app::App;
use crate::cli::args::ARGS;
use crate::cli::commands::import::PostmanEnvImport;
use crate::models::environment::Environment;

#[derive(Error, Debug)]
enum ImportPostmanEnvironmentError {
    #[error("Could not read Postman environment file\n\t{0}")]
    CouldNotReadFile(String),
    #[error("Could not parse Postman environment\n\t{0}")]
    CouldNotParsePostmanEnvironment(String),
}

#[derive(Deserialize)]
struct PostmanEnv {
    #[serde(rename = "id")]
    pub _id: Uuid,
    pub name: String,
    pub values: Vec<PostmanEnvVariable>,
    pub _postman_variable_scope: String,
    pub _postman_exported_at: String,
    pub _postman_exported_using: String,
}

#[derive(Deserialize)]
struct PostmanEnvVariable {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub enabled: bool
}


impl App<'_> {
    pub fn import_postman_environment(&mut self, postman_env_import: &PostmanEnvImport) -> anyhow::Result<()> {
        let path_buf = &postman_env_import.import_path;

        println!("Parsing Postman environment");

        // Read the file content
        let file_content = match fs::read_to_string(path_buf) {
            Ok(content) => content,
            Err(e) => {
                return Err(anyhow!(ImportPostmanEnvironmentError::CouldNotReadFile(e.to_string())));
            }
        };

        let postman_environment = match serde_yaml::from_str::<PostmanEnv>(&file_content) {
            Ok(postman_environment) => postman_environment,
            Err(e) => return Err(anyhow!(ImportPostmanEnvironmentError::CouldNotParsePostmanEnvironment(e.to_string()))),
        };

        println!("Postman environment name: {}", postman_environment.name);

        let filename = format!(".env.{}", postman_environment.name.to_lowercase().replace(" ", "_"));
        let path = ARGS.directory
            .as_ref()
            .unwrap()
            .join(filename);

        let mut env = Environment {
            name: postman_environment.name,
            values: IndexMap::new(),
            path
        };

        for env_variable in postman_environment.values {
            if !postman_env_import.use_disabled && !env_variable.enabled {
                continue;
            }

            let key = match postman_env_import.force_uppercase_keys {
                true => env_variable.key.to_uppercase(),
                false => env_variable.key.clone()
            };

            env.values.insert(
                key,
                env_variable.value.clone(),
            );
        }

        let env_count = self.environments.len();
        self.environments.push(Arc::new(RwLock::new(env)));

        self.save_environment_to_file(env_count);

        Ok(())
    }
}