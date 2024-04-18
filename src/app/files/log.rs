use std::io::Write;
use crate::app::app::App;

impl App<'_> {
    pub fn write_to_log_file(&mut self, modifier: String, key: String, app_state: String) {
        if let Some(log_file) = &mut self.log_file {
            log_file
                .write_fmt(format_args!(
                    "{:25}{:25}{:40}\n",
                    modifier,
                    key,
                    app_state,
                ))
                .expect("Could not write to log file")
        }
    }
}