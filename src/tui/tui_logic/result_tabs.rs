use std::str::Lines;
use ratatui::prelude::{Line, Stylize};
use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::models::body::find_file_format_in_content_type;
use crate::tui::ui::result_tabs::RequestResultTabs;
use crate::models::response::ResponseContent;
use crate::tui::utils::syntax_highlighting::highlight;

impl App<'_> {
    pub fn tui_next_request_result_tab(&mut self) {
        self.request_result_tab = match self.request_result_tab {
            RequestResultTabs::Body => RequestResultTabs::Cookies,
            RequestResultTabs::Cookies => RequestResultTabs::Headers,
            RequestResultTabs::Headers => {
                let local_selected_request = self.get_selected_request_as_local();
                let selected_request = local_selected_request.read();

                match (&selected_request.console_output.pre_request_output, &selected_request.console_output.post_request_output) {
                    (None, None) => RequestResultTabs::Body,
                    (_, _) => RequestResultTabs::Console
                }
            }
            RequestResultTabs::Console => RequestResultTabs::Body
        };

        *self.should_refresh_scrollbars_and_highlight_response.lock() = true;
    }

    pub fn tui_update_request_result_tab(&mut self) {
        if self.request_result_tab == RequestResultTabs::Console {
            let local_selected_request = self.get_selected_request_as_local();
            let selected_request = local_selected_request.read();

            if selected_request.console_output.pre_request_output.is_none() && selected_request.console_output.post_request_output.is_none() {
                self.request_result_tab = RequestResultTabs::Body;
            }
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
        let lines_count: usize;
        let horizontal_max: usize;

        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        match self.request_result_tab {
            RequestResultTabs::Body => {
                match &selected_request.response.content {
                    None => {
                        lines_count = 0;
                        horizontal_max = 0;
                    },
                    Some(content) => match content {
                        ResponseContent::Body(body) => {
                            lines_count = body.lines().count();
                            horizontal_max = App::get_max_str_len(body.lines());
                        }
                        ResponseContent::Image(_) => {
                            lines_count = 0;
                            horizontal_max = 0;
                        }
                    }
                }
            }
            RequestResultTabs::Cookies => {
                match &selected_request.response.cookies {
                    None => {
                        lines_count = 0;
                        horizontal_max = 0;
                    },
                    Some(cookies) => {
                        lines_count = cookies.lines().count();
                        horizontal_max = App::get_max_str_len(cookies.lines());
                    }
                }
            }
            RequestResultTabs::Headers => {
                lines_count = selected_request.response.headers.len();

                let mut max_tmp = 0;

                for (header, value) in &selected_request.response.headers {
                    let str_len = header.len() + value.len();
                    if str_len > max_tmp {
                        max_tmp = str_len;
                    }
                }
                
                horizontal_max = max_tmp;
            }
            RequestResultTabs::Console => {
                let console_output = match (&selected_request.console_output.pre_request_output, &selected_request.console_output.post_request_output) {
                    (None, None) => None,
                    (Some(pre_request_output), None) => Some(pre_request_output),
                    (None, Some(post_request_output)) => Some(post_request_output),
                    (Some(pre_request_output), Some(post_request_output)) => Some(&format!("{}\n{}", pre_request_output, post_request_output)),
                };

                match console_output {
                    None => {
                        lines_count = 0;
                        horizontal_max = 0;
                    },
                    Some(console_output) => {
                        lines_count = console_output.lines().count();
                        horizontal_max = App::get_max_str_len(console_output.lines());
                    }
                }
            }
        }

        self.result_vertical_scrollbar.set_scroll(lines_count);
        self.result_horizontal_scrollbar.set_scroll(horizontal_max);
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