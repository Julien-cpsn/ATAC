use anyhow::anyhow;
use clap::ValueEnum;
use tokio_util::sync::CancellationToken;
use crate::app::app::App;
use crate::cli::commands::request_commands::new::{AuthArgs, BodyArgs, MqttArgs, NewRequestCommand};
use crate::models::auth::auth::Auth;
use crate::models::auth::basic::BasicAuth;
use crate::models::auth::bearer_token::BearerToken;
use crate::models::auth::digest::{extract_www_authenticate_digest_data, Digest};
use crate::models::auth::jwt::{JwtAlgorithm, JwtSecretType, JwtToken};
use crate::models::protocol::http::body::ContentType;
use crate::models::protocol::http::method::Method;
use crate::models::protocol::mqtt::mqtt::{MqttRequest, MqttSubscription, QoS};
use crate::models::protocol::protocol::Protocol;
use crate::models::request::{ConsoleOutput, KeyValue, Request, DEFAULT_HEADERS};
use crate::models::response::RequestResponse;
use crate::models::scripts::RequestScripts;
use crate::models::settings::{RequestSettings, Setting};
use crate::panic_error;

impl App<'_> {
    pub fn cli_new_request(&mut self, collection_slash_request: (String, String), new_request_command: NewRequestCommand) -> anyhow::Result<()> {
        let collection_index = self.find_collection(&collection_slash_request.0)?;
        let new_request = create_request_from_new_request_command(collection_slash_request.1.trim().to_string(), new_request_command)?;
        
        self.new_request(collection_index, new_request)?;
        
        Ok(())
    }
}

pub fn create_request_from_new_request_command(request_name: String, new_request_command: NewRequestCommand) -> anyhow::Result<Request> {
    let params = string_array_to_key_value_array(new_request_command.add_param);
    let auth = get_auth_from_auth_args(new_request_command.auth)?;
    let headers = string_array_to_key_value_array(new_request_command.add_header);
    let body = get_content_type_from_body_args(new_request_command.body);

    // MQTT has no headers
    let base_headers = match new_request_command.no_base_headers || matches!(new_request_command.protocol, Protocol::MqttRequest(_)) {
        true => vec![],
        false => DEFAULT_HEADERS.clone()
    };

    let mut protocol = new_request_command.protocol.clone();

    if new_request_command.mqtt.is_used() && !matches!(protocol, Protocol::MqttRequest(_)) {
        return Err(anyhow!("MQTT options can only be used with an MQTT request"));
    }

    match &mut protocol {
        Protocol::HttpRequest(http_request) => {
            http_request.method = new_request_command.method;
            http_request.body = body;
        }
        Protocol::WsRequest(_) => {
            match new_request_command.method {
                Method::GET => {}
                _ => return Err(anyhow!("Setting a method with a websocket request is incompatible"))
            }

            match body {
                ContentType::NoBody => {}
                _ => return Err(anyhow!("Setting a body with a websocket request body is incompatible"))
            }
        }
        Protocol::MqttRequest(mqtt_request) => {
            set_mqtt_request_from_mqtt_args(mqtt_request, new_request_command.mqtt)?;

            match new_request_command.method {
                Method::GET => {}
                _ => return Err(anyhow!("Setting a method with an MQTT request is incompatible"))
            }

            match body {
                ContentType::NoBody => {}
                _ => return Err(anyhow!("Setting a body with an MQTT request is incompatible"))
            }

            if !headers.is_empty() {
                return Err(anyhow!("Setting a header with an MQTT request is incompatible"))
            }
        }
    };

    let mut request = Request {
        name: request_name,
        url: String::new(),
        protocol,
        params,
        auth,
        headers: vec![base_headers, headers].concat(),
        scripts: RequestScripts {
            pre_request_script: new_request_command.pre_request_script,
            post_request_script: new_request_command.post_request_script,
        },
        settings: RequestSettings {
            use_config_proxy: Setting::Bool(!new_request_command.no_proxy),
            allow_redirects: Setting::Bool(!new_request_command.no_redirects),
            timeout: Setting::U32(new_request_command.timeout),
            store_received_cookies: Setting::Bool(!new_request_command.no_cookies),
            pretty_print_response_content: Setting::Bool(!new_request_command.no_pretty),
            accept_invalid_certs: Setting::Bool(new_request_command.accept_invalid_certs),
            accept_invalid_hostnames: Setting::Bool(new_request_command.accept_invalid_hostnames),
        },
        response: RequestResponse::default(),
        console_output: ConsoleOutput::default(),
        is_pending: false,
        cancellation_token: CancellationToken::new(),
    };

    request.update_url_and_params(new_request_command.url);

    Ok(request)
}

