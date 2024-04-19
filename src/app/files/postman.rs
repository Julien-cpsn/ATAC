use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use parse_postman_collection::v2_1_0::{AuthType, Body, FormParameterSrcUnion, HeaderUnion, Items, Language, Mode, RequestClass, RequestUnion, Url};

use crate::app::app::App;
use crate::app::startup::args::ARGS;
use crate::panic_error;
use crate::request::auth::Auth;
use crate::request::body::ContentType;
use crate::request::collection::Collection;
use crate::request::method::Method;
use crate::request::request::{DEFAULT_HEADERS, KeyValue, Request};
use crate::request::settings::RequestSettings;

impl App<'_> {
    pub fn import_postman_collection(&mut self, path_buf: &PathBuf, max_depth: u16) {
        println!("Parsing Postman collection");

        let mut postman_collection = match parse_postman_collection::from_path(path_buf) {
            Ok(postman_collection) => postman_collection,
            Err(e) => panic_error(format!("Could not parse Postman collection\n\t{e}"))
        };

        let collection_name = postman_collection.info.name.clone();

        println!("Collection name: {}", collection_name);

        for existing_collection in &self.collections {
            if existing_collection.name == collection_name {
                panic!("Collection \"{}\" already exists", collection_name);
            }
        }

        let mut collections: Vec<Collection> = vec![
            Collection {
                name: collection_name.clone(),
                requests: vec![],
                path: ARGS.directory.join(format!("{}.json", collection_name))
            }
        ];
        
        let mut depth_level: u16 = 0;

        if max_depth == 0 {
            for item in postman_collection.item.iter_mut() {
                collections[0].requests.extend(recursive_get_requests(item));
            }
        }
        else {
            for mut item in postman_collection.item {
                if item.name.is_none() {
                    continue;
                }

                // If this is a folder
                if is_folder(&item) {
                    let mut temp_nesting_prefix = String::new();
                    let new_collections: Vec<Collection> = vec![];

                    recursive_has_requests(&mut item, &mut collections, &mut temp_nesting_prefix, &mut depth_level, max_depth);

                    collections.extend(new_collections);
                } else {
                    collections[0].requests.push(Arc::new(RwLock::new(parse_request(item))));
                }
            }
        }
        
        // Prevent from having an empty collection
        if collections.len() > 1 && collections[0].requests.len() == 0 {
            collections.remove(0);
        }

        let collections_length = collections.len();

        self.collections.extend(collections);

        for collection_index in 0..collections_length {
            self.save_collection_to_file(collection_index);
        }
    }
}

fn recursive_has_requests(item: &mut Items, collections: &mut Vec<Collection>, mut nesting_prefix: &mut String, mut depth_level: &mut u16, max_depth: u16) -> Option<Arc<RwLock<Request>>> {
    return if is_folder(&item) {
        let mut requests: Vec<Arc<RwLock<Request>>> = vec![];

        let mut folder_name = item.clone().name.unwrap();
        folder_name = folder_name.replace("/", "-");
        folder_name = folder_name.replace("\\", "-");
        folder_name = folder_name.trim().to_string();
        
        let collection_name = format!("{nesting_prefix}{folder_name}");

        *depth_level += 1;

        if *depth_level == max_depth {
            println!("\tMet max depth level");
            requests = recursive_get_requests(item);
        }
        else {
            nesting_prefix.push_str(&format!("{folder_name} "));

            let mut has_sub_folders = false;

            for mut sub_item in item.item.clone().unwrap() {
                if let Some(request) = recursive_has_requests(&mut sub_item, collections, &mut nesting_prefix, &mut depth_level, max_depth) {
                    requests.push(request);
                } else {
                    has_sub_folders = true;
                }
            }

            if has_sub_folders {
                nesting_prefix.clear();
            }
        }

        if requests.len() > 0 {
            println!("\tFound collection \"{}\"", collection_name);

            let collection = Collection {
                name: collection_name.clone(),
                requests,
                path: ARGS.directory.join(format!("{}.json", collection_name)),
            };

            collections.push(collection);
            *depth_level -= 1;
        }

        None
    } else {
        Some(Arc::new(RwLock::new(parse_request(item.clone()))))
    }
}

