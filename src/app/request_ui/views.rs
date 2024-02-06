
#[derive(Copy, Clone)]
pub enum RequestView {
    Normal,
    OnlyResult,
    OnlyParams,
}

pub fn next_request_view(current_view: RequestView) -> RequestView {
    match current_view {
        RequestView::Normal => RequestView::OnlyResult,
        RequestView::OnlyResult => RequestView::OnlyParams,
        RequestView::OnlyParams => RequestView::Normal
    }
}