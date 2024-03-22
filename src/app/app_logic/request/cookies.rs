use crate::app::app::App;
use crate::app::app_states::AppState;

impl App<'_> {
    pub fn update_cookies_table_selection(&mut self) {
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

    #[allow(dead_code)]
    pub fn create_new_cookie(&mut self) {
        self.cookies_popup.cookies_table.rows.push([
            String::from("https://url.com"),
            String::from("Name"),
            String::from("Value"),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new()
        ]);

        self.update_cookies_table_selection();
    }

    pub fn delete_cookie(&mut self) {
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

        self.update_cookies_table_selection();
    }

    pub fn modify_cookie(&mut self) {
        /*
        let input_text = self.cookies_popup.cookies_table.selection_text_input.text.clone();

        if input_text.trim().is_empty() {
            return;
        }

        let selection = self.cookies_popup.cookies_table.selection.unwrap();

        match selection {
            (x, 0) => self.cookies_popup.cookies_table.rows[x].data.0 = input_text,
            (x, 1) => self.cookies_popup.cookies_table.rows[x].data.1 = input_text,
            _ => {}
        }
        */
        self.state = AppState::DisplayingCookies;
    }
}