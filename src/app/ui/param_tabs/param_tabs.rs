use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::{Vertical};
use ratatui::prelude::{Color, Style};
use ratatui::style::{Stylize};
use ratatui::text::{Line};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
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
    Body,
    #[strum(to_string = "Cookies")]
    Cookies
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
                    RequestParamsTabs::Headers => tab.to_string(),
                    RequestParamsTabs::Body => match request.body {
                        NoBody => tab.to_string(),
                        Raw(_) | Json(_) | Xml(_) | Html(_) => format!("{} ({})", tab.to_string(), request.body.to_string())
                    }
                    RequestParamsTabs::Cookies => tab.to_string(),
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
                match self.request_param_table.selection {
                    None => {
                        let params_lines = vec![
                            Line::default(),
                            Line::from("No params"),
                            Line::from("(Add one via the URL)".dark_gray())
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
            RequestParamsTabs::Headers => {}
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
                    Raw(_) | Json(_) | Xml(_) | Html(_) => {
                        self.body_text_area.set_line_number_style(Style::new().fg(Color::DarkGray));
                        frame.render_widget(self.body_text_area.widget(), request_params_layout[1]);
                    }
                }
            }
            RequestParamsTabs::Cookies => {}
        }
    }
}