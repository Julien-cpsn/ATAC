use crate::cli::utils::arguments_validators::collection_and_request_validator;

#[derive(clap::Args, Debug, Clone)]
pub struct SendCommand {
    /// e.g. my_collection/my_request, my_collection
    #[arg(value_parser = collection_and_request_validator)]
    pub collection_and_request: CollectionAndRequestArg,

    /// Hide response content
    #[arg(long, default_value_t = false)]
    pub hide_content: bool,

    /// Show the status code
    #[arg(long, default_value_t = false)]
    pub status_code: bool,

    /// Show the duration
    #[arg(long, default_value_t = false)]
    pub duration: bool,

    /// Show the response headers
    #[arg(long, default_value_t = false)]
    pub headers: bool,

    /// Show the response cookies
    #[arg(long, default_value_t = false)]
    pub cookies: bool,

    /// Show the pre and post-request script console output
    #[arg(long, default_value_t = false)]
    pub console: bool,

    /// Show the request name
    #[arg(long, default_value_t = false)]
    pub request_name: bool,
}

#[derive(Debug, Clone)]
pub enum CollectionAndRequestArg {
    CollectionOnly(String),
    CollectionAndRequest(String, String),
}