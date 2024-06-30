use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use parking_lot::RwLock;
use regex::Regex;
use reqwest::header::CONTENT_TYPE;
use reqwest::Url;
use walkdir::WalkDir;
use rayon::prelude::*;

use crate::app::app::App;
use crate::app::startup::args::ARGS;
use crate::panic_error;
use crate::request::auth::Auth;
use crate::request::body::ContentType;
use crate::request::body::ContentType::NoBody;
use crate::request::collection::Collection;
use crate::request::method::Method;
use crate::request::request::{KeyValue, Request};

impl App<'_> {
    pub fn import_curl_file(&mut self, path_buf: &PathBuf, collection_name: &String, request_name: &Option<String>, recursive: &bool, max_depth: u16) {
        println!("Parsing cURL request");

        println!("Collection name: {}", collection_name);

        let (collection_index, collection) = match self.collections
            .par_iter_mut()
            .enumerate()
            .find_any(|(_, collection)| collection.name == collection_name.as_str()) {
            Some((index, collection)) => (index, collection),
            None => {
                println!("Collection does not exist. Creating it...");

                let file_format = self.config.get_preferred_collection_file_format();

                let collection = Collection {
                    name: collection_name.clone(),
                    requests: vec![],
                    path: ARGS.directory.join(format!("{}.{}", collection_name.clone(), file_format.to_string())),
                    file_format,
                };

                self.collections.push(collection);

                (self.collections.len()-1, self.collections.last_mut().unwrap())
            }
        };

        let request_name = match request_name {
            None => path_buf.file_stem().unwrap().to_str().unwrap().to_string(),
            Some(request_name) => request_name.clone()
        };

        let requests = match path_buf.is_file() {
            true => vec![
                parse_request(path_buf, request_name)
            ],
            false => parse_requests_recursively(path_buf, *recursive, max_depth),
        };

        // Add the parsed request to the collection
        collection.requests.extend(requests);

        self.save_collection_to_file(collection_index);
    }
}

fn parse_requests_recursively(path: &PathBuf, recursive: bool, max_depth: u16) -> Vec<Arc<RwLock<Request>>> {
    let max_depth: usize = match recursive {
        true => max_depth as usize,
        false => 1
    };

    let mut requests: Vec<Arc<RwLock<Request>>> = vec![];
    let walker = WalkDir::new(path)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in walker {
        if !entry.file_type().is_file() {
            continue;
        }

        // Will use the file name as the request name
        let file_name = entry.file_name().to_str().unwrap().to_string();
        let request = parse_request(&entry.path().to_path_buf(), file_name);

        requests.push(request);
    }

    return requests;
}

/// TODO: parse everything with regexes in order to handle everything
fn parse_request(path: &PathBuf, request_name: String) -> Arc<RwLock<Request>> {
    let curl_stringed = match fs::read_to_string(path) {
        Ok(original_curl) => original_curl,
        Err(e) => panic_error(format!("Could not read cURL file\n\t{e}")),
    };

    println!("\tRequest name: {}", request_name);

    let parsed_curl = match curl_parser::ParsedRequest::load(&curl_stringed, None::<String>) {
        Ok(parsed_curl) => parsed_curl,
        Err(e) => panic_error(format!("Could not parse cURL\n\t{e}")),
    };

    /* URL */

    // Parse the URL so we can transform it
    let mut curl_url = match Url::parse(&parsed_curl.url.to_string()) {
        Ok(url) => url,
        Err(e) => panic_error(format!("Could not parse URL\n\t{e}")),
    };

    curl_url.set_query(None);
    let url = curl_url.to_string();

    /* QUERY PARAMS */

    let params = curl_url
        .query_pairs()
        .par_bridge()
        .map(|(k, v)| KeyValue {
            enabled: true,
            data: (k.to_string(), v.to_string()),
        })
        .collect();

    /* METHOD */

    let method = match Method::from_str(parsed_curl.method.as_str()) {
        Ok(method) => method,
        Err(e) => panic_error(format!("Unknown method\n\t{e}")),
    };

    /* HEADERS */

    let headers: Vec<KeyValue> = parsed_curl.headers
        .iter()
        .par_bridge()
        .filter(|(header_name, _)| header_name.as_str() != "authorization") // Exclude Authorization header, as that will be handled by the auth field
        .map(|(k, v)| KeyValue {
            enabled: true,
            data: (k.to_string(), v.to_str().unwrap().to_string()),
        })
        .collect();

    /* AUTH */

    let basic_auth_regex = Regex::new(r#"(-u|--user) ["'](?<username>.*):(?<password>.*)["']"#).unwrap();

    let auth = match basic_auth_regex.captures(&curl_stringed) {
        None => {
            let bearer_token_header = parsed_curl.headers
                .iter()
                .par_bridge()
                .find_any(|(header_name, value)| header_name.as_str() == "authorization" && value.to_str().unwrap().starts_with("Bearer "));

            if let Some((_, bearer_token)) = bearer_token_header {
                let bearer_token = &bearer_token.to_str().unwrap()[7..];

                Auth::BearerToken(bearer_token.to_string())
            }
            else {
                Auth::NoAuth
            }
        }
        Some(capture) => Auth::BasicAuth(capture["username"].to_string(), capture["password"].to_string())
    };

    /* BODY */

    let body;

    // TODO: does not support forms yet
    if !parsed_curl.body.is_empty() {
        let content_type_header = headers.par_iter().find_any(|header| header.data.0 == CONTENT_TYPE.as_str());
        let body_stringed = parsed_curl.body.join("\n");


        if let Some(content_type) = content_type_header {
            body = ContentType::from_content_type(&content_type.data.1, body_stringed);
        }
        else {
            body = NoBody;
        }
    }
    else {
        body = NoBody;
    }

    let request = Request {
        name: request_name,
        url,
        method,
        params,
        headers,
        body,
        auth,
        ..Default::default()
    };

    return Arc::new(RwLock::new(request));
}