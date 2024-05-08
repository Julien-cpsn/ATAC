use std::fs::OpenOptions;

use crate::app::app::App;
use crate::cli::args::{ARGS, Command};
use crate::{panic_error, print_if_not_in_command};
use crate::app::startup::startup::AppMode::{CLI, TUI};
use crate::models::collection::CollectionFileFormat;

pub enum AppMode<'a> {
    TUI(&'a mut App<'a>),
    CLI(&'a mut App<'a>, Command),
}

impl<'a> App<'a> {
    /// Method called before running the app, returns the app if the TUI should be started
    pub fn startup(&'a mut self) -> AppMode<'a> {
        self.parse_app_directory();

        // Creates the log file only if the app is allowed to save files
        if ARGS.should_save {
            self.create_log_file();
        }

        if let Some(command) = &ARGS.command {
            return CLI(self, command.clone());
        }
        else {
            self.parse_key_bindings_file();

            return TUI(self);
        }
    }

    fn parse_app_directory(&mut self) {
        let paths = match ARGS.directory.read_dir() {
            Ok(paths) => paths,
            Err(e) => panic_error(format!("Directory \"{}\" not found\n\t{e}", ARGS.directory.display()))
        };

        for path in paths {
            let path = path.unwrap().path();

            if path.is_dir() {
                continue;
            }

            let file_name = path.file_name().unwrap().to_str().unwrap();

            print_if_not_in_command!("Checking: {}", path.display());

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
                print_if_not_in_command!("Nothing to parse here")
            }

            print_if_not_in_command!();
        }
    }

    fn create_log_file(&mut self) {
        let path = ARGS.directory.join("atac.log");

        let log_file = match OpenOptions::new().write(true).create(true).truncate(true).open(path) {
            Ok(log_file) => log_file,
            Err(e) => panic_error(format!("Could not open log file\n\t{e}"))
        };

        self.log_file = Some(log_file);
    }
}
