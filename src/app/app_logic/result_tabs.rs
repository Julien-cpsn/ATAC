use crate::app::app::App;
use crate::app::ui::result_tabs::RequestResultTabs;

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

        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read().unwrap();

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

}