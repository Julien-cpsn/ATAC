use std::fmt::{Debug, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::Utc;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use tracing::field::{Field, Visit};
use tracing::{Level, Subscriber};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

// Avoids been lock inside the logger widget when moving around
pub static SHOULD_RECORD_LOGS: AtomicBool = AtomicBool::new(true);

lazy_static! {
    pub static ref LOGS: Mutex<Vec<(String, Level, String, String)>>  = Mutex::new(Vec::new());
}

pub struct LogCounterLayer;

impl<S: Subscriber> Layer<S> for LogCounterLayer {
    fn on_event(&self, e: &tracing::Event<'_>, _: Context<'_, S>) {
        if !SHOULD_RECORD_LOGS.load(Ordering::SeqCst) {
            return;
        }
        
        let now = Utc::now().format("%H:%M:%S").to_string();
        let level = e.metadata().level().clone();
        let target = e.metadata().target().to_string();
        let mut message = String::new();
        e.record(&mut StringVisitor(&mut message));

        let mut logs = LOGS.lock();
        logs.push((
            now,
            level,
            target,
            message
        ));
        // Prevents keeping too much logs
        if logs.len() > 1000 {
            logs.pop().unwrap();
        }
    }
}

pub struct StringVisitor<'a>(&'a mut String);

impl<'a> Visit for StringVisitor<'a> {
    fn record_debug(&mut self, _: &Field, value: &dyn Debug) {
        self.0.write_str(&format!("{value:?}")).unwrap();
    }
}