// Mostly AI generated (Claude 3.7 Sonnet)

use std::fs;
use std::sync::Arc;

use anyhow::anyhow;
use openapiv3::{APIKeyLocation, OpenAPI, Operation, Parameter, ParameterSchemaOrContent, PathItem, ReferenceOr, RequestBody, Schema, SchemaKind, SecurityRequirement, SecurityScheme, Type};
use parking_lot::RwLock;
use reqwest::Url;

use thiserror::Error;

use crate::app::app::App;
use crate::cli::args::ARGS;
use crate::cli::cli_logic::import::openapi::ImportOpenApiError::InvalidUrl;
use crate::cli::commands::import::OpenApiImport;
use crate::models::auth::auth::Auth;
use crate::models::auth::basic::BasicAuth;
use crate::models::auth::bearer_token::BearerToken;
use crate::models::protocol::http::body::ContentType;
use crate::models::collection::Collection;
use crate::models::protocol::http::http::HttpRequest;
use crate::models::protocol::http::method::Method;
use crate::models::protocol::protocol::Protocol;
use crate::models::request::{KeyValue, Request};

#[derive(Error, Debug)]
enum ImportOpenApiError {
    #[error("Could not read OpenAPI file\n\t{0}")]
    CouldNotReadFile(String),
    #[error("Could not parse OpenAPI specification \"{0}\"\n\t{1}")]
    CouldNotParseSpec(String, String),
    #[error("Collection \"{0}\" already exists")]
    CollectionAlreadyExists(String),
    #[error("Invalid URL in OpenAPI spec: {0}")]
    InvalidUrl(String),
    #[error("Unknown content type: {0}")]
    UnknownContentType(String),
}

impl App<'_> {
    pub fn import_openapi_collection(&mut self, openapi_import: &OpenApiImport) -> anyhow::Result<()> {
        let path_buf = &openapi_import.import_path;

        println!("Parsing OpenAPI specification");

        // Read the file content
        let spec_content = match fs::read_to_string(path_buf) {
            Ok(content) => content,
            Err(e) => {
                return Err(anyhow!(ImportOpenApiError::CouldNotReadFile(e.to_string())));
            }
        };

        // Parse based on file extension
        let spec: OpenAPI = if path_buf.extension().map_or(false, |ext| ext == "json") {
            match serde_json::from_str(&spec_content) {
                Ok(spec) => spec,
                Err(e) => {
                    return Err(anyhow!(ImportOpenApiError::CouldNotParseSpec(
                        path_buf.display().to_string(),
                        e.to_string()
                    )));
                }
            }
        } else {
            // Assume YAML if not JSON
            match serde_yaml::from_str(&spec_content) {
                Ok(spec) => spec,
                Err(e) => {
                    return Err(anyhow!(ImportOpenApiError::CouldNotParseSpec(
                        path_buf.display().to_string(),
                        e.to_string()
                    )));
                }
            }
        };

        // Determine collection name
        let collection_name = spec.info.title.clone();

        println!("Collection name: {}", collection_name);

        // Check if collection already exists
        for existing_collection in &self.collections {
            if existing_collection.name == collection_name {
                return Err(anyhow!(ImportOpenApiError::CollectionAlreadyExists(collection_name)));
            }
        }

        let file_format = self.config.get_preferred_collection_file_format();

        // Create a new collection
        let mut collection = Collection {
            name: collection_name.clone(),
            last_position: Some(self.collections.len() - 1),
            requests: Vec::new(),
            path: ARGS.directory.as_ref().unwrap().join(format!("{}.{}", collection_name, file_format.to_string())),
            file_format,
        };

        // Parse and add all requests from paths
        let base_url = match spec.servers.first()  {
            Some(server) => match Url::parse(server.url.clone().as_str()) {
                Ok(url) => url.to_string(),
                Err(error) => {
                    return Err(anyhow!(InvalidUrl(error.to_string())))
                }
            },
            None => String::from("https://example.com")
        };

        // Process all paths and operations
        for (path, path_item) in spec.paths.iter().by_ref() {
            match path_item {
                ReferenceOr::Reference { reference: _ } => {
                    // Handle references - would need to resolve them
                    println!("\tSkipping reference for path: {}", path);
                }
                ReferenceOr::Item(path_item) => {
                    // Process each HTTP method in this path
                    process_path_operations(&mut collection, &path_item, &path, &base_url, &spec)?;
                }
            }
        }

        println!("\tFound {} requests in OpenAPI spec", collection.requests.len());

        // Add the collection to app's collections
        self.collections.push(collection);

        // Save the collection to file
        self.save_collection_to_file(self.collections.len() - 1);

        Ok(())
    }
}

