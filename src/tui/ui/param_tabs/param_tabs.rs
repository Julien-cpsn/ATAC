use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::prelude::Style;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::models::auth::Auth::*;
use crate::models::body::ContentType::*;
use crate::models::request::Request;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
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

        let param_tabs = RequestParamsTabs::iter()
            .map(|tab| {
                let text = match tab {
                    RequestParamsTabs::QueryParams => match request.params.is_empty() {
                        true => tab.to_string(),
                        false => format!("{} ({})", tab.to_string(), request.params.len())
                    },
                    RequestParamsTabs::Auth => match request.auth {
                        NoAuth => tab.to_string(),
                        BasicAuth { .. } | BearerToken { .. } => format!("{} ({})", tab.to_string(), request.auth.to_string())
                    },
                    RequestParamsTabs::Headers => match request.headers.is_empty() {
                        true => tab.to_string(),
                        false => format!("{} ({})", tab.to_string(), request.headers.len())
                    },
                    RequestParamsTabs::Body => match request.body {
                        NoBody => tab.to_string(),
                        Multipart(_) | Form(_) | File(_) | Raw(_) | Json(_) | Xml(_) | Html(_) | Javascript(_) => format!("{} ({})", tab.to_string(), request.body.to_string())
                    },
                    RequestParamsTabs::Scripts => tab.to_string(),
                };

                text.fg(THEME.read().ui.font_color)
            });

        let selected_param_tab_index = self.request_param_tab as usize;

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
                match self.query_params_table.selection {
                    None => {
                        let params_lines = vec![
                            Line::default(),
                            Line::from("No params").fg(THEME.read().ui.font_color),
                            Line::from("(Add one with n or via the URL)").fg(THEME.read().ui.secondary_foreground_color)
                        ];

                        let params_paragraph = Paragraph::new(params_lines).centered();

                        frame.render_widget(params_paragraph, request_params_layout[1]);
                    },
                    Some(param_selection) => {
                        self.render_query_params_tab(frame, request_params_layout[1], request, param_selection);
                    }
                }
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
                    BasicAuth { .. } => {
                        self.render_basic_auth_tab(frame, request_params_layout[1]);
                    }
                    BearerToken { .. } => {
                        self.render_bearer_token_tab(frame, request_params_layout[1]);
                    }
                }
            }
            RequestParamsTabs::Headers => {
                match self.headers_table.selection {
                    None => {
                        let headers_lines = vec![
                            Line::default(),
                            Line::from("Default headers").fg(THEME.read().ui.font_color),
                            Line::from("(Add one with n)").fg(THEME.read().ui.secondary_foreground_color)
                        ];

                        let headers_paragraph = Paragraph::new(headers_lines).centered();

                        frame.render_widget(headers_paragraph, request_params_layout[1]);
                    },
                    Some(header_selection) => {
                        self.render_headers_tab(frame, request_params_layout[1], request, header_selection);
                    }
                }
            }
            RequestParamsTabs::Body => {
                match &request.body {
                    NoBody => {
                        let body_lines = vec![
                            Line::default(),
                            Line::from("No body").fg(THEME.read().ui.font_color),
                            Line::from("(Change body type with ^b)").fg(THEME.read().ui.secondary_foreground_color)
                        ];

                        let body_paragraph = Paragraph::new(body_lines).centered();

                        frame.render_widget(body_paragraph, request_params_layout[1]);
                    }
                    Multipart(form) | Form(form) => {
                        match self.body_form_table.selection {
                            None => {
                                let multipart_form_lines = vec![
                                    Line::default(),
                                    Line::from("No form data").fg(THEME.read().ui.font_color),
                                    Line::from("(Add one with n)").fg(THEME.read().ui.secondary_foreground_color)
                                ];

                                let multipart_form_paragraph = Paragraph::new(multipart_form_lines).centered();

                                frame.render_widget(multipart_form_paragraph, request_params_layout[1]);
                            },
                            Some(multipart_form_selection) => {
                                self.render_form_body_tab(frame, request_params_layout[1], form, multipart_form_selection);
                            }
                        }
                    },
                    File(_) => {
                      self.render_file_body_tab(frame, request_params_layout[1]);
                    },
                    Raw(_) | Json(_) | Xml(_) | Html(_) | Javascript(_) => {
                        self.body_text_area.set_style(Style::new().fg(THEME.read().ui.font_color));
                        self.body_text_area.set_line_number_style(Style::new().fg(THEME.read().ui.secondary_foreground_color));

                        frame.render_widget(&self.body_text_area, request_params_layout[1]);
                    }
                }
            }
            RequestParamsTabs::Scripts => {
                self.render_request_script(frame, request_params_layout[1]);
            }
        }
    }
}