use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use ratatui::Frame;
use strum::{Display, EnumIter, FromRepr};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::models::auth::auth::Auth::{BasicAuth, BearerToken, Digest, JwtToken, NoAuth};
use crate::models::protocol::http::body::ContentType::*;
use crate::models::protocol::protocol::Protocol;
use crate::models::request::Request;
use crate::tui::app_states::AppState::{EditingRequestBodyString, EditingRequestBodyTable, EditingRequestHeader, EditingRequestMessage, EditingRequestParam};
use crate::tui::tui_logic::utils::key_value_vec_to_items_list;
use crate::tui::utils::stateful::text_input::MultiLineTextInput;
use crate::tui::utils::syntax_highlighting::{ENV_VARIABLE_SYNTAX_REF, HTML_SYNTAX_REF, JSON_SYNTAX_REF, JS_SYNTAX_REF, XML_SYNTAX_REF};

#[derive(Default, Clone, Copy, PartialEq, Display, FromRepr, EnumIter)]
pub enum RequestParamsTabs {
    #[default]
    #[strum(to_string = "Params")]
    QueryParams,
    #[strum(to_string = "Auth")]
    Auth,
    #[strum(to_string = "Headers")]
    Headers,
    #[strum(to_string = "Body")]
    Body,
    #[strum(to_string = "Message")]
    Message,
    #[strum(to_string = "Scripts")]
    Scripts
}

