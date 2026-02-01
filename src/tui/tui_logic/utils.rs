use std::sync::Arc;
use parking_lot::RwLock;
use ratatui::prelude::Stylize;
use ratatui::widgets::ListItem;
use crate::app::files::theme::THEME;
use crate::models::environment::Environment;
use crate::models::request::KeyValue;
use crate::tui::tui_logic::environment::tui_add_color_to_env_keys;

pub fn key_value_vec_to_items_list<'a>(local_env: &Option<Arc<RwLock<Environment>>>, rows: &Vec<KeyValue>) -> (Vec<ListItem<'a>>, Vec<ListItem<'a>>) {
    let mut keys: Vec<ListItem> = vec![];
    let mut values: Vec<ListItem> = vec![];

    for row in rows.iter() {
        let key = tui_add_color_to_env_keys(local_env, row.data.0.to_owned());
        let value = tui_add_color_to_env_keys(local_env, row.data.1.to_owned());

        let mut key = ListItem::from(key);
        let mut value = ListItem::from(value);

        if !row.enabled {
            key = key.fg(THEME.read().ui.secondary_foreground_color).dim();
            value = value.fg(THEME.read().ui.secondary_foreground_color).dim();
        }

        keys.push(key);
        values.push(value);
    }

    (keys, values)
}