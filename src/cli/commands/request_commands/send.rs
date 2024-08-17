#[derive(clap::Args, Debug, Clone)]
pub struct SendCommand {
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

    /// Name of the environment to use, e.g. my_env (from file .env.my_env)
    #[arg(long, value_name = "ENV_NAME", display_order = 98)]
    pub env: Option<String>
}