fn process_path_operations(collection: &mut Collection, path_item: &PathItem, path: &str, base_url: &str, spec: &OpenAPI) -> anyhow::Result<()> {
    // Process GET operations
    if let Some(op) = &path_item.get {
        let name = op.operation_id.clone().unwrap_or_else(|| format!("GET {}", path));
        let request = create_request(name, Method::GET, path, base_url, op, spec)?;
        collection.requests.push(Arc::new(RwLock::new(request)));
    }

    // Process POST operations
    if let Some(op) = &path_item.post {
        let name = op.operation_id.clone().unwrap_or_else(|| format!("POST {}", path));
        let request = create_request(name, Method::POST, path, base_url, op, spec)?;
        collection.requests.push(Arc::new(RwLock::new(request)));
    }

    // Process PUT operations
    if let Some(op) = &path_item.put {
        let name = op.operation_id.clone().unwrap_or_else(|| format!("PUT {}", path));
        let request = create_request(name, Method::PUT, path, base_url, op, spec)?;
        collection.requests.push(Arc::new(RwLock::new(request)));
    }

    // Process DELETE operations
    if let Some(op) = &path_item.delete {
        let name = op.operation_id.clone().unwrap_or_else(|| format!("DELETE {}", path));
        let request = create_request(name, Method::DELETE, path, base_url, op, spec)?;
        collection.requests.push(Arc::new(RwLock::new(request)));
    }

    // Process PATCH operations
    if let Some(op) = &path_item.patch {
        let name = op.operation_id.clone().unwrap_or_else(|| format!("PATCH {}", path));
        let request = create_request(name, Method::PATCH, path, base_url, op, spec)?;
        collection.requests.push(Arc::new(RwLock::new(request)));
    }

    // Process OPTIONS operations
    if let Some(op) = &path_item.options {
        let name = op.operation_id.clone().unwrap_or_else(|| format!("OPTIONS {}", path));
        let request = create_request(name, Method::OPTIONS, path, base_url, op, spec)?;
        collection.requests.push(Arc::new(RwLock::new(request)));
    }

    // Process HEAD operations
    if let Some(op) = &path_item.head {
        let name = op.operation_id.clone().unwrap_or_else(|| format!("HEAD {}", path));
        let request = create_request(name, Method::HEAD, path, base_url, op, spec)?;
        collection.requests.push(Arc::new(RwLock::new(request)));
    }

    Ok(())
}

fn create_request(name: String, method: Method, path: &str, base_url: &str, operation: &Operation, spec: &OpenAPI) -> anyhow::Result<Request> {
    println!("\tFound request \"{}\"", name);

    let mut request = Request {
        name,
        url: format!("{}{}", base_url, path),
        protocol: Protocol::HttpRequest(HttpRequest {
            method,
            body: ContentType::NoBody,
        }),
        ..Default::default()
    };

    // Process parameters (query params, headers)
    process_parameters(&mut request, operation, path, spec)?;

    // Process request body
    if let Some(req_body) = &operation.request_body {
        process_request_body(&mut request, req_body, spec)?;
    }

    // Process security schemes
    if let Some(security_requirements) = &operation.security {
        process_security(&mut request, security_requirements, spec)?;
    } else if let Some(security_requirements) = &spec.security {
        process_security(&mut request, security_requirements, spec)?;
    }

    Ok(request)
}