fn string_array_to_key_value_array(string_array: Vec<String>) -> Vec<KeyValue> {
    let mut key_value_array: Vec<KeyValue> = vec![];

    for i in (0..string_array.len()).step_by(2) {
        key_value_array.push(KeyValue {
            enabled: true,
            data: (string_array[i].clone(), string_array[i + 1].clone()),
        })
    }

    return key_value_array
}

fn set_mqtt_request_from_mqtt_args(mqtt_request: &mut MqttRequest, mqtt_args: MqttArgs) -> anyhow::Result<()> {
    if let Some(version) = mqtt_args.mqtt_version {
        mqtt_request.version = version;
    }

    if let Some(client_id) = mqtt_args.client_id {
        mqtt_request.client_id = client_id;
    }

    mqtt_request.clean_session = !mqtt_args.no_clean_session;

    if let Some(session_expiry) = mqtt_args.session_expiry {
        mqtt_request.session_expiry_interval = session_expiry;
    }

    if let Some(keep_alive) = mqtt_args.keep_alive {
        mqtt_request.keep_alive = keep_alive;
    }

    if let Some(max_packet_size) = mqtt_args.max_packet_size {
        mqtt_request.max_packet_size = max_packet_size;
    }

    for subscription in mqtt_args.add_subscription.chunks(2) {
        mqtt_request.subscriptions.push(MqttSubscription {
            enabled: true,
            topic: subscription[0].clone(),
            qos: QoS::from_str(&subscription[1], true).map_err(|_| anyhow!("Invalid QoS \"{}\", expected 0, 1 or 2", subscription[1]))?,
        });
    }

    if let Some(publish_topic) = mqtt_args.publish_topic {
        mqtt_request.publish.topic = publish_topic;
    }

    if let Some(publish_qos) = mqtt_args.publish_qos {
        mqtt_request.publish.qos = publish_qos;
    }

    mqtt_request.publish.retain = mqtt_args.publish_retain;

    Ok(())
}

fn get_auth_from_auth_args(auth_args: AuthArgs) -> anyhow::Result<Auth> {
    if !auth_args.auth_basic.is_empty() {
        Ok(
            Auth::BasicAuth(BasicAuth {
                username: auth_args.auth_basic[0].clone(),
                password: auth_args.auth_basic[1].clone()
            })
        )
    }
    else if !auth_args.auth_bearer_token.is_empty() {
        Ok(Auth::BearerToken(
            BearerToken {
                token: auth_args.auth_bearer_token[0].clone()
            })
        )
    }
    else if !auth_args.auth_jwt_token.is_empty() {
        return Ok(Auth::JwtToken(
            JwtToken {
                algorithm: JwtAlgorithm::from_str(&auth_args.auth_jwt_token[0], true).map_err(|e| anyhow!(e))?,
                secret_type: JwtSecretType::from_str(&auth_args.auth_jwt_token[1], true).map_err(|e| anyhow!(e))?,
                secret: auth_args.auth_jwt_token[2].clone(),
                payload: auth_args.auth_jwt_token[3].clone(),
            }
        ));
    }
    else if !auth_args.auth_digest.is_empty() {
        let www_authenticate_header = &auth_args.auth_digest[2];
        return match extract_www_authenticate_digest_data(www_authenticate_header) {
            Ok((domains, realm, nonce, opaque, stale, algorithm, qop, user_hash, charset)) => Ok(Auth::Digest(
                Digest {
                    username: auth_args.auth_digest[0].clone(),
                    password: auth_args.auth_digest[1].clone(),
                    domains,
                    realm,
                    nonce,
                    opaque,
                    stale,
                    algorithm,
                    qop,
                    user_hash,
                    charset,
                    nc: 0,
                }
            )),
            Err(error) => panic_error(error)
        };
    }
    else {
        return Ok(Auth::NoAuth);
    }
}

