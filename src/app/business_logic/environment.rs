use std::sync::Arc;
use parking_lot::RwLock;

use crate::app::app::App;
use crate::models::environment::Environment;

impl App<'_> {
    pub fn get_selected_env_as_local(&self) -> Option<Arc<RwLock<Environment>>> {
        match self.environments.get(self.selected_environment) {
            None => None,
            Some(env) => Some(env.clone())
        }
    }

    pub fn replace_env_keys_by_value(&self, input: &String) -> String {
        if self.environments.is_empty() {
            return input.to_string();
        }

        let mut tmp_string = input.to_string();

        let local_env = self.get_selected_env_as_local();

        if let Some(local_env) = local_env {
            let env = local_env.read();

            for (key, value) in &env.values {
                tmp_string = tmp_string.replace(&format!("{{{{{}}}}}", key), value);
            }
        }

        return tmp_string;
    }
}