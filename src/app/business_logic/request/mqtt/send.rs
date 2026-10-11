use std::sync::Arc;
use std::time::{Duration, Instant};
use chrono::Local;
use parking_lot::{Mutex, RwLock};
use reqwest::Url;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, trace};
use crate::app::app::App;
use crate::app::business_logic::request::mqtt::client::{MqttClient, MqttEvent, MqttEventLoop, PreparedMqttRequest};
use crate::app::business_logic::request::send::{PrepareRequestError, RequestResponseError};
use crate::models::auth::auth::Auth;
use crate::models::auth::basic::BasicAuth;
use crate::models::environment::Environment;
use crate::models::protocol::mqtt::mqtt::{MqttCommand, MqttMessage, MqttMessageContent, MqttRequest, MqttVersion, QoS};
use crate::models::protocol::mqtt::payload::MqttPayload;
use crate::models::protocol::ws::ws::Sender;
use crate::models::request::Request;
use crate::models::response::{RequestResponse, ResponseContent};

impl App<'_> {
    pub fn prepare_mqtt_request(&self, request: &mut Request) -> Result<PreparedMqttRequest, PrepareRequestError> {
        trace!("Preparing MQTT request");

        let env = self.get_selected_env_as_local();

        /* PRE-REQUEST SCRIPT */

        let modified_request = self.handle_pre_request_script(request, env)?;

        // The script could have changed the protocol
        let Ok(mqtt_request) = modified_request.get_mqtt_request() else {
            return Err(PrepareRequestError::PreRequestScriptChangedProtocol);
        };

        /* URL */

        let url = self.replace_env_keys_by_value(&modified_request.url);

        let url = match Url::parse(&url) {
            Ok(url) => url,
            Err(_) => return Err(PrepareRequestError::InvalidUrl)
        };

        let (use_tls, default_port) = match url.scheme() {
            "mqtt" | "tcp" => (false, 1883),
            "mqtts" | "ssl" => (true, 8883),
            _ => return Err(PrepareRequestError::InvalidMqttUrlScheme)
        };

        let host = match url.host_str() {
            Some(host) => host.to_string(),
            None => return Err(PrepareRequestError::InvalidUrl)
        };

        /* AUTH */

        let credentials = match &modified_request.auth {
            Auth::NoAuth => None,
            Auth::BasicAuth(BasicAuth { username, password }) => Some((
                self.replace_env_keys_by_value(username),
                self.replace_env_keys_by_value(password)
            )),
            Auth::BearerToken(_) | Auth::JwtToken(_) | Auth::Digest(_) => return Err(PrepareRequestError::UnsupportedMqttAuth)
        };

        /* CLIENT ID */

        let client_id = self.replace_env_keys_by_value(&mqtt_request.client_id);

        // Checked here because rumqttc panics on it, MQTT 5 allows it and lets the broker assign an ID
        if mqtt_request.version == MqttVersion::V3_1_1 && client_id.is_empty() && !mqtt_request.clean_session {
            return Err(PrepareRequestError::MqttClientIdRequired);
        }

        /* KEEP ALIVE */

        // Same, rumqttc panics on MQTT 5 keep alives under 5 seconds
        if mqtt_request.version == MqttVersion::V5 && mqtt_request.keep_alive < 5 {
            return Err(PrepareRequestError::MqttKeepAliveTooShort);
        }

        /* MAX PACKET SIZE */

        // A zero limit is a protocol error in MQTT 5 and would refuse every packet in 3.1.1
        if mqtt_request.max_packet_size == 0 {
            return Err(PrepareRequestError::MqttMaxPacketSizeZero);
        }

        /* SUBSCRIPTIONS */

        let subscriptions = mqtt_request.subscriptions
            .iter()
            .filter(|subscription| subscription.enabled)
            .map(|subscription| (self.replace_env_keys_by_value(&subscription.topic), subscription.qos))
            .collect();

        Ok(PreparedMqttRequest {
            version: mqtt_request.version,
            host,
            port: url.port().unwrap_or(default_port),
            use_tls,
            client_id,
            clean_session: mqtt_request.clean_session,
            session_expiry_interval: mqtt_request.session_expiry_interval,
            keep_alive: mqtt_request.keep_alive,
            max_packet_size: mqtt_request.max_packet_size,
            credentials,
            subscriptions,
            connection_timeout: (modified_request.settings.timeout.as_u32() as u64).div_ceil(1000).max(1),
            accept_invalid_certs: modified_request.settings.accept_invalid_certs.as_bool(),
            accept_invalid_hostnames: modified_request.settings.accept_invalid_hostnames.as_bool(),
        })
    }
}