fn get_content_type_from_body_args(body_args: BodyArgs) -> ContentType {
    if let Some(file_path) = &body_args.body_file {
        return ContentType::File(file_path.clone());
    }
    else if !body_args.add_body_multipart.is_empty() {
        let multipart_key_values = string_array_to_key_value_array(body_args.add_body_multipart);
        return ContentType::Multipart(multipart_key_values);
    }
    else if !body_args.add_body_form.is_empty() {
        let form_key_values = string_array_to_key_value_array(body_args.add_body_multipart);
        return ContentType::Form(form_key_values);
    }
    else if let Some(raw) = &body_args.body_raw {
        return ContentType::Raw(raw.clone());
    }
    else if let Some(json) = &body_args.body_json {
        return ContentType::Json(json.clone());
    }
    else if let Some(xml) = &body_args.body_xml {
        return ContentType::Xml(xml.clone());
    }
    else if let Some(html) = &body_args.body_html {
        return ContentType::Html(html.clone());
    }
    else if let Some(javascript) = &body_args.body_javascript {
        return ContentType::Javascript(javascript.clone());
    }
    else {
        return ContentType::NoBody;
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use crate::cli::cli_logic::request::new::create_request_from_new_request_command;
    use crate::cli::commands::request_commands::new::NewRequestCommand;
    use crate::models::protocol::mqtt::mqtt::{MqttVersion, QoS};
    use crate::models::protocol::protocol::Protocol;
    use crate::models::request::Request;

    #[derive(Parser)]
    struct TestArgs {
        #[command(flatten)]
        command: NewRequestCommand,
    }

    fn new_request(args: &[&str]) -> anyhow::Result<Request> {
        let command = TestArgs::try_parse_from(std::iter::once("atac").chain(args.iter().copied()))?.command;
        create_request_from_new_request_command(String::from("test"), command)
    }

    #[test]
    fn mqtt_options_are_applied() {
        let request = new_request(&[
            "-p", "MQTT", "-u", "mqtts://broker:8886",
            "--mqtt-version", "5", "--client-id", "atac", "--no-clean-session", "--session-expiry", "120",
            "--keep-alive", "30", "--max-packet-size", "4096",
            "--add-subscription", "a/#", "1", "--add-subscription", "b/+", "2",
            "--publish-topic", "a/b", "--publish-qos", "2", "--publish-retain",
        ]).unwrap();

        let Protocol::MqttRequest(mqtt_request) = &request.protocol else { panic!("not an MQTT request") };

        assert_eq!(request.url, "mqtts://broker:8886");
        assert_eq!(mqtt_request.version, MqttVersion::V5);
        assert_eq!(mqtt_request.client_id, "atac");
        assert!(!mqtt_request.clean_session);
        assert_eq!(mqtt_request.session_expiry_interval, 120);
        assert_eq!(mqtt_request.keep_alive, 30);
        assert_eq!(mqtt_request.max_packet_size, 4096);
        assert_eq!(mqtt_request.subscriptions.len(), 2);
        assert_eq!((mqtt_request.subscriptions[1].topic.as_str(), mqtt_request.subscriptions[1].qos), ("b/+", QoS::ExactlyOnce));
        assert_eq!((mqtt_request.publish.topic.as_str(), mqtt_request.publish.qos, mqtt_request.publish.retain), ("a/b", QoS::ExactlyOnce, true));
    }

    #[test]
    fn mqtt_requests_have_no_http_headers() {
        assert!(new_request(&["-p", "MQTT"]).unwrap().headers.is_empty());
        assert!(!new_request(&["-p", "HTTP"]).unwrap().headers.is_empty());
    }

    #[test]
    fn mqtt_options_need_an_mqtt_request() {
        for protocol in ["HTTP", "websocket"] {
            assert!(new_request(&["-p", protocol, "--client-id", "atac"]).is_err(), "{protocol}");
            assert!(new_request(&["-p", protocol, "--add-subscription", "a", "0"]).is_err(), "{protocol}");
        }
    }

    #[test]
    fn http_options_are_refused_on_mqtt_requests() {
        assert!(new_request(&["-p", "MQTT", "-m", "POST"]).is_err());
        assert!(new_request(&["-p", "MQTT", "--body-raw", "x"]).is_err());
        assert!(new_request(&["-p", "MQTT", "--add-header", "a", "b"]).is_err());
    }

    #[test]
    fn invalid_values_are_refused() {
        assert!(new_request(&["-p", "MQTT", "--add-subscription", "a", "3"]).is_err());
        assert!(new_request(&["-p", "MQTT", "--publish-qos", "3"]).is_err());
        assert!(new_request(&["-p", "MQTT", "--mqtt-version", "4"]).is_err());
        assert!(new_request(&["-p", "MQTT", "--max-packet-size", "0"]).is_err());
        assert!(new_request(&["-p", "MQTT", "--keep-alive", "65536"]).is_err());
    }
}
