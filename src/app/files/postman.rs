use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use postman_collection::v2_1_0::{AuthType, Items, Language, Mode, RequestClass, RequestUnion, Url};
use crate::app::app::App;
use crate::app::startup::args::ARGS;
use crate::request::auth::Auth;
use crate::request::body::ContentType;
use crate::request::collection::Collection;
use crate::request::method::Method;
use crate::request::request::Request;
use crate::utils::stateful_custom_table::Param;


impl App<'_> {
    pub fn import_postman_collection(&mut self, path_buf: &PathBuf) {
        let postman_collection = postman_collection::from_path(path_buf).expect("\tCould not parse Postman collection");

        let collection_name = postman_collection.info.name.clone();

        for existing_collection in &self.collections {
            if existing_collection.name == collection_name {
                panic!("Collection \"{}\" already exists", collection_name);
            }
        }

        let mut collection = Collection {
            name: collection_name.clone(),
            requests: vec![],
            path: ARGS.directory.join(format!("{}.json", collection_name))
        };

        for item in postman_collection.item {
            if item.name.is_none() {
                continue;
            }

            let requests = parse_request_or_folder(item);

            for request in requests {
                collection.requests.push(Arc::new(RwLock::new(request)))
            }
        }

        self.collections.push(collection);

        let collection_index = self.collections.len() - 1;
        self.save_collection_to_file(collection_index);
    }
}

fn parse_request_or_folder(item: Items) -> Vec<Request> {
    if let Some(items) = item.item {
        let mut requests: Vec<Request> = vec![];

        for item in items {
            requests.extend(parse_request_or_folder(item));
        }

        return requests;
    }
    else {
        return vec![parse_request(item)];
    }
}

fn parse_request(item: Items) -> Request {
    let item_name = item.name.unwrap();

    println!("Found \"{}\"", item_name);

    let mut request = Request::default();

    request.name = item_name;

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
                request.method = Method::from_str(method).expect(&format!("Unknown method \"{method}\""));
            }

            /* AUTH */

            match retrieve_auth(&request_class) {
                None => {}
                Some(auth) => request.auth = auth
            }

            /* BODY */

            match retrieve_body(&request_class) {
                None => {}
                Some(content_type) => request.body = content_type
            }
        }
        RequestUnion::String(_) => {}
    }

    return request;
}

fn retrieve_query_params(request_class: &RequestClass) -> Option<Vec<Param>> {
    let url = request_class.url.clone()?;

    match url {
        Url::String(_) => None,
        Url::UrlClass(url_class) => {
            let mut query_params: Vec<Param> = vec![];

            for query_param in url_class.query? {
                query_params.push(Param {
                    enabled: !query_param.disabled.unwrap_or(false), // Set default to enabled
                    data: (query_param.key?, query_param.value?),
                })
            }

            Some(query_params)
        }
    }
}

fn retrieve_body(request_class: &RequestClass) -> Option<ContentType> {
    let body_as_raw = request_class.body.clone()?.raw?;

    let body_mode = request_class.body.clone()?.mode?;

    return match body_mode {
        Mode::Raw => {
            if let Some(options) = request_class.body.clone()?.options {
                let language = options.raw?.language?;

                let request_body = match language {
                    Language::Html => ContentType::Html(body_as_raw),
                    Language::Json => ContentType::Json(body_as_raw),
                    Language::Text => ContentType::Raw(body_as_raw),
                    Language::Xml => ContentType::Xml(body_as_raw)
                };

                Some(request_body)
            }
            else { 
                Some(ContentType::Raw(body_as_raw))
            }
        },
        Mode::File => None,
        Mode::Formdata => None,
        Mode::Urlencoded => None
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