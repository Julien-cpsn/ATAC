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
        let paths = ARGS.directory.read_dir().expect(&format!("Directory \"{}\" not found", ARGS.directory.display()));

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
                self.parse_config_file(ARGS.directory.join("atac.toml"));
            }

            println!();
        }
    }
}
