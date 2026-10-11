use crate::models::protocol::http::method::Method;
use crate::models::protocol::mqtt::mqtt::{MqttVersion, QoS};
use crate::models::protocol::protocol::Protocol;

#[derive(clap::Args, Debug, Clone)]
pub struct NewRequestCommand {
    /// Request URL
    #[arg(short, long, value_hint = clap::ValueHint::Url, default_value_t = String::new(), display_order = 0)]
    pub url: String,

    #[arg(short, long, default_value_t = Protocol::default(), display_order = 1)]
    /// Request communication protocol
    pub protocol: Protocol,

    /// Request method
    #[arg(short, long, default_value_t = Method::GET, display_order = 2)]
    pub method: Method,

    /// Add a query param
    /// (can be used multiple times)
    #[arg(long, action = clap::ArgAction::Append, num_args = 2, value_names = ["KEY", "VALUE"], display_order = 3)]
    pub add_param: Vec<String>,

    #[command(flatten)]
    pub auth: AuthArgs,

    /// Do not use base headers
    /// (cache-control, user-agent, accept, accept-encoding, connection)
    #[arg(long, default_value_t = false, display_order = 7)]
    pub no_base_headers: bool,

    /// Add a header
    /// (can be used multiple times)
    #[arg(long, action = clap::ArgAction::Append, num_args = 2, value_names = ["KEY", "VALUE"], display_order = 8)]
    pub add_header: Vec<String>,

    #[command(flatten)]
    pub body: BodyArgs,

    /// Set a pre-request script
    #[arg(long, display_order = 17)]
    pub pre_request_script: Option<String>,

    /// Set a post-request script
    #[arg(long, display_order = 18)]
    pub post_request_script: Option<String>,

    /// Do not use config proxy
    #[arg(long, default_value_t = false, display_order = 19)]
    pub no_proxy: bool,

    /// Do not allow redirects
    #[arg(long, default_value_t = false, display_order = 20)]
    pub no_redirects: bool,

    /// Timeout (ms)
    #[arg(long, default_value_t = 30000, display_order = 21)]
    pub timeout: u32,

    /// Do not store received cookies
    #[arg(long, default_value_t = false, display_order = 22)]
    pub no_cookies: bool,

    /// Do not pretty print response content
    #[arg(long, default_value_t = false, display_order = 23)]
    pub no_pretty: bool,

    /// Accept invalid certificates
    #[arg(long, default_value_t = false, display_order = 24)]
    pub accept_invalid_certs: bool,

    /// Accept invalid hostnames
    #[arg(long, default_value_t = false, display_order = 25)]
    pub accept_invalid_hostnames: bool,

    #[command(flatten)]
    pub mqtt: MqttArgs,
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

    /// Set a JWT token auth method
    #[arg(long, group = "auth", action = clap::ArgAction::Set, num_args = 4, value_names = ["ALGORITHM", "SECRET_TYPE", "SECRET", "PAYLOAD"], display_order = 5)]
    pub auth_jwt_token: Vec<String>,

    /// Set a digest auth method
    #[arg(long, group = "auth", action = clap::ArgAction::Set, num_args = 3, value_names = ["USERNAME", "PASSWORD", "WWW_AUTHENTICATE_HEADER"], display_order = 6)]
    pub auth_digest: Vec<String>,
}

#[derive(clap::Args, Debug, Clone)]
#[group(multiple = false)]
pub struct BodyArgs {
    /// Set a file body
    #[arg(long, group = "body", value_name = "FILE_PATH", value_hint = clap::ValueHint::FilePath, display_order = 9)]
    pub body_file: Option<String>,

    /// Set a multipart form body
    /// (adds a value each time used)
    #[arg(long, action = clap::ArgAction::Append, num_args = 2, value_names = ["KEY", "VALUE"], display_order = 10)]
    pub add_body_multipart: Vec<String>,

    /// Set a form body
    /// (adds a value each time used)
    #[arg(long, action = clap::ArgAction::Append, num_args = 2, value_names = ["KEY", "VALUE"], display_order = 11)]
    pub add_body_form: Vec<String>,

    /// Set a raw test body
    #[arg(long, group = "body", value_name = "TEXT", display_order = 12)]
    pub body_raw: Option<String>,

    /// Set a JSON body
    #[arg(long, group = "body", value_name = "JSON", display_order = 13)]
    pub body_json: Option<String>,

    /// Set an XML body
    #[arg(long, group = "body", value_name = "XML", display_order = 14)]
    pub body_xml: Option<String>,

    /// Set an HTML body
    #[arg(long, group = "body", value_name = "HTML", display_order = 15)]
    pub body_html: Option<String>,

    /// Set an JavaScript body
    #[arg(long, group = "body", value_name = "JAVASCRIPT", display_order = 16)]
    pub body_javascript: Option<String>,
}


#[derive(clap::Args, Debug, Clone)]
pub struct MqttArgs {
    /// MQTT protocol version
    #[arg(long, value_name = "VERSION", display_order = 26)]
    pub mqtt_version: Option<MqttVersion>,

    /// MQTT client ID
    /// (assigned by the broker when empty)
    #[arg(long, display_order = 27)]
    pub client_id: Option<String>,

    /// Keep the MQTT session when disconnected
    #[arg(long, default_value_t = false, display_order = 28)]
    pub no_clean_session: bool,

    /// How long the broker keeps the session (s, MQTT 5)
    #[arg(long, value_name = "SECONDS", display_order = 29)]
    pub session_expiry: Option<u32>,

    /// MQTT keep alive (s)
    #[arg(long, value_name = "SECONDS", display_order = 30)]
    pub keep_alive: Option<u16>,

    /// MQTT max packet size (bytes)
    #[arg(long, value_name = "BYTES", value_parser = clap::value_parser!(u32).range(1..), display_order = 31)]
    pub max_packet_size: Option<u32>,

    /// Add an MQTT subscription, QoS is 0, 1 or 2
    /// (can be used multiple times)
    #[arg(long, action = clap::ArgAction::Append, num_args = 2, value_names = ["TOPIC", "QOS"], display_order = 32)]
    pub add_subscription: Vec<String>,

    /// MQTT topic to publish to
    #[arg(long, value_name = "TOPIC", display_order = 33)]
    pub publish_topic: Option<String>,

    /// MQTT publish QoS
    #[arg(long, value_name = "QOS", display_order = 34)]
    pub publish_qos: Option<QoS>,

    /// Retain published MQTT messages
    #[arg(long, default_value_t = false, display_order = 35)]
    pub publish_retain: bool,
}

impl MqttArgs {
    pub fn is_used(&self) -> bool {
        self.mqtt_version.is_some()
            || self.client_id.is_some()
            || self.no_clean_session
            || self.session_expiry.is_some()
            || self.keep_alive.is_some()
            || self.max_packet_size.is_some()
            || !self.add_subscription.is_empty()
            || self.publish_topic.is_some()
            || self.publish_qos.is_some()
            || self.publish_retain
    }
}
