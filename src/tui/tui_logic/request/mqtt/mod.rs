use tracing::info;
use crate::app::app::App;
use crate::app::business_logic::request::mqtt::send::mqtt_publish;
use crate::models::protocol::mqtt::mqtt::{MqttRequest, MqttSubscription, MqttVersion, QoS};
use crate::models::protocol::mqtt::payload::{next_payload_type, MqttPayload};
use crate::models::request::KeyValue;
use crate::tui::ui::param_tabs::param_tabs::RequestParamsTabs;

#[derive(Clone, Copy, PartialEq)]
pub enum MqttFormField {
    Version,
    ClientId,
    CleanSession,
    SessionExpiry,
    KeepAlive,
    MaxPacketSize,
    PublishTopic,
    PublishQoS,
    PublishRetain,
    PublishPayload,
}

impl MqttFormField {
    pub fn label(&self) -> &'static str {
        match self {
            MqttFormField::Version => "Version",
            MqttFormField::ClientId => "Client ID",
            MqttFormField::CleanSession => "Clean session",
            MqttFormField::SessionExpiry => "Session expiry (s, MQTT 5)",
            MqttFormField::KeepAlive => "Keep alive (s)",
            MqttFormField::MaxPacketSize => "Max packet size",
            MqttFormField::PublishTopic => "Topic",
            MqttFormField::PublishQoS => "QoS",
            MqttFormField::PublishRetain => "Retain",
            MqttFormField::PublishPayload => "Payload",
        }
    }

    pub fn value(&self, mqtt_request: &MqttRequest) -> String {
        match self {
            MqttFormField::Version => mqtt_request.version.to_string(),
            MqttFormField::ClientId => mqtt_request.client_id.clone(),
            MqttFormField::CleanSession => mqtt_request.clean_session.to_string(),
            MqttFormField::SessionExpiry => mqtt_request.session_expiry_interval.to_string(),
            MqttFormField::KeepAlive => mqtt_request.keep_alive.to_string(),
            MqttFormField::MaxPacketSize => mqtt_request.max_packet_size.to_string(),
            MqttFormField::PublishTopic => mqtt_request.publish.topic.clone(),
            MqttFormField::PublishQoS => mqtt_request.publish.qos.to_string(),
            MqttFormField::PublishRetain => mqtt_request.publish.retain.to_string(),
            MqttFormField::PublishPayload => mqtt_request.payload.to_content(),
        }
    }

    /// Fields edited with a text input, the others are cycled through
    pub fn is_text(&self) -> bool {
        matches!(self, MqttFormField::ClientId | MqttFormField::SessionExpiry | MqttFormField::KeepAlive | MqttFormField::MaxPacketSize | MqttFormField::PublishTopic)
    }
}

pub const MQTT_CONNECTION_FIELDS: [MqttFormField; 6] = [
    MqttFormField::Version,
    MqttFormField::CleanSession,
    MqttFormField::ClientId,
    MqttFormField::KeepAlive,
    MqttFormField::MaxPacketSize,
    MqttFormField::SessionExpiry,
];

pub const MQTT_PUBLISH_FIELDS: [MqttFormField; 4] = [
    MqttFormField::PublishTopic,
    MqttFormField::PublishQoS,
    MqttFormField::PublishRetain,
    MqttFormField::PublishPayload,
];

pub fn get_mqtt_form_fields(request_param_tab: RequestParamsTabs) -> &'static [MqttFormField] {
    match request_param_tab {
        RequestParamsTabs::MqttConnection => &MQTT_CONNECTION_FIELDS,
        RequestParamsTabs::MqttPublish => &MQTT_PUBLISH_FIELDS,
        _ => &[]
    }
}

fn next_qos(qos: QoS, forward: bool) -> QoS {
    match (qos, forward) {
        (QoS::AtMostOnce, true) | (QoS::ExactlyOnce, false) => QoS::AtLeastOnce,
        (QoS::AtLeastOnce, true) | (QoS::AtMostOnce, false) => QoS::ExactlyOnce,
        (QoS::ExactlyOnce, true) | (QoS::AtLeastOnce, false) => QoS::AtMostOnce,
    }
}

