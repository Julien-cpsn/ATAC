use std::str::Lines;
use ratatui::prelude::{Line, Stylize};
use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::models::protocol::http::body::find_file_format_in_content_type;
use crate::models::protocol::protocol::Protocol;
use crate::tui::ui::result_tabs::RequestResultTabs;
use crate::models::response::ResponseContent;
use crate::tui::utils::syntax_highlighting::highlight;

impl App<'_> {
    pub fn tui_next_request_result_tab(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        self.request_result_tab = match self.request_result_tab {
            RequestResultTabs::Body => RequestResultTabs::Cookies,
            RequestResultTabs::Messages => RequestResultTabs::Cookies,
            RequestResultTabs::Cookies => RequestResultTabs::Headers,
            RequestResultTabs::Headers => {
                let local_selected_request = self.get_selected_request_as_local();
                let selected_request = local_selected_request.read();

                match (&selected_request.console_output.pre_request_output, &selected_request.console_output.post_request_output) {
                    (None, None) => match selected_request.protocol {
                        Protocol::HttpRequest(_) => RequestResultTabs::Body,
                        Protocol::WsRequest(_) => RequestResultTabs::Messages
                    },
                    (_, _) => RequestResultTabs::Console
                }
            },
            RequestResultTabs::Console => match selected_request.protocol {
                Protocol::HttpRequest(_) => RequestResultTabs::Body,
                Protocol::WsRequest(_) => RequestResultTabs::Messages
            }
        };

        *self.received_response.lock() = true;
    }

    pub fn tui_update_request_result_tab(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        if self.request_result_tab == RequestResultTabs::Console && selected_request.console_output.pre_request_output.is_none() && selected_request.console_output.post_request_output.is_none() {
            self.request_result_tab = match selected_request.protocol {
                Protocol::HttpRequest(_) => RequestResultTabs::Body,
                Protocol::WsRequest(_) => RequestResultTabs::Messages
            };
        }
        else {
            match selected_request.protocol {
                Protocol::HttpRequest(_) if self.request_result_tab == RequestResultTabs::Messages => self.request_result_tab = RequestResultTabs::Body,
                Protocol::WsRequest(_) if self.request_result_tab == RequestResultTabs::Body => self.request_result_tab = RequestResultTabs::Messages,
                _ => {}
            };
        }
    }

    pub fn tui_highlight_response_body_and_console(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.write();

        self.syntax_highlighting.highlighted_body = None;
        self.syntax_highlighting.highlighted_console_output = vec![];

        if let Some(file_format) = find_file_format_in_content_type(&selected_request.response.headers) {
            if let Some(ResponseContent::Body(response_content)) = &selected_request.response.content.as_ref() {
                self.syntax_highlighting.highlighted_body = highlight(response_content, &file_format);
            }
        }

        if let Some(pre_request_console_output) = &selected_request.console_output.pre_request_output {
            let mut highlighted_console_output = highlight(pre_request_console_output, "json").unwrap();

            highlighted_console_output.insert(0, Line::default());
            highlighted_console_output.insert(1, Line::raw("----- Pre-request script start -----").fg(THEME.read().ui.secondary_foreground_color).centered());
            highlighted_console_output.push(Line::raw("----- Pre-request script end -----").fg(THEME.read().ui.secondary_foreground_color).centered());

            self.syntax_highlighting.highlighted_console_output.extend(highlighted_console_output);
        }

        if let Some(post_request_console_output) = &selected_request.console_output.post_request_output {
            let mut highlighted_console_output = highlight(post_request_console_output, "json").unwrap();

            highlighted_console_output.insert(0, Line::default());
            highlighted_console_output.insert(1, Line::raw("----- Post-request script start -----").fg(THEME.read().ui.secondary_foreground_color).centered());
            highlighted_console_output.push(Line::raw("----- Post-request script end -----").fg(THEME.read().ui.secondary_foreground_color).centered());

            self.syntax_highlighting.highlighted_console_output.extend(highlighted_console_output);
        }
    }

    pub fn tui_refresh_result_scrollbars(&mut self) {
        // Vertical max
        let vertical_max: u16;
        let horizontal_max: u16;

        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        match self.request_result_tab {
            RequestResultTabs::Body => {
                match &selected_request.response.content {
                    None => {
                        vertical_max = 0;
                        horizontal_max = 0;
                    },
                    Some(content) => match content {
                        ResponseContent::Body(body) => {
                            vertical_max = body.lines().count() as u16;
                            horizontal_max = App::get_max_str_len(body.lines()) as u16;
                        }
                        ResponseContent::Image(_) => {
                            vertical_max = 0;
                            horizontal_max = 0;
                        }
                    }
                }
            },
            RequestResultTabs::Messages => {
                let line_count = self.get_messages_lines_count()  as u16;
                let max_scroll = line_count.saturating_sub(self.last_messages_area_size.1);

                if self.last_messages_area_size.1 > 0 {
                    vertical_max = max_scroll;
                }
                else {
                    vertical_max = 0;
                }

                horizontal_max = 0;
            },
            RequestResultTabs::Cookies => {
                match &selected_request.response.cookies {
                    None => {
                        vertical_max = 0;
                        horizontal_max = 0;
                    },
                    Some(cookies) => {
                        vertical_max = cookies.lines().count() as u16;
                        horizontal_max = App::get_max_str_len(cookies.lines()) as u16;
                    }
                }
            },
            RequestResultTabs::Headers => {
                vertical_max = selected_request.response.headers.len()  as u16;

                let mut max_tmp: u16 = 0;

                for (header, value) in &selected_request.response.headers {
                    let str_len = (header.len() + value.len()) as u16;
                    if str_len > max_tmp {
                        max_tmp = str_len;
                    }
                }

                horizontal_max = max_tmp;
            },
            RequestResultTabs::Console => {
                let console_output = match (&selected_request.console_output.pre_request_output, &selected_request.console_output.post_request_output) {
                    (None, None) => None,
                    (Some(pre_request_output), None) => Some(pre_request_output),
                    (None, Some(post_request_output)) => Some(post_request_output),
                    (Some(pre_request_output), Some(post_request_output)) => Some(&format!("{}\n{}", pre_request_output, post_request_output)),
                };

                match console_output {
                    None => {
                        vertical_max = 0;
                        horizontal_max = 0;
                    },
                    Some(console_output) => {
                        vertical_max = console_output.lines().count() as u16;
                        horizontal_max = App::get_max_str_len(console_output.lines()) as u16;
                    }
                }
            }
        }

        self.result_vertical_scrollbar.set_max_scroll(vertical_max);
        self.result_horizontal_scrollbar.set_max_scroll(horizontal_max);
    }

    pub fn get_max_str_len(lines: Lines) -> usize {
        let mut max_tmp = 0;

        for line in lines {
            if line.len() > max_tmp {
                max_tmp = line.len();
            }
        }

        return max_tmp;
    }
}