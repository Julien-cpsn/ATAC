use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use parser4curls::{parse, Curl};
use reqwest::Url;

use crate::app::app::App;
use crate::app::startup::args::ARGS;
use crate::panic_error;
use crate::request::auth::Auth;
use crate::request::body::ContentType;
use crate::request::collection::Collection;
use crate::request::method::Method;
use crate::request::request::{KeyValue, Request};

impl App<'_> {
    pub fn import_curl_file(&mut self, path_buf: &PathBuf, save_to: &str) {
        println!("Importing curl file from {:?} to {:?}", path_buf, save_to);

        let original_curl = match fs::read_to_string(path_buf) {
            Ok(original_curl) => original_curl,
            Err(e) => panic_error(format!("Could not read cURL file\n\t{e}")),
        };

        let curl = match parse(original_curl.as_str()) {
            // The first element is the curl command - for now we only support one per file
            Ok(curl) => curl.1,
            Err(e) => panic_error(format!("Could not parse cURL\n\t{e}")),
        };

        let (collection_name, req_name) = match validate_save_to(save_to) {
            Ok((collection_name, req_name)) => (collection_name, req_name),
            Err(e) => panic_error(format!("Could not validate chosen collection/request\n\t{e}")),
        };

        let collection_exists = self.collections.iter().any(|c| c.name == collection_name);

        if collection_exists {
            let imported = self
                .collections
                .iter_mut()
                .find(|c| c.name == collection_name.clone())
                .unwrap();
            imported.requests.push(Arc::new(RwLock::new(parse_request(&curl, req_name))));
        } else {
            let collection = Collection {
                name: collection_name.clone(),
                requests: vec![Arc::new(RwLock::new(parse_request(&curl, req_name)))],
                path: ARGS.directory.join(collection_name.clone() + ".json"),
            };

            self.collections.push(collection);
        }

        let imported_index = self.collections.iter().position(|c| c.name == collection_name).unwrap();
        self.save_collection_to_file(imported_index);
    }
}

fn parse_request(curl: &Curl, req_name: String) -> Request {
    println!("Found cURL: {:?}", curl);

    let mut request = Request::default();

    request.name = req_name;

    // Parse the URL so we can transform it
    let mut curl_url = match Url::parse(curl.url) {
        Ok(url) => url,
        Err(e) => panic_error(format!("Could not parse URL\n\t{e}")),
    };

    /* QUERY PARAMS */

    request.params = curl_url
        .query_pairs()
        .map(|(k, v)| KeyValue {
            enabled: true,
            data: (k.to_string(), v.to_string()),
        })
        .collect();
    
    /* URL */

    curl_url.set_query(None);
    request.url = curl_url.to_string();

    /* METHOD */

    request.method = get_http_method(&curl);

    /* HEADERS */

    request.headers = curl
        .options_headers_more
        .iter()
        .filter(|&(k, _)| !k.eq_ignore_ascii_case("Authorization")) // Exclude Authorization header, as that will be handled by the auth field
        .map(|(k, v)| KeyValue {
            enabled: true,
            data: (k.to_string(), v.to_string()),
        })
    .collect();

    /* AUTH */

    if let Some(auth) = curl.options_more.get("u") {
        let parts: Vec<&str> = auth.splitn(2, ':').collect();
        if parts.len() == 2 {
            request.auth = Auth::BasicAuth(parts[0].to_string(), parts[1].to_string());
        }
    } else if let Some(auth) = curl.options_headers_more.get("Authorization") {
        let parts: Vec<&str> = auth.split_whitespace().collect();
        if parts.len() > 1 && parts[0].starts_with("Bearer") {
            request.auth = Auth::BearerToken(parts[1].to_string());
        }
    }

    /* BODY */

    // TODO: Handle content type, for now we just assume raw
    request.body = ContentType::Raw(curl.options_data_raw.to_string());

    request
}

fn get_http_method(curl: &Curl) -> Method {
    if let Some(x) = curl.options_more.get("X") {
        match x {
            &"PUT" => Method::PUT,
            &"DELETE" => Method::DELETE,
            _ => Method::GET,
        }
    } else if !curl.options_data_raw.is_empty() {
        Method::POST
    } else {
        Method::GET
    }
}

fn validate_save_to(save_to: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = save_to.split('/').collect();

    if parts.len() != 2 {
        return Err("Path is not in the format collection/request name".to_string());
    }

    Ok((parts[0].to_string(), parts[1].to_string()))
}
