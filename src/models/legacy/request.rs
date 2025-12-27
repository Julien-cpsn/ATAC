use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;

use crate::models::auth::auth::Auth;
use crate::models::protocol::http::body::ContentType;
use crate::models::protocol::http::http::HttpRequest;
use crate::models::protocol::http::method::Method;
use crate::models::protocol::protocol::Protocol;
use crate::models::request::{ConsoleOutput, KeyValue, Request};
use crate::models::response::RequestResponse;
use crate::models::scripts::RequestScripts;
use crate::models::settings::RequestSettings;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RequestV0_20_2 {
    pub name: String,
    pub url: String,
    pub method: Method,
    pub params: Vec<KeyValue>,
    pub headers: Vec<KeyValue>,
    pub auth: Auth,
    pub body: ContentType,
    pub scripts: RequestScripts,
    pub settings: RequestSettings,

    #[serde(skip)]
    pub response: RequestResponse,

    #[serde(skip)]
    pub console_output: ConsoleOutput,

    #[serde(skip)]
    pub is_pending: bool,

    #[serde(skip)]
    pub cancellation_token: CancellationToken,
}

impl From<RequestV0_20_2> for Request {
    fn from(request: RequestV0_20_2) -> Self {
        Self {
            name: request.name,
            url: request.url,
            params: request.params,
            headers: request.headers,
            auth: request.auth,
            scripts: request.scripts,
            settings: request.settings,
            protocol: Protocol::HttpRequest(HttpRequest {
                method: request.method,
                body: request.body,
            }),
            response: request.response,
            console_output: request.console_output,
            is_pending: request.is_pending,
            cancellation_token: request.cancellation_token,
        }
    }
}