use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    pub values: HashMap<String, String>,
}