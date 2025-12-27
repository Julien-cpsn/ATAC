use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use anyhow::anyhow;

use parking_lot::RwLock;
use regex::Regex;
use reqwest::header::CONTENT_TYPE;
use reqwest::Url;
use thiserror::Error;
use walkdir::WalkDir;
use rayon::prelude::*;

use crate::app::app::App;
use crate::cli::args::ARGS;
use crate::cli::cli_logic::import::curl::ImportCurlError::{CouldNotParseCurl, CouldNotParseUrl, CouldNotReadFile, UnknownMethod};
use crate::cli::commands::import::CurlImport;
use crate::models::auth::auth::Auth;
use crate::models::auth::basic::BasicAuth;
use crate::models::auth::bearer_token::BearerToken;
use crate::models::protocol::http::body::ContentType;
use crate::models::protocol::http::body::ContentType::NoBody;
use crate::models::collection::Collection;
use crate::models::protocol::http::http::HttpRequest;
use crate::models::protocol::http::method::Method;
use crate::models::protocol::protocol::Protocol;
use crate::models::request::{KeyValue, Request};

#[derive(Error, Debug)]
enum ImportCurlError {
    #[error("Could not read cURL file\n\t{0}")]
    CouldNotReadFile(String),
    #[error("Could not parse cURL\n\t{0}")]
    CouldNotParseCurl(String),
    #[error("Could not parse URL\n\t{0}")]
    CouldNotParseUrl(String),
    #[error("Unknown method\n\t{0}")]
    UnknownMethod(String),
}

impl App<'_> {
    pub fn import_curl_file(&mut self, curl_import: &CurlImport) -> anyhow::Result<()> {
        let path_buf = &curl_import.import_path;
        let collection_name = &curl_import.collection_name;
        let request_name = &curl_import.request_name;
        let recursive = &curl_import.recursive;
        let max_depth = curl_import.max_depth.unwrap_or(99);
        
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
                    last_position: Some(self.collections.len() - 1),
                    requests: vec![],
                    path: ARGS.directory.as_ref().unwrap().join(format!("{}.{}", collection_name.clone(), file_format.to_string())),
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
                parse_request(path_buf, request_name)?
            ],
            false => parse_requests_recursively(path_buf, *recursive, max_depth)?,
        };

        // Add the parsed request to the collection
        collection.requests.extend(requests);

        self.save_collection_to_file(collection_index);
        
        Ok(())
    }
}

fn parse_requests_recursively(path: &PathBuf, recursive: bool, max_depth: u16) -> anyhow::Result<Vec<Arc<RwLock<Request>>>> {
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
        let request = parse_request(&entry.path().to_path_buf(), file_name)?;

        requests.push(request);
    }

    return Ok(requests);
}

/// TODO: parse everything with regexes in order to handle everything
fn parse_request(path: &PathBuf, request_name: String) -> anyhow::Result<Arc<RwLock<Request>>> {
    let curl_stringed = match fs::read_to_string(path) {
        Ok(original_curl) => original_curl,
        Err(e) => {
            return Err(anyhow!(CouldNotReadFile(e.to_string())))
        },
    };

    println!("\tRequest name: {}", request_name);

    let parsed_curl = match curl_parser::ParsedRequest::load(&curl_stringed, None::<String>) {
        Ok(parsed_curl) => parsed_curl,
        Err(e) => {
            return Err(anyhow!(CouldNotParseCurl(e.to_string())))
        },
    };

    /* URL */

    // Parse the URL so we can transform it
    let mut curl_url = match Url::parse(&parsed_curl.url.to_string()) {
        Ok(url) => url,
        Err(e) => {
            return Err(anyhow!(CouldNotParseUrl(e.to_string())))
        },
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
        Err(e) => {
            return Err(anyhow!(UnknownMethod(e.to_string())))
        },
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
                let bearer_token = bearer_token.to_str()?[7..].to_string();

                Auth::BearerToken(BearerToken { token: bearer_token })
            }
            else {
                Auth::NoAuth
            }
        }
        Some(capture) => {
            let username = capture["username"].to_string();
            let password = capture["password"].to_string();
            
            Auth::BasicAuth(BasicAuth { username, password })
        }
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
        params,
        headers,
        auth,
        protocol: Protocol::HttpRequest(HttpRequest {
            method,
            body
        }),
        ..Default::default()
    };

    return Ok(Arc::new(RwLock::new(request)));
}