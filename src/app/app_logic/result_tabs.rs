use std::str::Lines;
use crate::app::app::App;
use crate::app::ui::result_tabs::RequestResultTabs;

impl App<'_> {
    pub fn next_request_result_tab(&mut self) {
        self.request_result_tab = match self.request_result_tab {
            RequestResultTabs::Body => RequestResultTabs::Cookies,
            RequestResultTabs::Cookies => RequestResultTabs::Headers,
            RequestResultTabs::Headers => {
                let local_console_output = self.script_console.console_output.read().unwrap();

                match local_console_output.as_ref() {
                    None => RequestResultTabs::Body,
                    Some(_) => RequestResultTabs::Console
                }
            }
            RequestResultTabs::Console => RequestResultTabs::Body
        };

        self.refresh_result_scrollbars();
    }

    pub fn refresh_result_scrollbars(&mut self) {
        // Vertical max
        let lines_count: usize;
        let horizontal_max: usize;

        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read().unwrap();

        match self.request_result_tab {
            RequestResultTabs::Body => {
                match &selected_request.response.body {
                    None => {
                        lines_count = 0;
                        horizontal_max = 0;
                    },
                    Some(body) => {
                        lines_count = body.lines().count();
                        horizontal_max = App::get_max_str_len(body.lines());
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
                let local_console_output = self.script_console.console_output.read().unwrap();

                match local_console_output.as_ref() {
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