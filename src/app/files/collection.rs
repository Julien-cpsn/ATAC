use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use crate::app::app::App;
use crate::app::startup::args::ARGS;
use crate::request::collection::Collection;

impl App<'_> {
    /// Set the app request to the requests found in the collection file
    pub fn set_collections_from_file(&mut self, path_buf: PathBuf) {
        let mut file_content = String::new();

        let mut collection_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path_buf.clone())
            .expect("\tCould not open collection file");

        collection_file.read_to_string(&mut file_content).expect("\tCould not read collection file");


        if file_content.len() == 0 {
            let collection = Collection {
                name: path_buf.file_stem().unwrap().to_str().unwrap().to_string(),
                requests: vec![],
                path: path_buf,
            };

            let collection_json = serde_json::to_string_pretty(&collection).expect("Could not serialize collection");

            collection_file.write_all(collection_json.as_bytes()).expect("Could not write to collection file")
        }
        else {
            let mut collection: Collection = serde_json::from_str(&file_content).expect("\tCould not parse collection");

            collection.path = path_buf;

            self.collections.push(collection);
        }

        println!("Collection file parsed!");
    }

    /// Save app collection in the collection file through a temporary file
    pub fn save_collection_to_file(&mut self, collection_index: usize) {
        if ARGS.dry_run {
            return;
        }

        let collection = &self.collections[collection_index];

        let temp_file_name = format!("{}_", collection.path.file_name().unwrap().to_str().unwrap());

        let temp_file_path = collection.path.with_file_name(temp_file_name);

        let mut temp_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_file_path)
            .expect("Could not open temp file");

        let collection_json = serde_json::to_string_pretty(collection).expect("Could not serialize collection");

        temp_file.write_all(collection_json.as_bytes()).expect("Could not write to temp file");
        temp_file.flush().unwrap();

        fs::rename(temp_file_path, &collection.path).expect("Could not move temp file to collection file");
    }

    /// Delete collection file
    pub fn delete_collection_file(&mut self, collection: Collection) {
        if ARGS.dry_run {
            return;
        }

        fs::remove_file(&collection.path).expect("Could not delete collection file");
    }
}