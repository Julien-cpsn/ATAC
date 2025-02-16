use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::prelude::Style;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Tabs};
use ratatui_image::{Image, Resize};
use ratatui_image::picker::Picker;
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use throbber_widgets_tui::{BRAILLE_DOUBLE, Throbber, WhichUse};
use rayon::prelude::*;

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::models::request::Request;
use crate::models::response::ResponseContent;
use crate::tui::utils::centered_rect::centered_rect;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum RequestResultTabs {
    #[default]
    #[strum(to_string = "Result body")]
    Body,
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

        let result_tabs = RequestResultTabs::iter()
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
                    RequestResultTabs::Cookies | RequestResultTabs::Headers => Some(tab.to_string()),
                    RequestResultTabs::Console => {
                        let local_console_output = self.script_console.console_output.read();

                        match local_console_output.as_ref() {
                            None => None,
                            Some(_) => Some(tab.to_string())
                        }
                    }
                };

                match text {
                    Some(text) => Some(text.fg(THEME.read().ui.font_color)),
                    None => None
                }
            });

        let selected_result_tab_index = self.request_result_tab as usize;

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
                            let last_highlighted = self.syntax_highlighting.highlighted_body.read();

                            if !self.config.is_syntax_highlighting_disabled() && last_highlighted.is_some() {
                                lines = last_highlighted.clone().unwrap();
                            }
                            else {
                                lines = body.lines().par_bridge().map(|line| Line::raw(line)).collect();
                            }

                            let body_paragraph = Paragraph::new(lines)
                                .scroll((
                                    self.result_vertical_scrollbar.scroll,
                                    self.result_horizontal_scrollbar.scroll
                                ));

                            frame.render_widget(body_paragraph, request_result_layout[2]);
                        }
                        ResponseContent::Image(image_response) => match &image_response.image {
                            _ if self.config.is_image_preview_disabled() => {
                                let image_disabled_paragraph = Paragraph::new("\nImage preview disabled").centered();
                                frame.render_widget(image_disabled_paragraph, request_result_layout[2]);
                            },
                            Some(image) => {
                                let picker = Picker::from_fontsize((3, 6));

                                let mut image_static = picker
                                    .new_protocol(image.clone(), request_result_layout[2], Resize::Fit(None))
                                    .unwrap();

                                let image = Image::new(&mut image_static);
                                frame.render_widget(image, request_result_layout[2]);
                            }
                            None => {
                                let image_error_paragraph = Paragraph::new("\nCould not decode image")
                                    .centered()
                                    .fg(THEME.read().ui.font_color);
                                frame.render_widget(image_error_paragraph, request_result_layout[2]);
                            }
                        },
                    }
                }
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
                }
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
                    let highlighted_console_output = self.syntax_highlighting.highlighted_console_output.read().clone();
                    
                    let console_paragraph = Paragraph::new(highlighted_console_output)
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
}