fn recursive_get_requests(item: &mut Items) -> Vec<Arc<RwLock<Request>>> {
    return if let Some(items) = &mut item.item {
        let mut requests: Vec<Arc<RwLock<Request>>> = vec![];

        for item in items {
            requests.extend(recursive_get_requests(item));
        }

        requests
    } else {
        vec![Arc::new(RwLock::new(parse_request(item.clone())))]
    }
}

fn is_folder(folder: &Items) -> bool {
    folder.item.is_some()
}

fn parse_request(item: Items) -> Request {
    let item_name = item.name.clone().unwrap();

    println!("\t\tFound request \"{}\"", item_name);

    let mut request = Request::default();

    request.name = item_name;

    /* SETTINGS */

    // TODO: update parse_postman_collection to handle "protocolProfileBehavior"
    match retrieve_settings(&item) {
        None => {}
        Some(request_settings) => request.settings = request_settings
    }

    /* REQUEST */

    let item_request = item.request.unwrap();

    match &item_request {
        RequestUnion::RequestClass(request_class) => {
            /* URL */

            if let Some(url) = &request_class.url {
                match url {
                    Url::String(url) => request.url = url.to_string(),
                    Url::UrlClass(url_class) => request.url = url_class.raw.clone().unwrap()
                }
            }

            /* QUERY PARAMS */

            match retrieve_query_params(&request_class) {
                None => {}
                Some(query_params) => request.params = query_params
            }

            /* METHOD */

            if let Some(method) = &request_class.method {
                request.method = match Method::from_str(method) {
                    Ok(method) => method,
                    Err(_) => panic_error(format!("Unknown method \"{method}\""))
                };
            }

            /* AUTH */

            match retrieve_auth(&request_class) {
                None => {}
                Some(auth) => request.auth = auth
            }

            /* HEADERS */

            match retrieve_headers(&request_class) {
                None => {}
                Some(headers) => request.headers = headers
            }

            /* BODY */

            match retrieve_body(&request_class) {
                None => {}
                Some(body) => {
                    match &body {
                        ContentType::Multipart(_) => {} // TODO: Not handled yet
                        body_type => {
                            let content_type = body_type.to_content_type().clone();
                            request.modify_or_create_header("content-type", &content_type);
                        }
                    }

                    request.body = body;
                }
            }
        }
        RequestUnion::String(_) => {}
    }

    return request;
}

fn retrieve_query_params(request_class: &RequestClass) -> Option<Vec<KeyValue>> {
    let url = request_class.url.clone()?;

    match url {
        Url::String(_) => None,
        Url::UrlClass(url_class) => {
            let mut query_params: Vec<KeyValue> = vec![];

            for query_param in url_class.query? {
                query_params.push(KeyValue {
                    enabled: !query_param.disabled.unwrap_or(false), // Set default to enabled
                    data: (query_param.key?, query_param.value?),
                })
            }

            Some(query_params)
        }
    }
}