fn process_parameters(request: &mut Request, operation: &Operation, path: &str, spec: &OpenAPI) -> anyhow::Result<()> {
    // Process path parameters
    let path_params: Vec<KeyValue> = path
        .split('/')
        .filter_map(|segment| {
            if segment.starts_with('{') && segment.ends_with('}') {
                let param_name = segment[1..segment.len()-1].to_string();
                Some(KeyValue {
                    enabled: true,
                    data: (param_name.clone(), format!("{{{}}}", param_name)),
                })
            } else {
                None
            }
        })
        .collect();
    
    request.params.extend(path_params);
    
    // Process operation parameters
    for param_or_ref in &operation.parameters {
        let param = resolve_parameter_reference(param_or_ref, spec)?;

        match param {
            Parameter::Query { parameter_data, .. } => {
                // Add query parameter
                let default_value = extract_default_value(&parameter_data.format);

                request.params.push(KeyValue {
                    enabled: !parameter_data.required,
                    data: (parameter_data.name.clone(), default_value.unwrap_or(String::from("value"))),
                });
            },
            Parameter::Header { parameter_data, .. } => {
                // Add header
                let default_value = extract_default_value(&parameter_data.format).unwrap_or(String::from("value"));

                request.modify_or_create_header(&parameter_data.name, &default_value);
            },
            // Skip other parameter types (path params are handled separately)
            _ => {}
        }
    }

    Ok(())
}

fn resolve_parameter_reference<'a>(param_or_ref: &'a ReferenceOr<Parameter>, spec: &'a OpenAPI) -> anyhow::Result<&'a Parameter> {
    match param_or_ref {
        ReferenceOr::Item(param) => Ok(param),
        ReferenceOr::Reference { reference } => {
            // Extract the parameter name from the reference
            let parts: Vec<&str> = reference.split('/').collect();
            if parts.len() < 4 || parts[1] != "components" || parts[2] != "parameters" {
                return Err(anyhow!("Invalid parameter reference: {}", reference));
            }

            let param_name = parts[3];

            // Look up the parameter in the components
            if let Some(components) = &spec.components {
                if let Some(param) = components.parameters.get(param_name) {
                    match param {
                        ReferenceOr::Item(p) => Ok(p),
                        ReferenceOr::Reference { .. } => {
                            Err(anyhow!("Nested references are not supported: {}", reference))
                        }
                    }
                } else {
                    Err(anyhow!("Parameter reference not found: {}", reference))
                }
            } else {
                Err(anyhow!("Components section not found in the spec"))
            }
        }
    }
}

fn extract_default_value(format: &ParameterSchemaOrContent) -> Option<String> {
    match format {
        ParameterSchemaOrContent::Schema(schema_or_ref) => {
            match schema_or_ref {
                ReferenceOr::Item(schema) => {
                    // Extract default value from schema if available
                    schema.schema_data.default.as_ref().map(|v| {
                        match v {
                            serde_json::Value::String(s) => s.clone(),
                            serde_json::Value::Number(n) => n.to_string(),
                            serde_json::Value::Bool(b) => b.to_string(),
                            _ => String::from("value"),
                        }
                    })
                },
                ReferenceOr::Reference { .. } => None,
            }
        },
        ParameterSchemaOrContent::Content(_) => None,
    }
}

