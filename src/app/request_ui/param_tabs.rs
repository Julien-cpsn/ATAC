use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::prelude::{Color, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use crate::app::app::App;
use crate::app::app_states::AppState::*;
use crate::request::auth::Auth::*;
use crate::request::body::ContentType::*;
use crate::request::request::Request;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum RequestParamsTabs {
    #[default]
    #[strum(to_string = "Params")]
    Params,
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
    pub fn next_request_param_tab(&mut self) {
        self.request_param_tab = match self.request_param_tab {
            RequestParamsTabs::Params => RequestParamsTabs::Auth,
            RequestParamsTabs::Auth => RequestParamsTabs::Headers,
            RequestParamsTabs::Headers => RequestParamsTabs::Body,
            RequestParamsTabs::Body => RequestParamsTabs::Cookies,
            RequestParamsTabs::Cookies => RequestParamsTabs::Params
        };

        self.load_request_param_tab();
    }

    pub fn load_request_param_tab(&mut self) {
        self.update_inputs();

        match self.request_param_tab {
            RequestParamsTabs::Params => {}
            RequestParamsTabs::Auth => self.load_request_auth_param_tab(),
            RequestParamsTabs::Headers => {}
            RequestParamsTabs::Body => self.load_request_body_param_tab(),
            RequestParamsTabs::Cookies => {}
        }
    }

    pub fn load_request_auth_param_tab(&mut self) {
        self.auth_text_input_selection.selected = 0;

        self.request_param_tab = RequestParamsTabs::Auth;

        self.update_inputs();
    }

    pub fn load_request_body_param_tab(&mut self) {
        self.request_param_tab = RequestParamsTabs::Body;

        self.update_inputs();
    }

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
                    RequestParamsTabs::Params => tab.to_string(),
                    RequestParamsTabs::Auth => match &request.auth {
                        NoAuth => tab.to_string(),
                        BasicAuth(_, _) | BearerToken(_) => format!("{} ({})", tab.to_string(), &request.auth.to_string())
                    },
                    RequestParamsTabs::Headers => tab.to_string(),
                    RequestParamsTabs::Body => match &request.body {
                        NoBody => tab.to_string(),
                        Raw(_) | JSON(_) | XML(_) | HTML(_) => format!("{} ({})", tab.to_string(), &request.body.to_string())
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
            RequestParamsTabs::Params => {}
            RequestParamsTabs::Auth => {
                match &request.auth {
                    NoAuth => {
                        let body_paragraph = Paragraph::new("\nNo auth").centered();
                        frame.render_widget(body_paragraph, request_params_layout[1]);
                    }
                    BasicAuth(_, _) => {
                        let basic_auth_layout = Layout::new(
                            Vertical,
                            [
                                Constraint::Length(3),
                                Constraint::Length(3),
                            ]
                        )
                            .vertical_margin(1)
                            .horizontal_margin(4)
                            .split(request_params_layout[1]);

                        let mut username_block = Block::new()
                            .title("Username")
                            .borders(Borders::ALL);


                        let mut password_block = Block::new()
                            .title("Password")
                            .borders(Borders::ALL);


                        let mut should_color_blocks = false;
                        let mut should_display_cursor = false;

                        // Prevent from rendering the cursor while no input text has been selected
                        match self.state {
                            SelectedRequest => {
                                should_color_blocks = true;
                            },
                            EditingRequestAuthUsername | EditingRequestAuthPassword => {
                                should_color_blocks = true;
                                should_display_cursor = true;
                            },
                            _ => {}
                        };

                        let input_selected = self.auth_text_input_selection.selected;

                        let input_cursor_position = match input_selected {
                            0 if should_color_blocks => {
                                username_block = username_block.yellow();
                                self.auth_basic_username_text_input.cursor_position as u16
                            },
                            1 if should_color_blocks => {
                                password_block = password_block.yellow();
                                self.auth_basic_password_text_input.cursor_position as u16
                            },
                            _ => 0
                        };

                        if should_display_cursor {
                            frame.set_cursor(
                                basic_auth_layout[input_selected].x + input_cursor_position + 1,
                                basic_auth_layout[input_selected].y + 1
                            );
                        }

                        let username_paragraph = Paragraph::new(self.auth_basic_username_text_input.text.as_str()).block(username_block);
                        let password_paragraph = Paragraph::new(self.auth_basic_password_text_input.text.as_str()).block(password_block);

                        frame.render_widget(username_paragraph, basic_auth_layout[0]);
                        frame.render_widget(password_paragraph, basic_auth_layout[1]);
                    }
                    BearerToken(_) => {}
                }
            }
            RequestParamsTabs::Headers => {}
            RequestParamsTabs::Body => {
                match &request.body {
                    NoBody => {
                        let body_paragraph = Paragraph::new("\nNo body").centered();
                        frame.render_widget(body_paragraph, request_params_layout[1]);
                    }
                    Raw(_) | JSON(_) | XML(_) | HTML(_) => {
                        self.body_text_area.set_line_number_style(Style::new().fg(Color::DarkGray));
                        frame.render_widget(self.body_text_area.widget(), request_params_layout[1]);
                    }
                }
            }
            RequestParamsTabs::Cookies => {}
        }
    }
}