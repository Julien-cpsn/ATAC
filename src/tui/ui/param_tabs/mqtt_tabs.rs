use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::Frame;

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::models::request::Request;
use crate::tui::app_states::AppState::{EditingMqttFormField, EditingRequestMessage, SelectedRequest};
use crate::tui::tui_logic::request::mqtt::MqttFormField;
use crate::tui::utils::stateful::text_input::{MultiLineTextInput, SingleLineTextInput};
use crate::tui::utils::syntax_highlighting::ENV_VARIABLE_SYNTAX_REF;

impl App<'_> {
    pub(super) fn render_mqtt_connection_tab(&mut self, frame: &mut Frame, area: Rect, request: &Request) {
        let layout = Layout::new(
            Vertical,
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(4)
            .split(area);

        let first_row = Layout::new(Horizontal, [Constraint::Percentage(50), Constraint::Percentage(50)]).split(layout[0]);
        let last_row = Layout::new(Horizontal, [Constraint::Percentage(50), Constraint::Percentage(50)]).split(layout[2]);

        self.render_mqtt_form_field(frame, first_row[0], request, MqttFormField::Version);
        self.render_mqtt_form_field(frame, first_row[1], request, MqttFormField::CleanSession);
        self.render_mqtt_form_field(frame, layout[1], request, MqttFormField::ClientId);
        self.render_mqtt_form_field(frame, last_row[0], request, MqttFormField::KeepAlive);
        self.render_mqtt_form_field(frame, last_row[1], request, MqttFormField::MaxPacketSize);
        self.render_mqtt_form_field(frame, layout[3], request, MqttFormField::SessionExpiry);
    }

    pub(super) fn render_mqtt_publish_tab(&mut self, frame: &mut Frame, area: Rect, request: &Request) {
        let layout = Layout::new(
            Vertical,
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Fill(1),
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(4)
            .split(area);

        let options_row = Layout::new(Horizontal, [Constraint::Percentage(50), Constraint::Percentage(50)]).split(layout[1]);

        self.render_mqtt_form_field(frame, layout[0], request, MqttFormField::PublishTopic);
        self.render_mqtt_form_field(frame, options_row[0], request, MqttFormField::PublishQoS);
        self.render_mqtt_form_field(frame, options_row[1], request, MqttFormField::PublishRetain);

        // PAYLOAD

        let mqtt_request = request.get_mqtt_request().unwrap();
        let is_selected = self.get_selected_mqtt_form_field() == Some(MqttFormField::PublishPayload);

        let hint = match mqtt_request.is_connected {
            true => "Enter to edit, confirm to publish",
            false => "Connect to publish",
        };

        let payload_block = Block::new()
            .title(format!("{} ({})", MqttFormField::PublishPayload.label(), mqtt_request.payload))
            .title_bottom(hint)
            .borders(Borders::ALL)
            .fg(match is_selected && matches!(self.state, SelectedRequest | EditingRequestMessage) {
                true => THEME.read().others.selection_highlight_color,
                false => THEME.read().ui.main_foreground_color,
            });

        let payload_area = payload_block.inner(layout[2]);
        frame.render_widget(payload_block, layout[2]);

        self.message_text_area.display_cursor = matches!(self.state, EditingRequestMessage);
        frame.render_widget(MultiLineTextInput(&mut self.message_text_area, ENV_VARIABLE_SYNTAX_REF.clone()), payload_area);
    }

    fn render_mqtt_form_field(&mut self, frame: &mut Frame, area: Rect, request: &Request, field: MqttFormField) {
        let is_selected = self.get_selected_mqtt_form_field() == Some(field);

        // The selected text field is being edited
        if is_selected && self.state == EditingMqttFormField {
            self.mqtt_form_text_input.highlight_text = true;
            self.mqtt_form_text_input.highlight_block = true;
            self.mqtt_form_text_input.display_cursor = true;

            frame.render_widget(SingleLineTextInput(&mut self.mqtt_form_text_input), area);
            return;
        }

        let mqtt_request = request.get_mqtt_request().unwrap();
        let value = field.value(mqtt_request);
        let should_highlight = is_selected && self.state == SelectedRequest;

        let value_paragraph = match (field, value.is_empty()) {
            (MqttFormField::ClientId, true) => Paragraph::new("Assigned by the broker").fg(THEME.read().ui.secondary_foreground_color),
            (_, _) if field.is_text() => Paragraph::new(value).fg(THEME.read().ui.font_color),
            // Values that are cycled through
            (_, _) => Paragraph::new(format!("< {value} >")).centered().fg(THEME.read().ui.font_color),
        };

        let block = Block::new()
            .title(field.label())
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .fg(match should_highlight {
                true => THEME.read().others.selection_highlight_color,
                false => THEME.read().ui.main_foreground_color,
            });

        frame.render_widget(value_paragraph.block(block), area);
    }
}
