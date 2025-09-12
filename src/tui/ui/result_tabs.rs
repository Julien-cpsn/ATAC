use chrono::Local;
use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::prelude::{Alignment, Style};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Tabs, Wrap};
use ratatui::Frame;
use ratatui_image::picker::Picker;
use ratatui_image::StatefulImage;
use rayon::prelude::*;
use strum::{Display, EnumIter, FromRepr};
use textwrap::wrap;
use throbber_widgets_tui::{Throbber, WhichUse, BRAILLE_DOUBLE};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::models::protocol::protocol::Protocol;
use crate::models::protocol::ws::ws::Sender;
use crate::models::request::Request;
use crate::models::response::ResponseContent;
use crate::tui::utils::centered_rect::centered_rect;

#[derive(Default, Clone, Copy, PartialOrd, PartialEq, Display, FromRepr, EnumIter)]
pub enum RequestResultTabs {
    #[default]
    #[strum(to_string = "Result body")]
    Body,
    #[strum(to_string = "Messages")]
    Messages,
    #[strum(to_string = "Cookies")]
    Cookies,
    #[strum(to_string = "Headers")]
    Headers,
    #[strum(to_string = "Console")]
    Console,
}

impl App<'_> {
    pub(super) fn render_request_result(&mut self, frame: &mut Frame, rect: Rect, request: &Request) {
        let request_result_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Fill(1)
            ]
        )
            .split(rect);


        // REQUEST RESULT TABS

        let allowed_tabs = match &request.protocol {
            Protocol::HttpRequest(_) => vec![
                RequestResultTabs::Body,
                RequestResultTabs::Cookies,
                RequestResultTabs::Headers,
                RequestResultTabs::Console
            ],
            Protocol::WsRequest(_) => vec![
                RequestResultTabs::Messages,
                RequestResultTabs::Cookies,
                RequestResultTabs::Headers,
                RequestResultTabs::Console
            ]
        };

        let result_tabs: Vec<Span> = allowed_tabs
            .iter()
            .filter_map(|tab| {
                let text = match tab {
                    RequestResultTabs::Body => {
                        if let Some(duration) = &request.response.duration {
                            Some(format!("{} ({})", tab.to_string(), duration))
                        }
                        else {
                            Some(format!("{}", tab.to_string()))
                        }
                    },
                    RequestResultTabs::Messages => {
                        let ws_request = request.get_ws_request().unwrap();

                        if !ws_request.messages.is_empty() {
                            Some(format!("{} ({})", tab.to_string(), ws_request.messages.len()))
                        }
                        else {
                            Some(format!("{}", tab.to_string()))
                        }
                    },
                    RequestResultTabs::Cookies | RequestResultTabs::Headers => Some(tab.to_string()),
                    RequestResultTabs::Console => {
                        match (&request.console_output.pre_request_output, &request.console_output.post_request_output) {
                            (None, None) => None,
                            (_, _) => Some(tab.to_string())
                        }
                    }
                };

                match text {
                    Some(text) => Some(Span::raw(text).fg(THEME.read().ui.font_color)),
                    None => None
                }
            })
            .collect();

        let selected_result_tab_index = match &request.protocol {
            Protocol::HttpRequest(_) => match self.request_result_tab {
                RequestResultTabs::Body => 0,
                RequestResultTabs::Cookies => 1,
                RequestResultTabs::Headers => 2,
                RequestResultTabs::Console => 3,
                _ => unreachable!()
            }
            Protocol::WsRequest(_) => match self.request_result_tab {
                RequestResultTabs::Messages => 0,
                RequestResultTabs::Cookies => 1,
                RequestResultTabs::Headers => 2,
                RequestResultTabs::Console => 3,
                _ => unreachable!()
            }
        };

        let result_tabs = Tabs::new(result_tabs)
            .highlight_style(THEME.read().others.selection_highlight_color)
            .select(selected_result_tab_index)
            .block(
                Block::new().borders(Borders::BOTTOM)
                    .fg(THEME.read().ui.main_foreground_color)
            );

        frame.render_widget(result_tabs, request_result_layout[0]);

        // If the selected request is currently pending
        if request.is_pending {
            let area = centered_rect(9, 1, request_result_layout[2]);

            self.result_throbber_state.calc_next();
            
            let throbber = Throbber::default()
                .label("Pending")
                .style(Style::new().fg(THEME.read().ui.secondary_foreground_color))
                .throbber_set(BRAILLE_DOUBLE)
                .use_type(WhichUse::Spin);

            frame.render_stateful_widget(throbber, area, &mut self.result_throbber_state);
        }
        // If the selected request is not pending
        else {
            // REQUEST RESULT STATUS CODE

            let status_code = match &request.response.status_code {
                None => "",
                Some(status_code) => status_code
            };

            let status_code_paragraph = Paragraph::new(status_code)
                .centered()
                .fg(THEME.read().ui.secondary_foreground_color);
            frame.render_widget(status_code_paragraph, request_result_layout[1]);


            // REQUEST RESULT CONTENT

            match self.request_result_tab {
                RequestResultTabs::Body => match &request.response.content {
                    None => {},
                    Some(content) => match content {
                        ResponseContent::Body(body) => {
                            let lines: Vec<Line>;
                            if !self.config.is_syntax_highlighting_disabled() && self.syntax_highlighting.highlighted_body.is_some() {
                                lines = self.syntax_highlighting.highlighted_body.clone().unwrap();
                            }
                            else {
                                lines = body.lines().map(|line| Line::raw(line)).collect();
                            }

                            let mut body_paragraph = Paragraph::new(lines);

                            if self.config.should_wrap_body() {
                                body_paragraph = body_paragraph
                                    .wrap(Wrap::default())
                                    .scroll((
                                        self.result_vertical_scrollbar.scroll,
                                        0
                                    ));
                            }
                            else {
                                body_paragraph = body_paragraph
                                    .scroll((
                                        self.result_vertical_scrollbar.scroll,
                                        self.result_horizontal_scrollbar.scroll
                                    ));
                            }

                            frame.render_widget(body_paragraph, request_result_layout[2]);
                        }
                        ResponseContent::Image(image_response) => match &image_response.image {
                            _ if self.config.is_image_preview_disabled() => {
                                let image_disabled_paragraph = Paragraph::new("\nImage preview disabled").centered();
                                frame.render_widget(image_disabled_paragraph, request_result_layout[2]);
                            },
                            Some(image) => {
                                let picker = Picker::from_query_stdio().unwrap_or(Picker::from_fontsize((7, 14)));

                                let mut image_static = picker
                                    .new_resize_protocol(image.clone());

                                frame.render_stateful_widget(StatefulImage::default(), request_result_layout[2], &mut image_static);
                            }
                            None => {
                                let image_error_paragraph = Paragraph::new("\nCould not decode image")
                                    .centered()
                                    .fg(THEME.read().ui.font_color);
                                frame.render_widget(image_error_paragraph, request_result_layout[2]);
                            }
                        },
                    }
                },
                RequestResultTabs::Messages => {
                    let ws_request = request.get_ws_request().unwrap();

                    let mut messages = vec![];
                    let mut last_sender: Option<&Sender> = None;

                    for message in &ws_request.messages {
                        let mut alignment = Alignment::Right;

                        let content = message.content.to_content();
                        let max_length = self.get_max_line_length(&content);
                        let lines = wrap(&content, max_length);

                        match message.sender {
                            Sender::You => {
                                for line in lines {
                                    let line = match line.is_empty() {
                                        true => " ".repeat(max_length),
                                        false => format!("{line:max_length$}")
                                    };


                                    messages.push(
                                        Line::raw(line)
                                            .fg(THEME.read().ui.font_color)
                                            .bg(THEME.read().websocket.messages.you_background_color)
                                            .alignment(alignment)
                                    );
                                }
                            },
                            Sender::Server => {
                                alignment = Alignment::Left;

                                if last_sender != Some(&message.sender) {
                                    messages.push(
                                        Line::raw(message.sender.to_string())
                                            .bold()
                                            .fg(THEME.read().websocket.messages.server_foreground_color)
                                            .alignment(alignment)
                                    );
                                }

                                for line in lines {
                                    let line = match line.is_empty() {
                                        true => " ".repeat(max_length),
                                        false => format!("{line:max_length$}")
                                    };


                                    messages.push(
                                        Line::raw(line)
                                            .fg(THEME.read().ui.font_color)
                                            .bg(THEME.read().websocket.messages.server_background_color)
                                            .alignment(alignment)
                                    );
                                }
                            }
                        }

                        let timestamp_format = match Local::now().date_naive() == message.timestamp.date_naive() {
                            true => "%H:%M:%S",
                            false => "%H:%M:%S %d/%m/%Y"
                        };

                        let timestamp = message.timestamp.format(timestamp_format).to_string();

                        messages.push(
                            Line::raw(format!("{} {}", message.content.to_string(), timestamp))
                                .fg(THEME.read().websocket.messages.details_color)
                                .alignment(alignment)
                        );

                        last_sender = Some(&message.sender);
                    }

                    let messages_paragraph = Paragraph::new(messages)
                        .scroll((
                            self.result_vertical_scrollbar.scroll,
                            self.result_horizontal_scrollbar.scroll
                        ));

                    let inner_area = Rect {
                        x: request_result_layout[2].x,
                        y: request_result_layout[2].y,
                        width: request_result_layout[2].width - 2,
                        height: request_result_layout[2].height,
                    };

                    frame.render_widget(messages_paragraph, inner_area);
                },
                RequestResultTabs::Cookies => {
                    let result_cookies = match &request.response.cookies {
                        None => "",
                        Some(cookies) => cookies
                    };

                    let cookies_paragraph = Paragraph::new(result_cookies)
                        .fg(THEME.read().ui.font_color)
                        .scroll((
                            self.result_vertical_scrollbar.scroll,
                            self.result_horizontal_scrollbar.scroll
                        ));

                    frame.render_widget(cookies_paragraph, request_result_layout[2]);
                },
                RequestResultTabs::Headers => {
                    let result_headers: Vec<Line> = request.response.headers
                        .par_iter()
                        .map(
                            |(header, value)| 
                                Line::from(vec![
                                    Span::raw(header).bold().fg(THEME.read().ui.secondary_foreground_color),
                                    Span::raw(": ").fg(THEME.read().ui.secondary_foreground_color),
                                    Span::raw(value).fg(THEME.read().ui.font_color)
                                ])
                        )
                        .collect();

                    let headers_paragraph = Paragraph::new(result_headers)
                        .scroll((
                            self.result_vertical_scrollbar.scroll,
                            self.result_horizontal_scrollbar.scroll
                        ));

                    frame.render_widget(headers_paragraph, request_result_layout[2]);
                },
                RequestResultTabs::Console => {
                    let console_paragraph = Paragraph::new(self.syntax_highlighting.highlighted_console_output.clone())
                        .scroll((
                            self.result_vertical_scrollbar.scroll,
                            self.result_horizontal_scrollbar.scroll
                        ));

                    frame.render_widget(console_paragraph, request_result_layout[2]);
                }
            };
        }

        let result_vertical_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::new().fg(THEME.read().ui.font_color));
        let result_horizontal_scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
            .style(Style::new().fg(THEME.read().ui.font_color))
            .thumb_symbol("■"); // Better than the default full block

        frame.render_stateful_widget(
            result_vertical_scrollbar,
            rect.inner(Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.result_vertical_scrollbar.state
        );

        if !(self.config.should_wrap_body() && self.request_result_tab == RequestResultTabs::Body) {
            frame.render_stateful_widget(
                result_horizontal_scrollbar,
                rect.inner(Margin {
                    // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                    vertical: 0,
                    horizontal: 1,
                }),
                &mut self.result_horizontal_scrollbar.state
            );
        }

        self.last_messages_area_size.0 = request_result_layout[2].width.saturating_sub(1);
        self.last_messages_area_size.1 = request_result_layout[2].height.saturating_sub(1);
    }
}
