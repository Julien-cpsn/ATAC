use std::path::PathBuf;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    pub values: IndexMap<String, String>,
    pub path: PathBuf
}