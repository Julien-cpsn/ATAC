use std::sync::Arc;

use reqwest_cookie_store::CookieStoreRwLock;
use crate::tui::utils::stateful::cookie_table::StatefulCookieTable;

#[derive(Default)]
pub struct CookiesPopup {
    pub cookies_table: StatefulCookieTable,
    pub cookie_store: Arc<CookieStoreRwLock>
}