pub async fn send_mqtt_request(prepared_request: PreparedMqttRequest, local_request: Arc<RwLock<Request>>, env: &Option<Arc<RwLock<Environment>>>, received_response: Arc<Mutex<bool>>) -> Result<RequestResponse, RequestResponseError> {
    info!("Connecting to MQTT broker");

    let mut request = local_request.write();
    request.is_pending = true;

    let cancellation_token = request.cancellation_token.clone();
    let timeout = Duration::from_millis(request.settings.timeout.as_u32() as u64);

    let mqtt_request = request.get_mqtt_request_mut().unwrap();
    mqtt_request.is_connected = false;
    mqtt_request.broker_max_packet_size = None;

    drop(request);

    let request_start = Instant::now();
    let mut broker_max_packet_size = None;
    let mut connection: Option<(MqttClient, MqttEventLoop)> = None;

    let mut response = match MqttClient::new(&prepared_request) {
        Err(error) => error_response(error),
        Ok((new_client, mut new_event_loop)) => {
            let connack = tokio::select! {
                _ = cancellation_token.cancelled() => Err(String::from("CANCELED")),
                _ = tokio::time::sleep(timeout) => Err(String::from("TIMEOUT")),
                connack = wait_for_connack(&mut new_event_loop) => Ok(connack),
            };

            match connack {
                Err(status_code) => RequestResponse {
                    duration: None,
                    status_code: Some(status_code),
                    content: None,
                    cookies: None,
                    headers: vec![],
                },
                Ok(Err(error)) => error_response(error),
                Ok(Ok((code, session_present, properties, max_packet_size))) => {
                    info!("Connected to MQTT broker");

                    broker_max_packet_size = max_packet_size;

                    let mut headers = vec![(String::from("session present"), session_present.to_string())];
                    headers.extend(properties);

                    connection = Some((new_client, new_event_loop));

                    RequestResponse {
                        duration: None,
                        status_code: Some(format!("CONNACK {code}")),
                        content: None,
                        cookies: None,
                        headers,
                    }
                }
            }
        }
    };

    response.duration = Some(format!("{:?}", request_start.elapsed()));

    trace!("MQTT connection attempt done");

    /* POST-REQUEST SCRIPT */

    let request = local_request.read();
    let (modified_response, post_request_output) = App::handle_post_request_script(&request, response, env)?;
    drop(request);

    let mut request = local_request.write();

    request.console_output.post_request_output = post_request_output;
    request.is_pending = false;
    request.cancellation_token = CancellationToken::new();

    let mqtt_request = request.get_mqtt_request_mut().unwrap();
    mqtt_request.messages = vec![];

    let Some((client, event_loop)) = connection else {
        // The messages tab is where MQTT requests are read, so the reason goes there too
        if let Some(ResponseContent::Body(reason)) = &modified_response.content {
            mqtt_request.messages.push(MqttMessage {
                timestamp: Local::now(),
                sender: Sender::Server,
                content: MqttMessageContent::Event(reason.clone()),
            });
        }

        return Ok(modified_response);
    };

    let (commands_sender, commands_receiver) = unbounded_channel();
    mqtt_request.connection = Some(commands_sender);
    mqtt_request.is_connected = true;
    mqtt_request.broker_max_packet_size = broker_max_packet_size;

    drop(request);

    // rumqttc queues requests in a bounded channel that only the event loop empties,
    // so requests are sent from their own task to never block the polling
    tokio::spawn(events_loop(event_loop, local_request.clone(), received_response));
    tokio::spawn(requests_loop(client, prepared_request.subscriptions, commands_receiver, local_request));

    Ok(modified_response)
}

