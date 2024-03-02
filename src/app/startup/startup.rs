use std::fs;
use std::fs::OpenOptions;
use crate::app::app::App;
use crate::app::startup::args::{ARGS, Command};

impl App<'_> {
    /// Method called before running the app
    pub fn startup(&mut self) -> &mut Self {
        self.parse_app_directory();

        if let Some(command) = &ARGS.command {
            match command {
                Command::Import(import_args) => {
                    self.import_postman_collection(&import_args.path, import_args.max_depth.unwrap_or(99));
                }
            }
        }
        
        self
    }

    pub fn parse_app_directory(&mut self) {
        // Create the app directory if it does not exist
        fs::create_dir_all(&ARGS.directory).expect(&format!("Could not create directory \"{}\"", ARGS.directory.display()));
        
        let paths = ARGS.directory.read_dir().expect(&format!("Directory \"{}\" not found", ARGS.directory.display()));

        let mut was_config_file_parsed = false;

        for path in paths {
            let path = path.unwrap().path();

            if path.is_dir() {
                continue;
            }

            let file_name = path.file_name().unwrap().to_str().unwrap();

            println!("Parsing: {}", path.display());

            if file_name.ends_with(".json") {
                self.set_collections_from_file(path);
            }
            else if file_name.starts_with(".env.") {
                self.add_environment_from_file(path)
            }
            else if file_name == "atac.toml" {
                self.parse_config_file(path);
                was_config_file_parsed = true;
            }
            else if file_name == "atac.log" {
                println!("Nothing to parse here")
            }
            else {
                panic!("unhandled file type");
            }

            println!();
        }

        self.log_file = Some(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(ARGS.directory.join("atac.log"))
                .expect("Could not open log file")
        );
        
        if !was_config_file_parsed {
            self.parse_config_file(ARGS.directory.join("atac.toml"));
        }
    }
}