fn process_request_body(request: &mut Request, req_body_or_ref: &ReferenceOr<RequestBody>, spec: &OpenAPI) -> anyhow::Result<()> {
    let req_body = match req_body_or_ref {
        ReferenceOr::Item(body) => body,
        ReferenceOr::Reference { reference } => {
            // Extract the request body name from the reference
            let parts: Vec<&str> = reference.split('/').collect();
            if parts.len() < 4 || parts[1] != "components" || parts[2] != "requestBodies" {
                return Err(anyhow!("Invalid request body reference: {}", reference));
            }

            let body_name = parts[3];

            // Look up the request body in the components
            if let Some(components) = &spec.components {
                if let Some(body) = components.request_bodies.get(body_name) {
                    match body {
                        ReferenceOr::Item(b) => b,
                        ReferenceOr::Reference { .. } => {
                            return Err(anyhow!("Nested references are not supported: {}", reference));
                        }
                    }
                } else {
                    return Err(anyhow!("Request body reference not found: {}", reference));
                }
            } else {
                return Err(anyhow!("Components section not found in the spec"));
            }
        }
    };

    // Process content based on media type
    if let Some((content_type, media_type)) = req_body.content.iter().next() {
        // Set the appropriate Content-Type header
        request.modify_or_create_header("content-type", content_type);
        let http_request = request.get_http_request_mut()?;

        // Create a sample request body based on the media type
        match content_type.as_str() {
            "application/json" => {
                if let Some(schema) = &media_type.schema {
                    // Generate a sample JSON body
                    let sample_json = generate_sample_json(schema, spec)?;
                    http_request.body = ContentType::Json(sample_json);
                } else {
                    http_request.body = ContentType::Json("{}".to_string());
                }
            },
            "application/xml" => {
                http_request.body = ContentType::Xml("<root></root>".to_string());
            },
            form @ "application/x-www-form-urlencoded" | form @ "multipart/form-data" => {
                let mut form_data = Vec::new();

                if let Some(schema) = &media_type.schema {
                    if let ReferenceOr::Item(schema) = schema {
                        if let SchemaKind::Type(Type::Object(obj)) = &schema.schema_kind {
                            for (prop_name, schema) in &obj.properties {
                                form_data.push(KeyValue {
                                    enabled: true,
                                    data: (prop_name.clone(), generate_sample_json(&schema.clone().unbox(), spec)?),
                                });
                            }
                        }
                    }
                }

                http_request.body = match form {
                    "application/x-www-form-urlencoded" => ContentType::Form(form_data),
                    "multipart/form-data" =>  ContentType::Multipart(form_data),
                    _ => unreachable!()
                };
            },
            "text/plain" => {
                http_request.body = ContentType::Raw("Sample text".to_string());
            },
            "text/html" => {
                http_request.body = ContentType::Html("<html><body>Sample HTML</body></html>".to_string());
            },
            "application/javascript" => {
                http_request.body = ContentType::Javascript("console.log('Sample JavaScript');".to_string());
            },
            _ => {
                return Err(anyhow!(ImportOpenApiError::UnknownContentType(content_type.clone())));
            }
        }
    }

    Ok(())
}

