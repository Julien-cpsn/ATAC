use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::prelude::{Color, Style};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

use crate::app::app::App;
use crate::request::auth::Auth::*;
use crate::request::body::ContentType::*;
use crate::request::request::Request;

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
    Body
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
                match tab {
                    RequestParamsTabs::QueryParams => match request.params.is_empty() {
                        true => tab.to_string(),
                        false => format!("{} ({})", tab.to_string(), request.params.len())
                    },
                    RequestParamsTabs::Auth => match request.auth {
                        NoAuth => tab.to_string(),
                        BasicAuth(_, _) | BearerToken(_) => format!("{} ({})", tab.to_string(), request.auth.to_string())
                    },
                    RequestParamsTabs::Headers => match request.headers.is_empty() {
                        true => tab.to_string(),
                        false => format!("{} ({})", tab.to_string(), request.headers.len())
                    },
                    RequestParamsTabs::Body => match request.body {
                        NoBody => tab.to_string(),
                        Multipart(_) | Form(_) | File(_) | Raw(_) | Json(_) | Xml(_) | Html(_) | Javascript(_) => format!("{} ({})", tab.to_string(), request.body.to_string())
                    }
                }
            });

        let selected_param_tab_index = self.request_param_tab as usize;

        let params_tabs = Tabs::new(param_tabs)
            .highlight_style(Style::default().yellow())
            .select(selected_param_tab_index)
            .block(
                Block::new().borders(Borders::BOTTOM)
            );

        frame.render_widget(params_tabs, request_params_layout[0]);

        // REQUEST PARAM TABS CONTENT

        match self.request_param_tab {
            RequestParamsTabs::QueryParams => {
                match self.query_params_table.selection {
                    None => {
                        let params_lines = vec![
                            Line::default(),
                            Line::from("No params"),
                            Line::from("(Add one with n or via the URL)".dark_gray())
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
                            Line::from("No auth"),
                            Line::from("(Change auth method with ^a)".dark_gray())
                        ];

                        let auth_paragraph = Paragraph::new(auth_lines).centered();

                        frame.render_widget(auth_paragraph, request_params_layout[1]);
                    }
                    BasicAuth(_, _) => {
                        self.render_basic_auth_tab(frame, request_params_layout[1]);
                    }
                    BearerToken(_) => {
                        self.render_bearer_token_tab(frame, request_params_layout[1]);
                    }
                }
            }
            RequestParamsTabs::Headers => {
                match self.headers_table.selection {
                    None => {
                        let headers_lines = vec![
                            Line::default(),
                            Line::from("Default headers"),
                            Line::from("(Add one with n)".dark_gray())
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
                            Line::from("No body"),
                            Line::from("(Change body type with ^b)".dark_gray())
                        ];

                        let body_paragraph = Paragraph::new(body_lines).centered();

                        frame.render_widget(body_paragraph, request_params_layout[1]);
                    }
                    Multipart(form) | Form(form) => {
                        match self.body_form_table.selection {
                            None => {
                                let multipart_form_lines = vec![
                                    Line::default(),
                                    Line::from("No form data"),
                                    Line::from("(Add one with n)".dark_gray())
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
                        self.body_text_area.set_line_number_style(Style::new().fg(Color::DarkGray));
                        frame.render_widget(self.body_text_area.widget(), request_params_layout[1]);
                    }
                }
            }
        }
    }

    pub fn copy_as_curl_command(&mut self) {
        let mut ctx = ClipboardContext::new().unwrap();

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        let selected_request = local_selected_request.write().unwrap();
        let mut curl_command =format!(
                "curl -X {} \"{}\"",
                selected_request.method,
                selected_request.url_with_params_to_string()
            );

        if !selected_request.headers.is_empty() {
            let headers = selected_request.headers
                .iter()
                .map(|key_value| {
                 format!(" \\\n-H \"{}: {}\"", key_value.data.0, key_value.data.1)})
                .collect::<Vec<String>>()
                .join("");
            curl_command.push_str(&headers);
        } 

        match &selected_request.auth {
            NoAuth => {},
            BasicAuth(username, password) => {
                curl_command.push_str(&format!(" \\\n-u {}:{}", username, password));
            },
            BearerToken(token) => {
                curl_command.push_str(&format!(" \\\n-H \"Authorization: Bearer {}\"", token));
            }
        }

         match &selected_request.body{
            NoBody => {},
            Multipart(form) | Form(form) => {
                let form_data = form.iter().map(|key_value| {
                    format!(" \\\n-F \"{}={}\"", key_value.data.0, key_value.data.1)
                }).collect::<Vec<String>>().join("");
                curl_command.push_str(&form_data);
            }
            Raw(content) | Json(content) => {
                if content != "" {
                    let res = format!(" \\\n--data-raw '{}'",content);
                    curl_command.push_str(&res);
                }
            },
            Html(content) | Javascript(content) | Xml(content) | File(content) => {
                if content != "" {
                    let res = format!(" \\\n--data-binary '{}'",content);
                    curl_command.push_str(&res);
                }
            },
        }
        ctx.set_contents(curl_command).unwrap();
    }
}
