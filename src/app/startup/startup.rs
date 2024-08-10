use std::fs::OpenOptions;

use crate::app::app::App;
use crate::app::startup::args::{Command, ImportType, ARGS};
use crate::panic_error;
use crate::request::collection::CollectionFileFormat;

impl App<'_> {
    /// Method called before running the app
    pub fn startup(&mut self) -> &mut Self {
        self.parse_key_bindings_file();
        self.parse_app_directory();

        // Creates the log file only if the app is allowed to save files
        if ARGS.should_save {
            self.create_log_file();
        }

        if let Some(command) = &ARGS.command {
            match command {
                Command::Import { import_type } => match import_type {
                    ImportType::Postman {
                        import_path,
                        max_depth,
                    } => self.import_postman_collection(&import_path, max_depth.unwrap_or(99)),

                    ImportType::Curl {
                        import_path,
                        collection_name,
                        request_name,
                        recursive,
                        max_depth,
                    } => self.import_curl_file(
                        &import_path,
                        collection_name,
                        request_name,
                        recursive,
                        max_depth.unwrap_or(99),
                    ),
                },
            }
        }

        self
    }

    fn parse_app_directory(&mut self) {
        let paths = match ARGS.directory.read_dir() {
            Ok(paths) => paths,
            Err(e) => panic_error(format!(
                "Directory \"{}\" not found\n\t{e}",
                ARGS.directory.display()
            )),
        };

        for path in paths {
            let path = path.unwrap().path();

            if path.is_dir() {
                continue;
            }

            let file_name = path.file_name().unwrap().to_str().unwrap();

            println!("Checking: {}", path.display());

            if file_name.ends_with("-atac.json") {
                self.set_collections_from_file(path, CollectionFileFormat::Json);
            } else if file_name.ends_with("-atac.yaml") {
                self.set_collections_from_file(path, CollectionFileFormat::Yaml);
            } else if file_name.starts_with(".env.") {
                self.add_environment_from_file(path)
            } else if file_name == "atac.toml" {
                self.parse_config_file(path);
            } else if file_name == "atac.log" {
                println!("Nothing to parse here")
            }

            println!();
        }
    }

    fn create_log_file(&mut self) {
        let path = ARGS.directory.join("atac.log");

        let log_file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
        {
            Ok(log_file) => log_file,
            Err(e) => panic_error(format!("Could not open log file\n\t{e}")),
        };

        self.log_file = Some(log_file);
    }
}
