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

    pub fn set_scroll(&mut self, lines: usize) {
        if lines > 0 {
            self.max_scroll = lines as u16 - 1;
        }
        else {
            self.max_scroll = 0;
        }

        self.scroll = 0;
        self.state.first();

        self.state = self.state.content_length(lines);
    }
}