use std::fs;
use std::path::PathBuf;
use crate::app::app::App;
use crate::app::startup::args::ARGS;

impl App<'_> {
    /// Method called before running the app
    pub fn startup(&mut self) -> &mut Self {
        self.parse_app_directory();

        self
    }

    pub fn parse_app_directory(&mut self) {
        let paths = fs::read_dir(&ARGS.directory).expect(&format!("Directory \"{}\" not found", ARGS.directory));

        let mut was_config_file_parsed = false;

        for path in paths {
            let path = path.unwrap().path();

            if path.is_dir() {
                continue;
            }

            let path_str = path.file_name().unwrap().to_str().unwrap();

            println!("Parsing: {}", path.display());

            if path_str.ends_with(".json") {
                self.set_collections_from_file(path);
            }
            else if path_str.starts_with(".env.") {
                println!("\tenv file are not supported yet");
            }
            else if path_str == "atac.toml" {
                self.parse_config_file(path);
                was_config_file_parsed = true;
            }
            else if path_str == "atac.log" {
                println!("Log file")
            }
            else {
                panic!("unhandled file type");
            }

            if !was_config_file_parsed {
                self.parse_config_file(PathBuf::from(&ARGS.directory).join("atac.toml"));
            }

            println!();
        }
    }
}