fn qos_to_digit(qos: QoS) -> String {
    match qos {
        QoS::AtMostOnce => String::from("0"),
        QoS::AtLeastOnce => String::from("1"),
        QoS::ExactlyOnce => String::from("2"),
    }
}

pub fn subscriptions_to_rows(subscriptions: &[MqttSubscription]) -> Vec<KeyValue> {
    subscriptions
        .iter()
        .map(|subscription| KeyValue {
            enabled: subscription.enabled,
            data: (subscription.topic.clone(), qos_to_digit(subscription.qos)),
        })
        .collect()
}

impl App<'_> {
    pub fn is_selected_request_mqtt(&self) -> bool {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        selected_request.get_mqtt_request().is_ok()
    }

    pub fn get_selected_mqtt_form_field(&self) -> Option<MqttFormField> {
        get_mqtt_form_fields(self.request_param_tab)
            .get(self.mqtt_form_selection.selected)
            .copied()
    }

    pub fn tui_load_request_mqtt_tab(&mut self, request_param_tab: RequestParamsTabs) {
        self.mqtt_form_selection.selected = 0;
        self.request_param_tab = request_param_tab;
        self.update_inputs();
    }

    /// Publishes the payload being edited to the publish topic
    pub fn tui_publish_mqtt_message(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();
            let mqtt_request = selected_request.get_mqtt_request_mut().unwrap();

            let lines = self.message_text_area.to_lines();

            mqtt_request.payload = match mqtt_request.payload {
                MqttPayload::Text(_) => MqttPayload::Text(lines.join("\n")),
                MqttPayload::Binary(_) => MqttPayload::Binary(lines.join("").as_bytes().to_vec().into_boxed_slice()),
            };

            if mqtt_request.is_connected {
                let topic = self.replace_env_keys_by_value(&mqtt_request.publish.topic);
                let payload = mqtt_request.payload.clone();
                let (qos, retain) = (mqtt_request.publish.qos, mqtt_request.publish.retain);

                info!("Publishing MQTT message");
                mqtt_publish(mqtt_request, topic, payload, qos, retain);

                *self.received_response.lock() = true;
            }
            else {
                info!("MQTT client is not connected");
            }
        }

        self.select_request_state();
    }

    pub fn tui_next_mqtt_payload_type(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();
            let mqtt_request = selected_request.get_mqtt_request_mut().unwrap();

            mqtt_request.payload = next_payload_type(&mqtt_request.payload);

            info!("Payload type set to \"{}\"", mqtt_request.payload);
        }

        self.update_inputs();
    }

    /// Edits text fields, cycles through the values of the other ones
    pub fn tui_edit_mqtt_form_field(&mut self) {
        let Some(field) = self.get_selected_mqtt_form_field() else {
            return;
        };

        match field {
            MqttFormField::PublishPayload => self.edit_request_message_state(),
            _ if field.is_text() => self.edit_mqtt_form_field_state(),
            _ => self.tui_change_mqtt_form_field_value(true),
        }
    }

    pub fn tui_change_mqtt_form_field_value(&mut self, forward: bool) {
        let Some(field) = self.get_selected_mqtt_form_field() else {
            return;
        };

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();
            let mqtt_request = selected_request.get_mqtt_request_mut().unwrap();

            match field {
                MqttFormField::Version => mqtt_request.version = match mqtt_request.version {
                    MqttVersion::V3_1_1 => MqttVersion::V5,
                    MqttVersion::V5 => MqttVersion::V3_1_1,
                },
                MqttFormField::CleanSession => mqtt_request.clean_session = !mqtt_request.clean_session,
                MqttFormField::PublishQoS => mqtt_request.publish.qos = next_qos(mqtt_request.publish.qos, forward),
                MqttFormField::PublishRetain => mqtt_request.publish.retain = !mqtt_request.publish.retain,
                _ => return
            }

            info!("MQTT {} set to \"{}\"", field.label(), field.value(mqtt_request));
        }

        self.save_collection_to_file(selected_request_index.0);
        self.update_inputs();
    }

    pub fn tui_modify_mqtt_form_field(&mut self) {
        let Some(field) = self.get_selected_mqtt_form_field() else {
            return;
        };

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);
        let input_text = self.mqtt_form_text_input.to_string();

        {
            let mut selected_request = local_selected_request.write();
            let mqtt_request = selected_request.get_mqtt_request_mut().unwrap();

            match field {
                MqttFormField::ClientId => mqtt_request.client_id = input_text.trim().to_string(),
                MqttFormField::PublishTopic => mqtt_request.publish.topic = input_text.trim().to_string(),
                MqttFormField::KeepAlive => match input_text.trim().parse::<u16>() {
                    Ok(keep_alive) => mqtt_request.keep_alive = keep_alive,
                    Err(_) => info!("Keep alive must be a number of seconds between 0 and 65535"),
                },
                MqttFormField::SessionExpiry => match input_text.trim().parse::<u32>() {
                    Ok(session_expiry_interval) => mqtt_request.session_expiry_interval = session_expiry_interval,
                    Err(_) => info!("Session expiry must be a number of seconds"),
                },
                MqttFormField::MaxPacketSize => match input_text.trim().parse::<u32>() {
                    Ok(max_packet_size) if max_packet_size > 0 => mqtt_request.max_packet_size = max_packet_size,
                    _ => info!("Max packet size must be a positive number of bytes"),
                },
                _ => {}
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    /* Subscriptions */

    pub fn tui_update_mqtt_subscriptions_selection(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        let is_empty = match selected_request.get_mqtt_request() {
            Ok(mqtt_request) => mqtt_request.subscriptions.is_empty(),
            Err(_) => true
        };

        match is_empty {
            false => self.mqtt_subscriptions_table.update_selection(Some((0, 0))),
            true => self.mqtt_subscriptions_table.update_selection(None)
        }
    }

    fn modify_mqtt_subscriptions(&mut self, modify: impl FnOnce(&mut Vec<MqttSubscription>)) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();
            let mqtt_request = selected_request.get_mqtt_request_mut().unwrap();

            modify(&mut mqtt_request.subscriptions);
        }

        self.save_collection_to_file(selected_request_index.0);
    }

    pub fn tui_create_mqtt_subscription(&mut self) {
        self.modify_mqtt_subscriptions(|subscriptions| subscriptions.push(MqttSubscription {
            enabled: true,
            topic: String::from("topic/#"),
            qos: QoS::AtMostOnce,
        }));

        self.tui_update_mqtt_subscriptions_selection();
        self.update_inputs();
    }

    pub fn tui_delete_mqtt_subscription(&mut self) {
        let Some((row, _)) = self.mqtt_subscriptions_table.selection else {
            return;
        };

        self.modify_mqtt_subscriptions(|subscriptions| {
            subscriptions.remove(row);
        });

        self.tui_update_mqtt_subscriptions_selection();
        self.update_inputs();
    }

    pub fn tui_toggle_mqtt_subscription(&mut self) {
        let Some((row, _)) = self.mqtt_subscriptions_table.selection else {
            return;
        };

        self.modify_mqtt_subscriptions(|subscriptions| subscriptions[row].enabled = !subscriptions[row].enabled);
        self.update_inputs();
    }

    pub fn tui_duplicate_mqtt_subscription(&mut self) {
        let Some((row, _)) = self.mqtt_subscriptions_table.selection else {
            return;
        };

        self.modify_mqtt_subscriptions(|subscriptions| {
            let subscription = subscriptions[row].clone();
            subscriptions.insert(row + 1, subscription);
        });

        self.update_inputs();
    }

    pub fn tui_next_mqtt_subscription_qos(&mut self) {
        let Some((row, _)) = self.mqtt_subscriptions_table.selection else {
            return;
        };

        self.modify_mqtt_subscriptions(|subscriptions| subscriptions[row].qos = next_qos(subscriptions[row].qos, true));
        self.update_inputs();
    }

    pub fn tui_modify_mqtt_subscription(&mut self) {
        let Some((row, column)) = self.mqtt_subscriptions_table.selection else {
            return;
        };

        let input_text = self.mqtt_subscriptions_table.selection_text_input.to_string();

        self.modify_mqtt_subscriptions(|subscriptions| match column {
            0 => subscriptions[row].topic = input_text.trim().to_string(),
            _ => {}
        });

        self.select_request_state();
    }
}
