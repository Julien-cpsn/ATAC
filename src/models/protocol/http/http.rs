use serde::{Deserialize, Serialize};
use crate::models::protocol::http::body::ContentType;
use crate::models::protocol::http::method::Method;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: Method,
    pub body: ContentType,
}