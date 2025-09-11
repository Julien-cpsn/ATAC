use ratatui::widgets::{ScrollbarState};

#[derive(Default)]
pub struct StatefulScrollbar {
    pub scroll: u16,
    pub max_scroll: u16,
    pub state: ScrollbarState,
}

impl StatefulScrollbar {
    pub fn page_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
        self.state.prev();
    }

    pub fn page_down(&mut self) {
        if self.scroll < self.max_scroll {
            self.scroll += 1;
        }
        self.state.next();
    }

    pub fn set_max_scroll(&mut self, max_scroll: u16) {
        self.max_scroll = max_scroll;
        self.state = self.state.content_length(max_scroll as usize);
    }

    #[allow(unused)]
    pub fn top(&mut self) {
        self.scroll = 0;
        self.state.first()
    }

    #[allow(unused)]
    pub fn bottom(&mut self) {
        self.scroll = self.max_scroll;
        self.state.last();
    }
}