use std::fs::{File, OpenOptions};
use clap_verbosity_flag::log::LevelFilter;
use tracing::{trace};
use tracing_log::{AsTrace};
use tracing_subscriber::layer::{SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;
use crate::app::app::App;
use crate::app::log::LogCounterLayer;
use crate::cli::args::{ARGS, Command};
use crate::panic_error;
use crate::app::startup::startup::AppMode::{CLI, TUI};
use crate::models::collection::CollectionFileFormat;

pub enum AppMode {
    TUI,
    CLI(Command),
}

impl<'a> App<'a> {
    /// Method called before running the app, returns the app if the TUI should be started
    pub fn startup(&mut self) -> AppMode {
        // Logging is initialized before anything else
        match ARGS.command.is_some() {
            // CLI
            true => tracing_subscriber::fmt()
                .pretty()
                .with_max_level(ARGS.verbosity.log_level_filter().as_trace())
                .with_file(false)
                .with_line_number(false)
                .with_ansi(ARGS.ansi_log)
                .init(),
            // TUI
            false => {
                let verbosity = match ARGS.verbosity.log_level_filter() {
                    LevelFilter::Error => LevelFilter::Debug, // Ensure that at least the debug level is always active
                    level => level
                };
                
                // Using a separate file allows to redirect the output and avoid printing to screen
                let log_file = self.create_log_file();

                tracing_subscriber::fmt()
                    .with_max_level(verbosity.as_trace())
                    .with_writer(log_file)
                    .with_file(false)
                    .with_line_number(false)
                    .with_ansi(ARGS.ansi_log)
                    .finish()
                    .with(LogCounterLayer)
                    .init();
            }
        };

        if ARGS.should_parse_directory {
            self.parse_app_directory();
        }

        if let Some(command) = &ARGS.command {
            CLI(command.clone())
        }
        else {
            self.parse_key_bindings_file();
            self.parse_theme_file();
            
            TUI
        }
    }

    fn parse_app_directory(&mut self) {
        let paths = match ARGS.directory.as_ref().unwrap().read_dir() {
            Ok(paths) => paths,
            Err(e) => panic_error(format!("Directory \"{}\" not found\n\t{e}", ARGS.directory.as_ref().unwrap().display()))
        };

        for path in paths {
            let path = path.unwrap().path();

            if path.is_dir() {
                continue;
            }

            let file_name = path.file_name().unwrap().to_str().unwrap();

            trace!("Checking file \"{}\"", path.display());

            if file_name.ends_with(".json") {
                self.set_collections_from_file(path, CollectionFileFormat::Json);
            }
            else if file_name.ends_with(".yaml") {
                self.set_collections_from_file(path, CollectionFileFormat::Yaml);
            }
            else if file_name.starts_with(".env.") {
                self.add_environment_from_file(path)
            }
            else if file_name == "atac.toml" {
                self.parse_config_file(path);
            }
            else if file_name == "atac.log" {
                trace!("Log file is not parsable")
            }
        }
    }

    fn create_log_file(&mut self) -> File {
        let path = ARGS.directory.as_ref().unwrap().join("atac.log");

        let log_file = match OpenOptions::new().write(true).create(true).truncate(true).open(path) {
            Ok(log_file) => log_file,
            Err(e) => panic_error(format!("Could not open log file\n\t{e}"))
        };

        return log_file;
    }
}