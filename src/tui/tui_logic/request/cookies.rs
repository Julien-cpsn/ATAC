use crate::app::app::App;

impl App<'_> {
    pub fn tui_update_cookies_table_selection(&mut self) {
        match self.cookies_popup.cookies_table.rows.is_empty() {
            false => {
                self.cookies_popup.cookies_table.selection = Some((0, 0));

                for table_state in self.cookies_popup.cookies_table.lists_states.iter_mut() {
                    table_state.select(Some(0));
                }
            },
            true => {
                self.cookies_popup.cookies_table.selection = None;

                for table_state in self.cookies_popup.cookies_table.lists_states.iter_mut() {
                    table_state.select(None);
                }
            }
        }
    }

    pub fn tui_delete_cookie(&mut self) {
        if self.cookies_popup.cookies_table.rows.is_empty() || self.cookies_popup.cookies_table.selection.is_none() {
            return;
        }

        let selection = self.cookies_popup.cookies_table.selection.unwrap();
        let cookie_row = self.cookies_popup.cookies_table.rows.remove(selection.0);
        
        {
            let mut local_cookie_store = self.cookies_popup.cookie_store.write().unwrap();

            local_cookie_store.remove(
                &cookie_row[0],
                &cookie_row[3],
                &cookie_row[1],
            );
        }

        self.tui_update_cookies_table_selection();
    }
}