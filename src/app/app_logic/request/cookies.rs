use std::sync::Arc;
use cookie_store::{Cookie, CookieResult};
use reqwest::Url;
use crate::app::app::App;
use crate::app::app_states::AppState;
use crate::request::request::KeyValue;

impl App<'_> {
    pub fn update_cookies_table_selection(&mut self) {
        match self.cookies_popup.cookies_table.rows.is_empty() {
            false => {
                self.cookies_popup.cookies_table.selection = Some((0, 0));
                self.cookies_popup.cookies_table.left_state.select(Some(0));
                self.cookies_popup.cookies_table.right_state.select(Some(0));
            },
            true => {
                self.cookies_popup.cookies_table.selection = None;
                self.cookies_popup.cookies_table.left_state.select(None);
                self.cookies_popup.cookies_table.right_state.select(None);
            }
        }
    }

    pub fn create_new_cookie(&mut self) {
        self.cookies_popup.cookies_table.rows.push(
            KeyValue {
                enabled: true,
                data: (String::from("Cookie"), String::from("Value")),
            }
        );

        self.update_cookies_table_selection();
    }

    pub fn delete_cookie(&mut self) {
        if self.cookies_popup.cookies_table.rows.is_empty() || self.cookies_popup.cookies_table.selection.is_none() {
            return;
        }

        let selection = self.cookies_popup.cookies_table.selection.unwrap();
        self.cookies_popup.cookies_table.rows.remove(selection.0);

        self.update_cookies_table_selection();
    }

    pub fn toggle_cookie(&mut self) {
        if self.cookies_popup.cookies_table.rows.is_empty() || self.cookies_popup.cookies_table.selection.is_none() {
            return;
        }

        let selection = self.cookies_popup.cookies_table.selection.unwrap();
        self.cookies_popup.cookies_table.rows[selection.0].enabled = !self.cookies_popup.cookies_table.rows[selection.0].enabled;
    }

    pub fn modify_cookie(&mut self) {
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

        self.state = AppState::DisplayingCookies;
    }
}