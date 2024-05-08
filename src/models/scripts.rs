use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RequestScripts {
    pub pre_request_script: Option<String>,
    pub post_request_script: Option<String>,
}