/// Queue a message to be published on the request's broker connection, and add it to the messages
pub fn mqtt_publish(mqtt_request: &mut MqttRequest, topic: String, payload: MqttPayload, qos: QoS, retain: bool) {
    let Some(connection) = &mqtt_request.connection else {
        info!("MQTT client is not connected");
        return;
    };

    let payload_bytes = payload.to_bytes();

    // Sending a packet over the limit would close the connection, the fixed and variable headers are at most 9 bytes,
    // plus the properties length for MQTT 5
    let headers_size = match mqtt_request.version {
        MqttVersion::V3_1_1 => 9,
        MqttVersion::V5 => 10,
    };
    let packet_size = payload_bytes.len() + topic.len() + headers_size;

    // An MQTT 5 broker can accept less than the request allows, rumqttc would then close the connection
    let (max_packet_size, limit_owner) = match mqtt_request.broker_max_packet_size {
        Some(broker_max_packet_size) if broker_max_packet_size < mqtt_request.max_packet_size => (broker_max_packet_size, "the broker's "),
        _ => (mqtt_request.max_packet_size, "the "),
    };

    if packet_size > max_packet_size as usize {
        mqtt_request.messages.push(MqttMessage {
            timestamp: Local::now(),
            sender: Sender::You,
            content: MqttMessageContent::Event(format!(
                "Not published to \"{topic}\": the message is about {packet_size} bytes, over {limit_owner}{max_packet_size} bytes max packet size"
            )),
        });
        return;
    }

    let command = MqttCommand::Publish {
        topic: topic.clone(),
        payload: payload_bytes,
        qos,
        retain,
    };

    if connection.send(command).is_err() {
        return;
    }

    mqtt_request.messages.push(MqttMessage {
        timestamp: Local::now(),
        sender: Sender::You,
        content: MqttMessageContent::Publish {
            topic,
            payload,
            qos,
            retain,
        },
    });
}

pub fn mqtt_disconnect(mqtt_request: &mut MqttRequest) {
    if let Some(connection) = &mqtt_request.connection {
        connection.send(MqttCommand::Disconnect).ok();
    }
}

async fn wait_for_connack(event_loop: &mut MqttEventLoop) -> Result<(String, bool, Vec<(String, String)>, Option<u32>), String> {
    loop {
        if let MqttEvent::ConnAck { code, session_present, properties, max_packet_size } = event_loop.poll().await? {
            return Ok((code, session_present, properties, max_packet_size));
        }
    }
}

/// Sends the subscriptions, then the publish and disconnect commands, until disconnected
async fn requests_loop(client: MqttClient, subscriptions: Vec<(String, QoS)>, mut commands_receiver: UnboundedReceiver<MqttCommand>, local_request: Arc<RwLock<Request>>) {
    for (topic, qos) in subscriptions {
        if let Err(error) = client.subscribe(topic.clone(), qos).await {
            push_event(&local_request, format!("Could not subscribe to \"{topic}\": {error}"));
        }
    }

    loop {
        match commands_receiver.recv().await {
            Some(MqttCommand::Publish { topic, payload, qos, retain }) => {
                if let Err(error) = client.publish(topic.clone(), payload, qos, retain).await {
                    push_event(&local_request, format!("Could not publish to \"{topic}\": {error}"));
                }
            },
            // The command sender is dropped when the connection has ended
            Some(MqttCommand::Disconnect) | None => {
                client.disconnect().await.ok();
                break;
            }
        }
    }
}

/// Polls the broker connection and logs the incoming messages until it ends
async fn events_loop(mut event_loop: MqttEventLoop, local_request: Arc<RwLock<Request>>, received_response: Arc<Mutex<bool>>) {
    let reason = loop {
        match event_loop.poll().await {
            Ok(MqttEvent::Publish { topic, payload, qos, retain }) => {
                let mut request = local_request.write();
                let mqtt_request = request.get_mqtt_request_mut().unwrap();

                mqtt_request.messages.push(MqttMessage {
                    timestamp: Local::now(),
                    sender: Sender::Server,
                    content: MqttMessageContent::Publish {
                        topic,
                        payload: MqttPayload::from_bytes(&payload),
                        qos,
                        retain,
                    },
                });
            },
            Ok(MqttEvent::SubAck(return_codes)) => push_event(&local_request, format!("Subscription result: {return_codes}")),
            Ok(MqttEvent::Disconnect(reason)) => break reason,
            Ok(MqttEvent::Disconnected) => break String::from("Disconnected"),
            Ok(MqttEvent::ConnAck { .. } | MqttEvent::Other) => continue,
            Err(error) => {
                error!("MQTT connection error: {}", error);
                break format!("Connection closed: {error}");
            }
        }

        *received_response.lock() = true;
    };

    push_event(&local_request, reason);

    // Dropping the command sender also ends the requests loop
    let mut request = local_request.write();
    let mqtt_request = request.get_mqtt_request_mut().unwrap();
    mqtt_request.connection = None;
    mqtt_request.is_connected = false;

    *received_response.lock() = true;
}

