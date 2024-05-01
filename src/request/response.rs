use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RequestResponse {
    #[serde(skip)]
    pub duration: Option<String>,

    #[serde(skip)]
    pub status_code: Option<String>,

    pub body: Option<String>,
    pub cookies: Option<String>,
    pub headers: Vec<(String, String)>
}