fn generate_sample_json(schema_or_ref: &ReferenceOr<Schema>, spec: &OpenAPI) -> anyhow::Result<String> {
    let schema = match schema_or_ref {
        ReferenceOr::Item(schema) => schema,
        ReferenceOr::Reference { reference } => {
            // Extract the schema name from the reference
            let parts: Vec<&str> = reference.split('/').collect();
            if parts.len() < 4 || parts[1] != "components" || parts[2] != "schemas" {
                return Err(anyhow!("Invalid schema reference: {}", reference));
            }

            let schema_name = parts[3];

            // Look up the schema in the components
            if let Some(components) = &spec.components {
                if let Some(schema) = components.schemas.get(schema_name) {
                    match schema {
                        ReferenceOr::Item(s) => s,
                        ReferenceOr::Reference { .. } => {
                            return Err(anyhow!("Nested references are not supported: {}", reference));
                        }
                    }
                } else {
                    // Default if not found
                    return Ok("{}".to_string());
                }
            } else {
                // Default if components not found
                return Ok("{}".to_string());
            }
        }
    };

    // Generate sample JSON based on schema type
    let sample_value = match &schema.schema_kind {
        SchemaKind::Type(schema_type) => {
            match schema_type {
                Type::String(_) => serde_json::Value::String("string".to_string()),
                Type::Number(_) => serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap()),
                Type::Integer(_) => serde_json::Value::Number(serde_json::Number::from(0)),
                Type::Boolean(_) => serde_json::Value::Bool(false),
                Type::Object(obj) => {
                    let mut props = serde_json::Map::new();
                    for (prop_name, prop_schema) in &obj.properties {
                        if let Ok(sample) = generate_sample_json(&prop_schema.clone().unbox(), spec) {
                            if let Ok(v) = serde_json::from_str(&sample) {
                                props.insert(prop_name.clone(), v);
                            }
                        }
                    }
                    serde_json::Value::Object(props)
                },
                Type::Array(arr) => match &arr.items {
                    Some(items) => match generate_sample_json(&items.clone().unbox(), spec) {
                        Ok(sample) => match serde_json::from_str(&sample) {
                            Ok(value) => serde_json::Value::Array(vec![value]),
                            Err(_) => serde_json::Value::Array(vec![])
                        },
                        Err(_) => serde_json::Value::Array(vec![])
                    }
                    None => serde_json::Value::Array(vec![])
                }
            }
        },
        SchemaKind::Not { not: _ } => {
            // Default empty object for 'not' schemas
            serde_json::Value::Object(serde_json::Map::new())
        },
        leftover => {
            let of = match leftover {
                SchemaKind::OneOf { one_of, .. } => one_of,
                SchemaKind::AllOf { all_of, .. } => all_of,
                SchemaKind::AnyOf { any_of, .. } => any_of,
                _ => unreachable!()
            };

            // Just take the first schema for a sample
            if let Some(first_schema) = of.first() {
                if let Ok(sample) = generate_sample_json(first_schema, spec) {
                    if let Ok(v) = serde_json::from_str(&sample) {
                        v
                    } else {
                        serde_json::Value::Object(serde_json::Map::new())
                    }
                } else {
                    serde_json::Value::Object(serde_json::Map::new())
                }
            } else {
                serde_json::Value::Object(serde_json::Map::new())
            }
        }
    };

    serde_json::to_string_pretty(&sample_value).map_err(|e| anyhow!("Failed to serialize sample JSON: {}", e))
}

fn process_security(request: &mut Request, security_requirements: &Vec<SecurityRequirement>, spec: &OpenAPI) -> anyhow::Result<()> {
    // Process only the first security requirement for simplicity
    if let Some(security_req) = security_requirements.first() {
        // Take only the first security scheme for simplicity
        if let Some((scheme_name, _scopes)) = security_req.iter().next() {
            // Find the security scheme in components
            if let Some(components) = &spec.components {
                if let Some(scheme) = components.security_schemes.get(scheme_name) {
                    match scheme {
                        ReferenceOr::Item(scheme) => {
                            match &scheme {
                                SecurityScheme::APIKey { name, location, .. } => {
                                    match location {
                                        APIKeyLocation::Header => request.modify_or_create_header(
                                            &name.clone(),
                                            "API_KEY"
                                        ),
                                        APIKeyLocation::Query => request.params.push(KeyValue {
                                            enabled: true,
                                            data: (name.clone(), "API_KEY".to_string()),
                                        }),
                                        // Not supported
                                        APIKeyLocation::Cookie => {}
                                    }
                                },
                                SecurityScheme::HTTP { scheme, .. } => {
                                    request.auth = match scheme.as_str() {
                                        "basic" => Auth::BasicAuth(BasicAuth {
                                            username: "username".to_string(),
                                            password: "password".to_string(),
                                        }),
                                        "bearer" => Auth::BearerToken(BearerToken {
                                            token: "BEARER_TOKEN".to_string(),
                                        }),
                                        // Digest not supported
                                        _ => Auth::NoAuth
                                    }
                                },
                                // Not supported
                                SecurityScheme::OAuth2 { .. } | SecurityScheme::OpenIDConnect { .. } => {}
                            }
                        },
                        // For simplicity, not resolving nested references
                        ReferenceOr::Reference { .. } => {}
                    }
                }
            }
        }
    }

    Ok(())
}