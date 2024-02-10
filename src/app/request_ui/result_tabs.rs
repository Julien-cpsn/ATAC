use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::prelude::Style;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Tabs};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use crate::app::app::App;
use crate::request::request::Request;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum RequestResultTabs {
    #[default]
    #[strum(to_string = "Result body")]
    Body,
    #[strum(to_string = "Cookies")]
    Cookies,
    #[strum(to_string = "Headers")]
    Headers,
}

impl App<'_> {
    pub fn next_request_result_tab(&mut self) {
        self.request_result_tab = match self.request_result_tab {
            RequestResultTabs::Body => RequestResultTabs::Cookies,
            RequestResultTabs::Cookies => RequestResultTabs::Headers,
            RequestResultTabs::Headers => RequestResultTabs::Body,
        };

        self.refresh_result_scrollbar();
    }

    pub fn refresh_result_scrollbar(&mut self) {
        let lines_count: usize;
        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &mut self.collection.items[selected_request_index];

        match self.request_result_tab {
            RequestResultTabs::Body => {
                lines_count = match &selected_request.result.body {
                    None => 0,
                    Some(body) => body.lines().count()
                }
            }
            RequestResultTabs::Cookies => {
                lines_count = match &selected_request.result.cookies {
                    None => 0,
                    Some(cookies) => cookies.lines().count()
                }
            }
            RequestResultTabs::Headers => {
                lines_count = match &selected_request.result.headers {
                    None => 0,
                    Some(headers) => headers.lines().count()
                }
            }
        }

        self.result_scrollbar.set_scroll(lines_count);
    }

    pub fn render_request_result(&mut self, frame: &mut Frame, rect: Rect, request: &Request) {
        let request_result_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(2),
                Constraint::Fill(1)
            ]
        )
            .split(rect);


        // REQUEST RESULT TABS

        let result_tabs = RequestResultTabs::iter()
            .map(|tab| {
                match tab {
                    RequestResultTabs::Body => {
                        if let Some(status_code) = request.result.status_code {
                            format!("{} ({})", tab.to_string(), status_code)
                        }
                        else {
                            format!("{}", tab.to_string())
                        }
                    },
                    RequestResultTabs::Cookies | RequestResultTabs::Headers => tab.to_string()
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

        // REQUEST RESULT CONTENT

        let mut result_widget: Paragraph = match self.request_result_tab {
            RequestResultTabs::Body => {
                let result_body = match &request.result.body {
                    None => "",
                    Some(result) => result
                };

                Paragraph::new(result_body)
            }
            RequestResultTabs::Cookies => {
                let result_cookies = match &request.result.cookies {
                    None => "",
                    Some(cookies) => cookies
                };

                Paragraph::new(result_cookies)
            }
            RequestResultTabs::Headers => {
                let result_headers = match &request.result.headers {
                    None => "",
                    Some(headers) => headers
                };

                Paragraph::new(result_headers)
            }
        };

        result_widget = result_widget.scroll((self.result_scrollbar.scroll, 0));

        frame.render_widget(result_widget, request_result_layout[1]);

        let result_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);

        frame.render_stateful_widget(
            result_scrollbar,
            rect.inner(&Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.result_scrollbar.state
        )
    }
}
