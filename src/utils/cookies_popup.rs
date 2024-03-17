use crate::request::request::KeyValue;
use crate::utils::stateful_custom_table::StatefulCustomTable;

#[derive(Default)]
pub struct CookiesPopup {
    pub cookies: Vec<KeyValue>,
    pub cookies_table: StatefulCustomTable,
}