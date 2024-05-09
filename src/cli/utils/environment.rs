use anyhow::anyhow;
use thiserror::Error;
use crate::app::app::App;
use crate::cli::utils::environment::FindEnvironmentError::EnvironmentNotFound;

#[derive(Error, Debug)]
pub enum FindEnvironmentError {
    #[error("Environment not found")]
    EnvironmentNotFound,
}

impl App<'_> {
    pub fn find_environment(&mut self, environment_name: &str) -> anyhow::Result<usize> {
        for (index, environment) in self.environments.iter().enumerate() {
            if environment.read().name == environment_name {
                return Ok(index);
            }
        }

        return Err(anyhow!(EnvironmentNotFound));
    }
}