use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::str::from_utf8;
use std::sync::Arc;

use indexmap::IndexMap;
use parking_lot::RwLock;
use snailquote::unescape;

use crate::app::app::App;
use crate::cli::args::ARGS;
use crate::{panic_error, print_if_not_in_command};
use crate::models::environment::Environment;

impl App<'_> {
    /// Add the environment file to the app environments
    pub fn add_environment_from_file(&mut self, path_buf: PathBuf) {
        let file_name = path_buf.file_name().unwrap().to_str().unwrap().to_string().replace(".env.", "");

        let env_file: File = match File::open(path_buf.clone()) {
            Ok(env_file) => env_file,
            Err(e) => panic_error(format!("Could not open environment file\n\t{e}"))
        };

        let environment = Environment {
            name: file_name,
            values: read_environment_from_file(env_file),
            path: path_buf,
        };
        
        self.environments.push(Arc::new(RwLock::new(environment)));

        print_if_not_in_command!("environment file parsed!");
    }
}

fn read_environment_from_file(file: File) -> IndexMap<String, String> {
    let reader = BufReader::new(file);
    let mut environment_values = IndexMap::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some((key, value)) = parse_line(line.trim().as_bytes()) {
                environment_values.insert(key, value);
            }
        }
    }

    environment_values
}

// Code from the EnvFile crate
fn parse_line(entry: &[u8]) -> Option<(String, String)> {
    from_utf8(entry).ok().and_then(|l| {
        let line = l.trim();

        // Ignore comment line
        if line.starts_with('#') {
            return None;
        }

        let vline = line.as_bytes();

        vline.iter().position(|&x| x == b'=').and_then(|pos| {
            from_utf8(&vline[..pos]).ok().and_then(|x| {
                from_utf8(&vline[pos+1..]).ok().and_then(|right| {
                    // The right hand side value can be a quoted string
                    unescape(right).ok().map(|y| (x.to_owned(), y))
                })
            })
        })
    })
}

/// Save app environment in a file through a temporary file
pub fn save_environment_to_file(environment: &Environment) {
    if !ARGS.should_save {
        return;
    }

    let temp_file_name = format!("{}_", environment.path.file_name().unwrap().to_str().unwrap());

    let temp_file_path = environment.path.with_file_name(temp_file_name);

    let mut temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temp_file_path)
        .expect("Could not open temp file");

    let mut data: String = environment.values
        .iter()
        .map(|(key, value)| format!("{key}={value}\n"))
        .collect();
    
    data.pop();

    temp_file.write_all(data.as_bytes()).expect("Could not write to temp file");
    temp_file.flush().unwrap();

    fs::rename(temp_file_path, &environment.path).expect("Could not move temp file to environment file");
}