impl App<'_> {
    pub fn render_request_params(&mut self, frame: &mut Frame, rect: Rect, request: &Request) {
        let request_params_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(2),
                Constraint::Fill(1)
            ]
        )
            .split(rect);

        // REQUEST PARAM TABS

        let allowed_tabs = match &request.protocol {
            Protocol::HttpRequest(_) => vec![
                RequestParamsTabs::QueryParams,
                RequestParamsTabs::Auth,
                RequestParamsTabs::Headers,
                RequestParamsTabs::Body,
                RequestParamsTabs::Scripts
            ],
            Protocol::WsRequest(_) => vec![
                RequestParamsTabs::QueryParams,
                RequestParamsTabs::Auth,
                RequestParamsTabs::Headers,
                RequestParamsTabs::Message,
                RequestParamsTabs::Scripts
            ]
        };

        let param_tabs = allowed_tabs
            .iter()
            .map(|tab| {
                let text = match tab {
                    RequestParamsTabs::QueryParams => match request.params.is_empty() {
                        true => tab.to_string(),
                        false => format!("{} ({})", tab.to_string(), request.params.len())
                    },
                    RequestParamsTabs::Auth => match request.auth {
                        NoAuth => tab.to_string(),
                        BasicAuth(_) | BearerToken(_) | JwtToken(_) | Digest(_) => format!("{} ({})", tab.to_string(), request.auth.to_string())
                    },
                    RequestParamsTabs::Headers => match request.headers.is_empty() {
                        true => tab.to_string(),
                        false => format!("{} ({})", tab.to_string(), request.headers.len())
                    },
                    RequestParamsTabs::Body => {
                        let http_request = request.get_http_request().unwrap();

                        match http_request.body {
                            NoBody => tab.to_string(),
                            Multipart(_) | Form(_) | File(_) | Raw(_) | Json(_) | Xml(_) | Html(_) | Javascript(_) => format!("{} ({})", tab.to_string(), http_request.body.to_string())
                        }
                    },
                    RequestParamsTabs::Message => {
                        let ws_request = request.get_ws_request().unwrap();

                        format!("{} ({})", tab.to_string(), ws_request.message_type.to_string())
                    },
                    RequestParamsTabs::Scripts => tab.to_string(),
                };

                text.fg(THEME.read().ui.font_color)
            });

        let selected_param_tab_index = match &request.protocol {
            Protocol::HttpRequest(_) => match self.request_param_tab {
                RequestParamsTabs::QueryParams => 0,
                RequestParamsTabs::Auth => 1,
                RequestParamsTabs::Headers => 2,
                RequestParamsTabs::Body => 3,
                RequestParamsTabs::Scripts => 4,
                _ => unreachable!()
            }
            Protocol::WsRequest(_) => match self.request_param_tab {
                RequestParamsTabs::QueryParams => 0,
                RequestParamsTabs::Auth => 1,
                RequestParamsTabs::Headers => 2,
                RequestParamsTabs::Message => 3,
                RequestParamsTabs::Scripts => 4,
                _ => unreachable!()
            }
        };
        
        let params_tabs = Tabs::new(param_tabs)
            .highlight_style(THEME.read().others.selection_highlight_color)
            .select(selected_param_tab_index)
            .block(
                Block::new().borders(Borders::BOTTOM)
                    .fg(THEME.read().ui.main_foreground_color)
            );

        frame.render_widget(params_tabs, request_params_layout[0]);

        // REQUEST PARAM TABS CONTENT

        match self.request_param_tab {
            RequestParamsTabs::QueryParams => {
                self.query_params_table.is_editing = matches!(&self.state, EditingRequestParam);

                let mut rows = key_value_vec_to_items_list(&self.get_selected_env_as_local(), &self.query_params_table.rows);

                frame.render_stateful_widget(&mut self.query_params_table, request_params_layout[1], &mut rows);
            }
            RequestParamsTabs::Auth => {
                match &request.auth {
                    NoAuth => {
                        let auth_lines = vec![
                            Line::default(),
                            Line::from("No auth").fg(THEME.read().ui.font_color),
                            Line::from("(Change auth method with ^a)").fg(THEME.read().ui.secondary_foreground_color)
                        ];

                        let auth_paragraph = Paragraph::new(auth_lines).centered();

                        frame.render_widget(auth_paragraph, request_params_layout[1]);
                    }
                    BasicAuth(_) => self.render_basic_auth_tab(frame, request_params_layout[1]),
                    BearerToken(_) => self.render_bearer_token_tab(frame, request_params_layout[1]),
                    JwtToken(_) => self.render_jwt_token_tab(frame, request_params_layout[1]),
                    Digest(_) => self.render_digest_tab(frame, request_params_layout[1]),
                }
            }
            RequestParamsTabs::Headers => {
                self.headers_table.is_editing = matches!(self.state, EditingRequestHeader);

                let mut rows = key_value_vec_to_items_list(&self.get_selected_env_as_local(), &self.headers_table.rows);

                frame.render_stateful_widget(&mut self.headers_table, request_params_layout[1], &mut rows);
            }
            RequestParamsTabs::Body => {
                let http_request = request.get_http_request().unwrap();

                match &http_request.body {
                    NoBody => {
                        let body_lines = vec![
                            Line::default(),
                            Line::from("No body").fg(THEME.read().ui.font_color),
                            Line::from("(Change body type with ^b)").fg(THEME.read().ui.secondary_foreground_color)
                        ];

                        let body_paragraph = Paragraph::new(body_lines).centered();

                        frame.render_widget(body_paragraph, request_params_layout[1]);
                    }
                    Multipart(_) | Form(_) => {
                        self.body_form_table.is_editing = matches!(self.state, EditingRequestBodyTable);

                        let mut rows = key_value_vec_to_items_list(&self.get_selected_env_as_local(), &self.body_form_table.rows);

                        frame.render_stateful_widget(&mut self.body_form_table, request_params_layout[1], &mut rows);
                    },
                    File(_) => {
                      self.render_file_body_tab(frame, request_params_layout[1]);
                    },
                    Raw(_) | Json(_) | Xml(_) | Html(_) | Javascript(_) => {
                        let display_cursor = matches!(&self.state, EditingRequestBodyString);
                        let syntax_reference = match &http_request.body {
                            Raw(_) => ENV_VARIABLE_SYNTAX_REF.clone(),
                            Json(_) => JSON_SYNTAX_REF.clone(),
                            Xml(_) => XML_SYNTAX_REF.clone(),
                            Html(_) => HTML_SYNTAX_REF.clone(),
                            Javascript(_) => JS_SYNTAX_REF.clone(),
                            _ => unreachable!()
                        };
                        
                        self.body_text_area.display_cursor = display_cursor;

                        frame.render_widget(MultiLineTextInput(&mut self.body_text_area, syntax_reference), request_params_layout[1]);
                    }
                }
            },
            RequestParamsTabs::Message => {
                let display_cursor = matches!(&self.state, EditingRequestMessage);

                self.message_text_area.display_cursor = display_cursor;

                frame.render_widget(MultiLineTextInput(&mut self.message_text_area, ENV_VARIABLE_SYNTAX_REF.clone()), request_params_layout[1]);            }
            RequestParamsTabs::Scripts => {
                self.render_request_script(frame, request_params_layout[1]);
            }
        }
    }
}
