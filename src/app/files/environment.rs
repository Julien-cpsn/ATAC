use std::path::PathBuf;
use envfile::EnvFile;
use crate::app::app::App;
use crate::request::environment::Environment;

impl App<'_> {
    /// Add the environment file to the app environments
    pub fn add_environment_from_file(&mut self, path_buf: PathBuf) {
        let file_name = path_buf.file_name().unwrap().to_str().unwrap().to_string().replace(".env.", "");
        let env = EnvFile::new(path_buf).expect("\tCould not parse environment file");

        let environment = Environment {
            name: file_name,
            values: env.store.iter().map(|(key, value)| (key.to_string(), value.to_string())).collect()
        };

        self.environments.push(environment);

        println!("environment file parsed!");
    }
}