fn push_event(local_request: &Arc<RwLock<Request>>, event: String) {
    let mut request = local_request.write();
    let mqtt_request = request.get_mqtt_request_mut().unwrap();

    mqtt_request.messages.push(MqttMessage {
        timestamp: Local::now(),
        sender: Sender::Server,
        content: MqttMessageContent::Event(event),
    });
}

fn error_response(error: String) -> RequestResponse {
    error!("MQTT connection error: {}", error);

    RequestResponse {
        duration: None,
        status_code: Some(error.clone()),
        content: Some(ResponseContent::Body(error)),
        cookies: None,
        headers: vec![],
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use indexmap::IndexMap;
    use parking_lot::RwLock;
    use crate::app::app::App;
    use crate::app::business_logic::request::mqtt::client::PreparedMqttRequest;
    use crate::app::business_logic::request::mqtt::send::mqtt_publish;
    use crate::app::business_logic::request::send::PrepareRequestError;
    use crate::models::auth::auth::Auth;
    use crate::models::auth::basic::BasicAuth;
    use crate::models::auth::bearer_token::BearerToken;
    use crate::models::environment::Environment;
    use crate::models::protocol::mqtt::mqtt::{MqttCommand, MqttMessageContent, MqttRequest, MqttSubscription, MqttVersion, QoS};
    use crate::models::protocol::mqtt::payload::MqttPayload;
    use crate::models::protocol::protocol::Protocol;
    use crate::models::request::Request;
    use crate::models::settings::Setting;

    fn request(url: &str, edit: impl FnOnce(&mut MqttRequest)) -> Request {
        let mut mqtt_request = MqttRequest::default();
        edit(&mut mqtt_request);

        Request {
            url: url.to_string(),
            protocol: Protocol::MqttRequest(mqtt_request),
            ..Default::default()
        }
    }

    fn prepare(request: &mut Request) -> Result<PreparedMqttRequest, PrepareRequestError> {
        App::new().unwrap().prepare_mqtt_request(request)
    }

    #[test]
    fn url_schemes_and_default_ports() {
        for (url, use_tls, port) in [
            ("mqtt://broker", false, 1883),
            ("tcp://broker", false, 1883),
            ("mqtts://broker", true, 8883),
            ("ssl://broker", true, 8883),
            ("mqtt://broker:1884", false, 1884),
            ("mqtts://broker:8886", true, 8886),
        ] {
            let prepared = prepare(&mut request(url, |_| {})).unwrap();

            assert_eq!(prepared.host, "broker", "{url}");
            assert_eq!(prepared.use_tls, use_tls, "{url}");
            assert_eq!(prepared.port, port, "{url}");
        }
    }

    #[test]
    fn invalid_urls_are_refused() {
        for url in ["http://broker", "ws://broker", "wss://broker"] {
            assert!(matches!(prepare(&mut request(url, |_| {})), Err(PrepareRequestError::InvalidMqttUrlScheme)), "{url}");
        }

        for url in ["", "broker", "mqtt://"] {
            assert!(matches!(prepare(&mut request(url, |_| {})), Err(PrepareRequestError::InvalidUrl)), "{url:?}");
        }
    }

    #[test]
    fn only_basic_auth_is_supported() {
        let mut no_auth = request("mqtt://broker", |_| {});
        assert!(prepare(&mut no_auth).unwrap().credentials.is_none());

        let mut basic = request("mqtt://broker", |_| {});
        basic.auth = Auth::BasicAuth(BasicAuth { username: String::from("user"), password: String::from("pass") });
        assert_eq!(prepare(&mut basic).unwrap().credentials, Some((String::from("user"), String::from("pass"))));

        let mut bearer = request("mqtt://broker", |_| {});
        bearer.auth = Auth::BearerToken(BearerToken { token: String::from("token") });
        assert!(matches!(prepare(&mut bearer), Err(PrepareRequestError::UnsupportedMqttAuth)));
    }

    #[test]
    fn empty_client_id_needs_clean_session_in_3_1_1_only() {
        let persistent = |version| request("mqtt://broker", |mqtt_request| {
            mqtt_request.version = version;
            mqtt_request.clean_session = false;
        });

        assert!(matches!(prepare(&mut persistent(MqttVersion::V3_1_1)), Err(PrepareRequestError::MqttClientIdRequired)));
        assert!(prepare(&mut persistent(MqttVersion::V5)).is_ok());
    }

    #[test]
    fn mqtt_5_keep_alive_must_be_at_least_5_seconds() {
        let keep_alive = |version, keep_alive| request("mqtt://broker", |mqtt_request| {
            mqtt_request.version = version;
            mqtt_request.keep_alive = keep_alive;
        });

        assert!(matches!(prepare(&mut keep_alive(MqttVersion::V5, 4)), Err(PrepareRequestError::MqttKeepAliveTooShort)));
        assert!(prepare(&mut keep_alive(MqttVersion::V5, 5)).is_ok());
        assert!(prepare(&mut keep_alive(MqttVersion::V3_1_1, 0)).is_ok());
    }

    #[test]
    fn zero_max_packet_size_is_refused() {
        let mut zero = request("mqtt://broker", |mqtt_request| mqtt_request.max_packet_size = 0);
        assert!(matches!(prepare(&mut zero), Err(PrepareRequestError::MqttMaxPacketSizeZero)));
    }

    #[test]
    fn only_enabled_subscriptions_are_kept() {
        let mut subscriptions = request("mqtt://broker", |mqtt_request| {
            mqtt_request.subscriptions = vec![
                MqttSubscription { enabled: true, topic: String::from("a/#"), qos: QoS::AtLeastOnce },
                MqttSubscription { enabled: false, topic: String::from("b/#"), qos: QoS::AtMostOnce },
            ];
        });

        assert_eq!(prepare(&mut subscriptions).unwrap().subscriptions, vec![(String::from("a/#"), QoS::AtLeastOnce)]);
    }

    #[test]
    fn timeout_is_rounded_up_to_whole_seconds() {
        for (timeout_ms, seconds) in [(0, 1), (1, 1), (1000, 1), (1001, 2), (30000, 30)] {
            let mut timeout = request("mqtt://broker", |_| {});
            timeout.settings.timeout = Setting::U32(timeout_ms);

            assert_eq!(prepare(&mut timeout).unwrap().connection_timeout, seconds, "{timeout_ms} ms");
        }
    }

    #[test]
    fn tls_settings_are_passed_on() {
        let mut settings = request("mqtts://broker", |_| {});
        settings.settings.accept_invalid_certs = Setting::Bool(true);

        let prepared = prepare(&mut settings).unwrap();
        assert!(prepared.accept_invalid_certs);
        assert!(!prepared.accept_invalid_hostnames);
    }

    #[test]
    fn environment_variables_are_replaced() {
        let mut app = App::new().unwrap();
        app.environments.push(Arc::new(RwLock::new(Environment {
            name: String::from("env"),
            values: IndexMap::from([
                (String::from("HOST"), String::from("broker")),
                (String::from("ID"), String::from("client")),
                (String::from("TOPIC"), String::from("sensors")),
                (String::from("BROKER_USER"), String::from("user")),
            ]),
            path: Default::default(),
        })));

        let mut with_variables = request("mqtts://{{HOST}}:8886", |mqtt_request| {
            mqtt_request.client_id = String::from("{{ID}}");
            mqtt_request.subscriptions = vec![MqttSubscription { enabled: true, topic: String::from("{{TOPIC}}/#"), qos: QoS::AtMostOnce }];
        });
        with_variables.auth = Auth::BasicAuth(BasicAuth { username: String::from("{{BROKER_USER}}"), password: String::new() });

        let prepared = app.prepare_mqtt_request(&mut with_variables).unwrap();

        assert_eq!(prepared.host, "broker");
        assert_eq!(prepared.client_id, "client");
        assert_eq!(prepared.subscriptions[0].0, "sensors/#");
        assert_eq!(prepared.credentials, Some((String::from("user"), String::new())));
    }

    #[test]
    fn pre_request_script_changing_the_protocol_is_an_error() {
        let mut script = request("mqtt://broker", |_| {});
        script.scripts.pre_request_script = Some(String::from(r#"request.protocol = { "type": "http", "method": "GET", "body": "no_body" };"#));

        assert!(matches!(prepare(&mut script), Err(PrepareRequestError::PreRequestScriptChangedProtocol)));
    }

    /// Returns the connection receiver to look at what would be sent to the broker
    fn connected(version: MqttVersion, max_packet_size: u32) -> (MqttRequest, tokio::sync::mpsc::UnboundedReceiver<MqttCommand>) {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let mqtt_request = MqttRequest {
            version,
            max_packet_size,
            connection: Some(sender),
            is_connected: true,
            ..Default::default()
        };

        (mqtt_request, receiver)
    }

    #[test]
    fn publish_up_to_the_max_packet_size() {
        // One byte topic, 9 bytes of headers in 3.1.1 and 10 in MQTT 5
        for (version, largest_payload) in [(MqttVersion::V3_1_1, 90), (MqttVersion::V5, 89)] {
            let (mut mqtt_request, mut receiver) = connected(version, 100);

            mqtt_publish(&mut mqtt_request, String::from("t"), MqttPayload::Text("x".repeat(largest_payload)), QoS::AtLeastOnce, false);
            assert!(matches!(receiver.try_recv(), Ok(MqttCommand::Publish { .. })), "{version}");
            assert!(matches!(mqtt_request.messages.last().unwrap().content, MqttMessageContent::Publish { .. }));

            mqtt_publish(&mut mqtt_request, String::from("t"), MqttPayload::Text("x".repeat(largest_payload + 1)), QoS::AtLeastOnce, false);
            assert!(receiver.try_recv().is_err(), "{version}");
            assert!(matches!(&mqtt_request.messages.last().unwrap().content, MqttMessageContent::Event(event) if event.starts_with("Not published")));
        }
    }

    #[test]
    fn the_smaller_of_the_request_and_broker_limits_applies() {
        // MQTT 5, one byte topic: a payload of limit - 11 bytes fits exactly
        let (mut mqtt_request, mut receiver) = connected(MqttVersion::V5, 1000);
        mqtt_request.broker_max_packet_size = Some(100);

        mqtt_publish(&mut mqtt_request, String::from("t"), MqttPayload::Text("x".repeat(89)), QoS::AtMostOnce, false);
        assert!(matches!(receiver.try_recv(), Ok(MqttCommand::Publish { .. })));

        mqtt_publish(&mut mqtt_request, String::from("t"), MqttPayload::Text("x".repeat(90)), QoS::AtMostOnce, false);
        assert!(receiver.try_recv().is_err());
        assert!(matches!(&mqtt_request.messages.last().unwrap().content, MqttMessageContent::Event(event) if event.contains("the broker's 100 bytes")));

        // A broker allowing more doesn't raise the request's own limit
        mqtt_request.broker_max_packet_size = Some(100_000);
        mqtt_publish(&mut mqtt_request, String::from("t"), MqttPayload::Text("x".repeat(990)), QoS::AtMostOnce, false);
        assert!(receiver.try_recv().is_err());
        assert!(matches!(&mqtt_request.messages.last().unwrap().content, MqttMessageContent::Event(event) if event.contains("the 1000 bytes")));
    }

    #[test]
    fn publish_is_ignored_when_not_connected() {
        let mut mqtt_request = MqttRequest::default();

        mqtt_publish(&mut mqtt_request, String::from("t"), MqttPayload::Text(String::from("x")), QoS::AtMostOnce, false);
        assert!(mqtt_request.messages.is_empty());
    }

    #[test]
    fn publish_after_the_connection_ended_is_not_logged() {
        let (mut mqtt_request, receiver) = connected(MqttVersion::V3_1_1, 100);
        drop(receiver);

        mqtt_publish(&mut mqtt_request, String::from("t"), MqttPayload::Text(String::from("x")), QoS::AtMostOnce, false);
        assert!(mqtt_request.messages.is_empty());
    }
}
