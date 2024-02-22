use crate::app::app::App;

#[derive(Copy, Clone)]
pub enum RequestView {
    Normal,
    OnlyResult,
    OnlyParams,
}

impl App<'_> {
    pub fn next_request_view(&mut self) {
        self.request_view = match self.request_view {
            RequestView::Normal => RequestView::OnlyResult,
            RequestView::OnlyResult => RequestView::OnlyParams,
            RequestView::OnlyParams => RequestView::Normal
        };
    }
}