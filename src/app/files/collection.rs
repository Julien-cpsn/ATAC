use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use crate::app::app::App;
use crate::app::startup::args::ARGS;
use crate::request::collection::Collection;

impl App<'_> {

    /// Set the app request to the requests found in the collection file
    pub fn set_collections_from_file(&mut self) {
        println!("Parsing collection file...");

        let mut file_content = String::new();

        let mut collection_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&ARGS.collection_file)
            .expect(&format!("Could not open collection file \"{}\"", ARGS.collection_file));

        collection_file.read_to_string(&mut file_content).expect(&format!("Could not read collection file \"{}\"", ARGS.collection_file));

        let collections: Vec<Collection>;

        if file_content.len() == 0 {
            collections = vec![];
        }
        else {
            collections = serde_json::from_str(&file_content).expect("Could not parse collection");
        }

        self.collections = collections;

        println!("Collection file parsed!");
    }

    /// Save app collection in the collection file through a temporary file
    pub fn save_collections_to_file(&mut self) {
        let temps_file_path = format!("{}_", ARGS.collection_file);

        let mut temp_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temps_file_path)
            .expect("Could not open temp file");

        let collections_json = serde_json::to_string_pretty(&self.collections).expect("Could not serialize collection");

        temp_file.write_all(collections_json.as_bytes()).expect("Could not write to temp file");
        temp_file.flush().unwrap();

        fs::rename(temps_file_path, &ARGS.collection_file).expect("Could not move temp file to collection file");
    }
}