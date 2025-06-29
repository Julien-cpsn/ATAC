use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::str::from_utf8;
use std::sync::Arc;

use indexmap::IndexMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use snailquote::unescape;
use tracing::{info, trace, warn};
use rayon::prelude::*;

use crate::app::app::App;
use crate::cli::args::ARGS;
use crate::panic_error;
use crate::models::environment::Environment;

lazy_static! {
    pub static ref OS_ENV_VARS: IndexMap<String, String> = {
        env::vars()
            .collect()
    };
}

impl App<'_> {
    /// Add the environment file to the app environments
    pub fn add_environment_from_file(&mut self, path_buf: PathBuf) {
        let file_name = path_buf.file_name().unwrap().to_str().unwrap().to_string().replace(".env.", "");

        trace!("Trying to open \"{}\" env file", path_buf.display());

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

        trace!("Environment file parsed!");
    }

    pub fn save_environment_to_file(&mut self, env_index: usize) {
        let environment = self.environments[env_index].read();

        save_environment_to_file(&environment);
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

        vline.par_iter().position_first(|&x| x == b'=').and_then(|pos| {
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
        warn!("Dry-run, not saving the environment");
        return;
    }

    info!("Saving environment \"{}\"", environment.name);

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
        .par_bridge()
        .map(|(key, value)| format!("{key}={value}\n"))
        .collect();
    
    // Remove trailing \n
    data.pop();

    temp_file.write_all(data.as_bytes()).expect("Could not write to temp file");
    temp_file.flush().unwrap();

    fs::rename(temp_file_path, &environment.path).expect("Could not move temp file to environment file");

    trace!("Environment saved")
}
