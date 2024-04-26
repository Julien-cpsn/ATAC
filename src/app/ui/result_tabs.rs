use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::prelude::Style;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Tabs};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use throbber_widgets_tui::{BRAILLE_DOUBLE, Throbber, WhichUse};

use crate::app::app::App;
use crate::request::request::Request;
use crate::utils::centered_rect::centered_rect;

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
                match tab {
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
                        let local_console_output = self.script_console.console_output.read().unwrap();

                        match local_console_output.as_ref() {
                            None => None,
                            Some(_) => Some(tab.to_string())
                        }
                    }
                }
            });
        
        let selected_result_tab_index = self.request_result_tab as usize;

        let result_tabs = Tabs::new(result_tabs)
            .highlight_style(Style::default().yellow())
            .select(selected_result_tab_index)
            .block(
                Block::new().borders(Borders::BOTTOM)
            );

        frame.render_widget(result_tabs, request_result_layout[0]);

        // If the selected request is currently pending
        if request.is_pending {
            let area = centered_rect(9, 1, request_result_layout[2]);

            self.result_throbber_state.calc_next();
            
            let throbber = Throbber::default()
                .label("Pending")
                .style(Style::new().dark_gray())
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

            let status_code_paragraph = Paragraph::new(status_code).centered().dark_gray();
            frame.render_widget(status_code_paragraph, request_result_layout[1]);


            // REQUEST RESULT CONTENT

            let mut result_widget: Paragraph = match self.request_result_tab {
                RequestResultTabs::Body => match &request.response.body {
                    None => Paragraph::new(""),
                    Some(body) => {
                        let lines: Vec<Line>;
                        let last_highlighted = self.syntax_highlighting.highlighted_body.read().unwrap();

                        if !self.config.disable_syntax_highlighting.unwrap_or(false) && last_highlighted.is_some() {
                            lines = last_highlighted.clone().unwrap();
                        }
                        else {
                            lines = body.lines().map(|line| Line::raw(line)).collect();
                        }

                        Paragraph::new(lines)
                    }
                }
                RequestResultTabs::Cookies => {
                    let result_cookies = match &request.response.cookies {
                        None => "",
                        Some(cookies) => cookies
                    };

                    Paragraph::new(result_cookies)
                }
                RequestResultTabs::Headers => {
                    let result_headers: Vec<Line> = request.response.headers
                        .iter()
                        .map(|(header, value)| Line::from(format!("{header}: {value}")))
                        .collect();

                    Paragraph::new(result_headers)
                }
                RequestResultTabs::Console => {
                    let highlighted_console_output = self.syntax_highlighting.highlighted_console_output.read().unwrap().clone();
                    Paragraph::new(highlighted_console_output)
                }
            };

            result_widget = result_widget.scroll((
                self.result_vertical_scrollbar.scroll,
                self.result_horizontal_scrollbar.scroll
            ));

            frame.render_widget(result_widget, request_result_layout[2]);
        }

        let result_vertical_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        let result_horizontal_scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
            .thumb_symbol("■"); // Better than the default full block

        frame.render_stateful_widget(
            result_vertical_scrollbar,
            rect.inner(&Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.result_vertical_scrollbar.state
        );

        frame.render_stateful_widget(
            result_horizontal_scrollbar,
            rect.inner(&Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 0,
                horizontal: 1,
            }),
            &mut self.result_horizontal_scrollbar.state
        );
    }
}
