use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::{Color, Style};
use ratatui::style::Color::{Yellow};
use ratatui::style::{Modifier, Stylize};
use ratatui::text::{Line};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs};
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

        self.load_a_request_param_tab();
    }

    pub fn load_a_request_param_tab(&mut self) {
        match self.request_param_tab {
            RequestParamsTabs::Params => self.load_request_params_tab(),
            RequestParamsTabs::Auth => self.load_request_auth_param_tab(),
            RequestParamsTabs::Headers => {}
            RequestParamsTabs::Body => self.load_request_body_param_tab(),
            RequestParamsTabs::Cookies => {}
        }
    }

    pub fn load_request_params_tab(&mut self) {
        self.update_params_selection();

        self.request_param_tab = RequestParamsTabs::Params;
        self.update_inputs();
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

    pub(super) fn render_request_params(&mut self, frame: &mut Frame, rect: Rect, request: &Request) {
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
                    RequestParamsTabs::Params => match request.params.is_empty() {
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
            RequestParamsTabs::Params => {
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
                        let params_layout = Layout::new(
                            Vertical,
                            [
                                Constraint::Length(2),
                                Constraint::Fill(1)
                            ]
                        )
                            .split(request_params_layout[1]);

                        let header_layout = Layout::new(
                            Horizontal,
                            [
                                Constraint::Percentage(50),
                                Constraint::Percentage(50)
                            ]
                        )
                            .split(params_layout[0]);

                        let header_param = Paragraph::new("Param")
                            .centered()
                            .block(Block::new().borders(Borders::BOTTOM | Borders::RIGHT))
                            .dark_gray();
                        let header_value = Paragraph::new("Value")
                            .centered()
                            .block(Block::new().borders(Borders::BOTTOM))
                            .dark_gray();

                        frame.render_widget(header_param, header_layout[0]);
                        frame.render_widget(header_value, header_layout[1]);

                        let horizontal_margin = 2;

                        let table_layout = Layout::new(
                            Horizontal,
                            [
                                Constraint::Percentage(50),
                                Constraint::Percentage(50)
                            ]
                        )
                            .horizontal_margin(horizontal_margin)
                            .split(params_layout[1]);

                        let mut params: Vec<ListItem> = vec![];
                        let mut values: Vec<ListItem> = vec![];

                        for param in request.params.iter() {
                            let mut key = ListItem::from(param.data.0.clone());
                            let mut value = ListItem::from(param.data.1.clone());

                            if !param.enabled {
                                key = key.dark_gray().dim();
                                value = value.dark_gray().dim();
                            }

                            params.push(key);
                            values.push(value);
                        }

                        let mut left_list_style = Style::default();
                        let mut right_list_style = Style::default();

                        match param_selection.1 {
                            0 => left_list_style = left_list_style.fg(Yellow).add_modifier(Modifier::BOLD),
                            1 => right_list_style = right_list_style.fg(Yellow).add_modifier(Modifier::BOLD),
                            _ => {}
                        }

                        let left_list = List::new(params)
                            .highlight_style(left_list_style);

                        let right_list = List::new(values)
                            .highlight_style(right_list_style);

                        frame.render_stateful_widget(left_list, table_layout[0], &mut self.request_param_table.left_state);
                        frame.render_stateful_widget(right_list, table_layout[1], &mut self.request_param_table.right_state);

                        // Param input & cursor

                        if self.state == EditingRequestParam {
                            let cell_with = params_layout[1].width / 2;

                            let width_adjustment = match param_selection.1 {
                                0 => 0,
                                1 => {
                                    let even_odd_adjustment = match params_layout[1].width % 2 {
                                        1 => 1,
                                        0 => 2,
                                        _ => 0
                                    };
                                    cell_with - even_odd_adjustment
                                },
                                _ => 0
                            };

                            let height_adjustment = (param_selection.0 - self.request_param_table.left_state.offset()) as u16 % params_layout[1].height;

                            let selection_position_x = params_layout[1].x + width_adjustment + horizontal_margin;
                            let selection_position_y = params_layout[1].y + height_adjustment;

                            let param_text = self.request_param_table.param_selection_text_input.text.clone();

                            let text_input = Paragraph::new(format!("{:fill$}", param_text, fill = (cell_with - horizontal_margin) as usize));
                            let text_rect = Rect::new(selection_position_x, selection_position_y, cell_with, 1);

                            frame.render_widget(text_input, text_rect);

                            frame.set_cursor(
                                selection_position_x + self.request_param_table.param_selection_text_input.cursor_position as u16,
                                selection_position_y
                            );
                        }
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
                    BearerToken(_) => {
                        let bearer_token_auth_layout = Layout::new(
                            Vertical,
                            [
                                Constraint::Length(3),
                            ]
                        )
                            .vertical_margin(1)
                            .horizontal_margin(4)
                            .split(request_params_layout[1]);

                        let mut bearer_token_block = Block::new()
                            .title("Bearer token")
                            .borders(Borders::ALL);

                        let mut should_color_blocks = false;
                        let mut should_display_cursor = false;

                        // Prevent from rendering the cursor while no input text has been selected
                        match self.state {
                            SelectedRequest => {
                                should_color_blocks = true;
                            },
                            EditingRequestAuthBearerToken => {
                                should_color_blocks = true;
                                should_display_cursor = true;
                            },
                            _ => {}
                        };

                        let input_selected = self.auth_text_input_selection.selected;

                        let input_cursor_position = match input_selected {
                            0 if should_color_blocks => {
                                bearer_token_block = bearer_token_block.yellow();
                                self.auth_bearer_token_text_input.cursor_position as u16
                            },
                            _ => 0
                        };

                        if should_display_cursor {
                            frame.set_cursor(
                                bearer_token_auth_layout[input_selected].x + input_cursor_position + 1,
                                bearer_token_auth_layout[input_selected].y + 1
                            );
                        }

                        let username_paragraph = Paragraph::new(self.auth_bearer_token_text_input.text.as_str()).block(bearer_token_block);

                        frame.render_widget(username_paragraph, bearer_token_auth_layout[0]);
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