fn retrieve_body(request_class: &RequestClass) -> Option<ContentType> {
    let body = request_class.body.clone()?;

    match body {
        Body::String(body_as_raw) => Some(ContentType::Raw(body_as_raw)),
        Body::BodyClass(body) => {
            let body_mode = body.mode?;

            return match body_mode {
                Mode::Raw => {
                    let body_as_raw = body.raw?;

                    if let Some(options) = body.options {
                        let language = options.raw?.language?;

                        let request_body = match language {
                            Language::Html => ContentType::Html(body_as_raw),
                            Language::Json => ContentType::Json(body_as_raw),
                            Language::Text => ContentType::Raw(body_as_raw),
                            Language::Xml => ContentType::Xml(body_as_raw),
                            Language::Javascript => ContentType::Javascript(body_as_raw),
                        };

                        Some(request_body)
                    }
                    else {
                        Some(ContentType::Raw(body_as_raw))
                    }
                },
                Mode::File => { 
                    let file = body.file?;
                    let file_path = file.src?;
                    
                    Some(ContentType::File(file_path))
                },
                Mode::Formdata => {
                    let form_data = body.formdata?;

                    let mut multipart: Vec<KeyValue> = vec![];

                    for param in form_data {
                        let param_type = param.form_parameter_type?;

                        let key_value = match param_type.as_str() {
                            "text" => KeyValue {
                                enabled: true,
                                data: (param.key, param.value.unwrap_or(String::new())),
                            },
                            "file" => {
                                let file = match param.src? {
                                    FormParameterSrcUnion::File(file) => file,
                                    // If there are many files, tries to get the first one
                                    FormParameterSrcUnion::Files(files) => files.get(0)?.to_string()
                                };

                                KeyValue {
                                    enabled: true,
                                    data: (param.key, format!("!!{file}")),
                                }
                            },
                            param_type => {
                                println!("\t\t\tUnknown Multipart form type \"{param_type}\"");
                                return None;
                            }
                        };

                        multipart.push(key_value);
                    }

                    Some(ContentType::Multipart(multipart))
                },
                Mode::Urlencoded => {
                    let form_data = body.urlencoded?;

                    let mut url_encoded: Vec<KeyValue> = vec![];

                    for param in form_data {
                        let value = param.value.unwrap_or(String::new());
                        let is_disabled = param.disabled.unwrap_or(false);

                        let key_value = KeyValue {
                            enabled: !is_disabled,
                            data: (param.key, value),
                        };

                        url_encoded.push(key_value);
                    }

                    Some(ContentType::Form(url_encoded))
                }
            }
        }
    }
}

fn retrieve_auth(request_class: &RequestClass) -> Option<Auth> {
    let auth = request_class.auth.clone()?;

    match auth.auth_type {
        AuthType::Basic => {
            let basic_attributes = auth.basic?;

            let mut username = String::new();
            let mut password = String::new();

            for basic_attribute in basic_attributes {
                match basic_attribute.key.as_str() {
                    "username" => username = basic_attribute.value.unwrap().as_str()?.to_string(),
                    "password" => password = basic_attribute.value.unwrap().as_str()?.to_string(),
                    _ => {}
                }
            }

            Some(Auth::BasicAuth(username, password))
        },
        AuthType::Bearer => {
            let bearer_token_attributes = auth.bearer?;

            let mut bearer_token = String::new();

            for bearer_token_attribute in bearer_token_attributes {
                match bearer_token_attribute.key.as_str() {
                    "token" => bearer_token = bearer_token_attribute.value.unwrap().as_str()?.to_string(),
                    _ => {}
                }
            }

            Some(Auth::BearerToken(bearer_token))
        },
        AuthType::Awsv4 => None,
        AuthType::Digest => None,
        AuthType::Hawk => None,
        AuthType::Noauth => None,
        AuthType::Ntlm => None,
        AuthType::Oauth1 => None,
        AuthType::Oauth2 => None,
    }
}

fn retrieve_headers(request_class: &RequestClass) -> Option<Vec<KeyValue>> {
    let headers = request_class.header.clone()?;

    let mut headers_to_return: Vec<KeyValue> = DEFAULT_HEADERS.clone();

    match headers {
        HeaderUnion::HeaderArray(headers) => {
            for header in headers {
                headers_to_return.push(KeyValue {
                    enabled: !header.disabled.unwrap_or(false),
                    data: (header.key, header.value),
                })
            }

            Some(headers_to_return)
        }
        HeaderUnion::String(_) => None
    }
}

fn retrieve_settings(item: &Items) -> Option<RequestSettings> {
    let protocol_profile_behavior = item.protocol_profile_behavior.clone()?;

    let mut settings = RequestSettings::default();

    if let Some(follow_redirects) = protocol_profile_behavior.follow_redirects {
        settings.allow_redirects = follow_redirects;
    }
    
    if let Some(disable_cookies) = protocol_profile_behavior.disable_cookies {
        settings.store_received_cookies = !disable_cookies;
    }
    
    Some(settings)
}