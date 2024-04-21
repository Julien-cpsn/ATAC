use std::str::Lines;
use crate::app::app::App;
use crate::app::ui::result_tabs::RequestResultTabs;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

impl App<'_> {
    pub fn next_request_result_tab(&mut self) {
        self.request_result_tab = match self.request_result_tab {
            RequestResultTabs::Body => RequestResultTabs::Cookies,
            RequestResultTabs::Cookies => RequestResultTabs::Headers,
            RequestResultTabs::Headers => RequestResultTabs::Body,
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
                match &selected_request.result.body {
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
                match &selected_request.result.cookies {
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
                lines_count = selected_request.result.headers.len();

                let mut max_tmp = 0;

                for (header, value) in &selected_request.result.headers {
                    let str_len = header.len() + value.len();
                    if str_len > max_tmp {
                        max_tmp = str_len;
                    }
                }
                
                horizontal_max = max_tmp;
            }
        }

        self.result_vertical_scrollbar.set_scroll(lines_count);
        self.result_horizontal_scrollbar.set_scroll(horizontal_max);
    }
    
    fn get_max_str_len(lines: Lines) -> usize {
        let mut max_tmp = 0;

        for line in lines {
            if line.len() > max_tmp {
                max_tmp = line.len();
            }
        }

        return max_tmp;
    }

    pub fn copy_to_clipboard(&mut self) {
        let mut ctx = ClipboardContext::new().unwrap();
         match self.request_result_tab {
            RequestResultTabs::Body => {
                let body = self.get_selected_request_as_local().read().unwrap().result.body.clone();
                match body {
                    Some(body) => {
                        ctx.set_contents(body).unwrap();
                    },
                    None => {}
                }
            }
            RequestResultTabs::Cookies => {
                let cookies = self.get_selected_request_as_local().read().unwrap().result.cookies.clone();
                match cookies {
                    Some(cookies) => {
                        ctx.set_contents(cookies).unwrap();
                    },
                    None => {}
                }
            }
            RequestResultTabs::Headers => {
                let mut headers = String::new();
                for (header, value) in &self.get_selected_request_as_local().read().unwrap().result.headers {
                    headers.push_str(&format!("{header}: {value}\n"));
                }
                match headers.len() {
                    0 => {},
                    _ => {
                        ctx.set_contents(headers).unwrap();
                    },
                }
            }
        }
    }
}
