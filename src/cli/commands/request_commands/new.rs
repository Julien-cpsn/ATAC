use crate::models::method::Method;

#[derive(clap::Args, Debug, Clone)]
pub struct NewRequestCommand {
    /// Request URL
    #[arg(short, long, value_hint = clap::ValueHint::Url, default_value_t = String::new(), display_order = 0)]
    pub url: String,

    /// Request method
    #[arg(short, long, default_value_t = Method::GET, display_order = 1)]
    pub method: Method,

    /// Add a query param
    /// (can be used multiple times)
    #[arg(long, action = clap::ArgAction::Append, num_args = 2, value_names = ["KEY", "VALUE"], display_order = 2)]
    pub add_param: Vec<String>,

    #[command(flatten)]
    pub auth: AuthArgs,

    /// Do not use base headers
    /// (cache-control, user-agent, accept, accept-encoding, connection)
    #[arg(long, default_value_t = false, display_order = 5)]
    pub no_base_headers: bool,

    /// Add a header
    /// (can be used multiple times)
    #[arg(long, action = clap::ArgAction::Append, num_args = 2, value_names = ["KEY", "VALUE"], display_order = 6)]
    pub add_header: Vec<String>,

    #[command(flatten)]
    pub body: BodyArgs,

    /// Set a pre-request script
    #[arg(long, display_order = 15)]
    pub pre_request_script: Option<String>,

    /// Set a post-request script
    #[arg(long, display_order = 16)]
    pub post_request_script: Option<String>,

    /// Do not use config proxy
    #[arg(long, default_value_t = false, display_order = 17)]
    pub no_proxy: bool,

    /// Do not allow redirects
    #[arg(long, default_value_t = false, display_order = 18)]
    pub no_redirects: bool,

    /// Do not store received cookies
    #[arg(long, default_value_t = false, display_order = 19)]
    pub no_cookies: bool,

    /// Do not pretty print response content
    #[arg(long, default_value_t = false, display_order = 20)]
    pub no_pretty: bool
}

#[derive(clap::Args, Debug, Clone)]
#[group(multiple = false)]
pub struct AuthArgs {
    /// Set a basic auth method
    #[arg(long, group = "auth", action = clap::ArgAction::Set, num_args = 2, value_names = ["USERNAME", "PASSWORD"], display_order = 3)]
    pub auth_basic: Vec<String>,

    /// Set a bearer token auth method
    #[arg(long, group = "auth", action = clap::ArgAction::Set, num_args = 1, value_name = "TOKEN", display_order = 4)]
    pub auth_bearer_token: Vec<String>,
}

#[derive(clap::Args, Debug, Clone)]
#[group(multiple = false)]
pub struct BodyArgs {
    /// Set a file body
    #[arg(long, group = "body", value_name = "FILE_PATH", value_hint = clap::ValueHint::FilePath, display_order = 7)]
    pub body_file: Option<String>,

    /// Set a multipart form body
    /// (adds a value each time used)
    #[arg(long, action = clap::ArgAction::Append, num_args = 2, value_names = ["KEY", "VALUE"], display_order = 8)]
    pub add_body_multipart: Vec<String>,

    /// Set a form body
    /// (adds a value each time used)
    #[arg(long, action = clap::ArgAction::Append, num_args = 2, value_names = ["KEY", "VALUE"], display_order = 9)]
    pub add_body_form: Vec<String>,

    /// Set a raw test body
    #[arg(long, group = "body", value_name = "TEXT", display_order = 10)]
    pub body_raw: Option<String>,

    /// Set a JSON body
    #[arg(long, group = "body", value_name = "JSON", display_order = 11)]
    pub body_json: Option<String>,

    /// Set an XML body
    #[arg(long, group = "body", value_name = "XML", display_order = 12)]
    pub body_xml: Option<String>,

    /// Set an HTML body
    #[arg(long, group = "body", value_name = "HTML", display_order = 13)]
    pub body_html: Option<String>,

    /// Set an JavaScript body
    #[arg(long, group = "body", value_name = "JAVASCRIPT", display_order = 14)]
    pub body_javascript: Option<String>,
}