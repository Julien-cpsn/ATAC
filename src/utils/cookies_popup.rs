use std::sync::Arc;
use reqwest_cookie_store::CookieStoreRwLock;
use crate::utils::stateful_custom_table::StatefulCustomTable;

#[derive(Default)]
pub struct CookiesPopup {
    pub cookies_table: StatefulCustomTable,
    pub cookie_store: Arc<CookieStoreRwLock>
}