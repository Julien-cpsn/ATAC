use base64::prelude::BASE64_STANDARD;
use base64::write::EncoderWriter;
use reqwest::Url;
use std::path::PathBuf;
use anyhow::anyhow;
use thiserror::Error;
use crate::app::app::App;
use crate::app::business_logic::request::export::ExportError::{CouldNotParseUrl, ExportFormatNotSupported};
use crate::app::business_logic::request::send::{ do_jaat, get_file_content_with_name};
use crate::models::auth::Auth;
use crate::models::protocol::http::body::ContentType::{File, Form, Html, Javascript, Json, Multipart, NoBody, Raw, Xml};
use crate::models::export::ExportFormat;
use crate::models::export::ExportFormat::{Curl, NodeJsAxios, PhpGuzzle, RustReqwest, HTTP};
use crate::models::protocol::http::method::Method;
use crate::models::protocol::protocol::Protocol;
use crate::models::request::Request;

#[derive(Error, Debug)]
enum ExportError {
    #[error("Could not parse URL")]
    CouldNotParseUrl,

    #[error("Export format not supported for the {0} protocol")]
    ExportFormatNotSupported(String),
}

impl App<'_> {
    pub fn export_request_to_string_with_format(&self, export_format: &ExportFormat, request: &Request) -> anyhow::Result<String> {
        let output = String::new();

        let params = self.key_value_vec_to_tuple_vec(&request.params);
        let url = self.replace_env_keys_by_value(&request.url);

        let url = match Url::parse_with_params(&url, &params) {
            Ok(url) => url,
            Err(_) => return Err(anyhow!(CouldNotParseUrl))
        };

        let headers = self.key_value_vec_to_tuple_vec(&request.headers);

        let export = match request.protocol {
            Protocol::HttpRequest(_) => match export_format {
                HTTP => self.raw_html(output, request, url, headers),
                Curl => self.curl(output, request, url, headers),
                PhpGuzzle => self.php_guzzle(output, request, url, headers),
                NodeJsAxios => self.node_axios(output, request, url, headers),
                RustReqwest => self.rust_request(output, request, url, headers),
            }
            Protocol::WsRequest(_) => match export_format {
                RustReqwest => self.rust_request(output, request, url, headers),
                PhpGuzzle | NodeJsAxios | HTTP | Curl => return Err(anyhow!(ExportFormatNotSupported(request.protocol.to_string())))
            }
        };

        Ok(export)
    }

    pub fn copy_request_export_to_clipboard(&mut self) {
        let content = &self.display_request_export.content;
        self.clipboard.set_text(content).expect("Could not copy request export to clipboard")
    }

    fn raw_html(&self, mut output: String, request: &Request, url: Url, headers: Vec<(String, String)>) -> String {
        let http_request = request.get_http_request().unwrap();

        /* URL & Query params */

        output += &format!(
            "{} {}{} HTTP/1.1",
            http_request.method.to_string(),
            url.path(),
            match url.query() {
                None => String::new(),
                Some(query) => format!("?{query}")
            }
        );

        output += &format!("\nHost: {}", url.host_str().unwrap_or("localhost"));

        /* Headers */

        for (header, value) in &headers {
            output += &format!("\n{}: {}", header, value);
        }

        /* Auth */

        output += &match &request.auth {
            Auth::NoAuth => String::new(),
            Auth::BasicAuth { username, password } => {
                let username = self.replace_env_keys_by_value(username);
                let password = self.replace_env_keys_by_value(password);

                format!("\nAuthorization: {}", encode_basic_auth(&username, &password))
            },
            Auth::BearerToken { token } => {
                let bearer_token = self.replace_env_keys_by_value(token);

                format!("\nAuthorization: Bearer {}", bearer_token)
            },
            Auth::JwtToken { algorythm, secret, payload } => {
                let algorythm = self.replace_env_keys_by_value(algorythm);
                let secret = self.replace_env_keys_by_value(secret);
                let payload = self.replace_env_keys_by_value(payload);

                let token = do_jaat(algorythm, secret, payload);
                format!("Authorization: Bearer {}", token)
            }
        };

        /* Body */

        output += &match &http_request.body {
            NoBody => String::new(),
            File(file_path) => {
                let file_path_with_env_values = self.replace_env_keys_by_value(file_path);

                let file_content = match get_file_content_with_name(PathBuf::from(file_path_with_env_values)) {
                    Ok((content, _)) => content,
                    Err(_) => {
                        return String::from("Could not open file");
                    }
                };

                format!("\n\n\"{}\"", String::from_utf8_lossy(&file_content))
            },
            Multipart(multipart) => {
                let boundary = "WebKitFormBoundaryTODO"; // TODO user proper boundary
                let mut multipart_output = format!("\nContent-Type: {}; boundary={}\n", &http_request.body.to_content_type(), boundary);

                for key_value in multipart {
                    if !key_value.enabled {
                        continue;
                    }

                    let key = self.replace_env_keys_by_value(&key_value.data.0);
                    let mut value = self.replace_env_keys_by_value(&key_value.data.1);

                    multipart_output += &format!("\n--{boundary}\nContent-Disposition: form-data; name=\"{key}\"");

                    // If the value starts with !!, then it is supposed to be a file
                    if value.starts_with("!!") {
                        let file_path = &value[2..];

                        let (file_content, file_name) = match get_file_content_with_name(PathBuf::from(file_path)) {
                            Ok(result) => result,
                            Err(_) => {
                                return String::from("Could not open file");
                            }
                        };

                        value = format!("; filename=\"{file_name}\"\nContent-Type: {}\n\n{}", &http_request.body.to_content_type(), String::from_utf8_lossy(&file_content));
                    }
                    else {
                        value = format!("\n\n{}", &value);
                    }

                    multipart_output += &value;
                }

                multipart_output += &format!("\n--{boundary}--");

                multipart_output
            }
            Form(form_data) => {
                let form = self.key_value_vec_to_tuple_vec(form_data);
                let mut form_str = String::new();

                for (key, value) in &form {
                    if !form_str.is_empty() {
                        form_str += "&";
                    }
                    form_str += &format!("{}={}", key, value);
                }

                let form_output = format!(
                    "\nContent-Type: {}\nContent-Length: {}\n\n{}",
                    &http_request.body.to_content_type(),
                    form_str.len(),
                    form_str
                );

                form_output
            }
            Raw(body) | Json(body) | Xml(body) | Html(body) | Javascript(body) => {
                format!(
                    "\nContent-Type: {}\nContent-Length: {}\n\n{}",
                    &http_request.body.to_content_type(),
                    body.len(),
                    body
                )
            }
        };

        output
    }
    
    fn curl(&self, mut output: String, request: &Request, url: Url, headers: Vec<(String, String)>) -> String {
        let http_request = request.get_http_request().unwrap();

        /* URL & Query params */

        output += &format!(
            "curl --location --request {} '{}' \\",
            http_request.method.to_string(),
            url
        );

        /* Auth */

        output += &match &request.auth {
            Auth::NoAuth => String::new(),
            Auth::BasicAuth { username, password } => {
                let username = self.replace_env_keys_by_value(username);
                let password = self.replace_env_keys_by_value(password);

                format!("\n--header '{}' \\", encode_basic_auth(&username, &password))
            },
            Auth::BearerToken { token } => {
                let bearer_token = self.replace_env_keys_by_value(token);

                format!("\n--header 'Authorization: Bearer {}' \\", bearer_token)
            },
            Auth::JwtToken { algorythm, secret, payload } => {
                let algorythm = self.replace_env_keys_by_value(algorythm);
                let secret = self.replace_env_keys_by_value(secret);
                let payload = self.replace_env_keys_by_value(payload);

                let token =do_jaat(algorythm, secret, payload); 
                format!("\n--header 'Authorization: Bearer {}' \\", token)
            }
        };

        /* Headers */

        for (header, value) in &headers {
            output += &format!("\n--header '{}: {}' \\", header, value);
        }

        /* Body */

        output += &match &http_request.body {
            NoBody => String::new(),
            File(file_path) => {
                let file_path_with_env_values = self.replace_env_keys_by_value(file_path);

                format!("\n--header 'Content-Type: {}' \\\n--data '@/{}' \\", &http_request.body.to_content_type(), file_path_with_env_values)
            },
            Multipart(multipart) => {
                let mut multipart_output = String::new();

                for key_value in multipart {
                    if !key_value.enabled {
                        continue;
                    }

                    let key = self.replace_env_keys_by_value(&key_value.data.0);
                    let mut value = self.replace_env_keys_by_value(&key_value.data.1);

                    // If the value starts with !!, then it is supposed to be a file
                    if value.starts_with("!!") {
                        value = format!("@\"{}\"", &value[2..]);
                    }
                    else {
                        value = format!("\"{}\"", &value);
                    }

                    multipart_output += &format!("\n--form '{}={}' \\", key, value);
                }

                multipart_output
            }
            Form(form_data) => {
                let mut form_output = format!("\n--header 'Content-Type: {}' \\", &http_request.body.to_content_type());

                let form = self.key_value_vec_to_tuple_vec(form_data);

                for (key, value) in form {
                    form_output += &format!("\n--data-urlencode '{}={}' \\", key, value);
                }

                form_output
            }
            Raw(body) | Json(body) | Xml(body) | Html(body) | Javascript(body) => {
                format!("\n--header 'Content-Type: {}' \\\n--data '{}' \\", &http_request.body.to_content_type(), body)
            }
        };

        if request.settings.use_config_proxy.as_bool() {
            if let Some(proxy) = &self.config.proxy {
                let mut proxy_output = String::from("--proxy ");

                if let Some(http_proxy) = &proxy.http_proxy {
                    proxy_output += &format!("\"{}\" ", http_proxy);
                }

                if let Some(https_proxy) = &proxy.https_proxy {
                    proxy_output += &format!("\"{}\" ", https_proxy);
                }

                output += &format!("\n{} \\", proxy_output)
            }
        }

        output.pop(); // Remove trailing char

        output
    }
    
    fn php_guzzle(&self, mut output: String, request: &Request, url: Url, headers: Vec<(String, String)>) -> String {
        let http_request = request.get_http_request().unwrap();

        output += "<?php\n$client = new GuzzleHttp\\Client();\n";

        let mut headers_str = String::from("\n$headers = [");

        /* Auth */

        headers_str += &match &request.auth {
            Auth::NoAuth => String::new(),
            Auth::BasicAuth { username, password } => {
                let username = self.replace_env_keys_by_value(username);
                let password = self.replace_env_keys_by_value(password);

                format!("\n    'Authorization' => '{}',", encode_basic_auth(&username, &password))
            },
            Auth::BearerToken { token } => {
                let bearer_token = self.replace_env_keys_by_value(token);

                format!("\n    'Authorization' => 'Bearer {}',", bearer_token)
            },
            Auth::JwtToken { algorythm, secret, payload } => {
                let algorythm = self.replace_env_keys_by_value(algorythm);
                let secret = self.replace_env_keys_by_value(secret);
                let payload = self.replace_env_keys_by_value(payload);

                let token = do_jaat(algorythm, secret, payload);
                format!("\n--header 'Authorization: Bearer {}' \\", token)
            }
        };

        /* Headers */

        for (header, value) in &headers {
            headers_str += &format!("\n    '{}' => '{}',", header, value);
        }

        /* Proxy */

        if request.settings.use_config_proxy.as_bool() {
            if let Some(proxy) = &self.config.proxy {
                let mut proxy_output = String::from("\n    'proxy' => [");

                if let Some(http_proxy) = &proxy.http_proxy {
                    proxy_output += &format!("\n        'http' => '{}',", http_proxy);
                }

                if let Some(https_proxy) = &proxy.https_proxy {
                    proxy_output += &format!("\n        'https' => '{}',", https_proxy);
                }

                headers_str += &format!("{}\n    ],", proxy_output);
            }
        }

        headers_str += "\n];\n";
        output += &headers_str;

        /* Body */

        let mut has_body = false;
        let mut options_str: Option<String> = None;

        match &http_request.body {
            NoBody => {},
            File(file_path) => {
                let file_path_with_env_values = self.replace_env_keys_by_value(file_path);

                let file_content = match get_file_content_with_name(PathBuf::from(file_path_with_env_values)) {
                    Ok((content, _)) => content,
                    Err(_) => {
                        return String::from("Could not open file");
                    }
                };

                has_body = true;
                output += &format!("\n$body = '{}';", String::from_utf8_lossy(&file_content));
            },
            Multipart(multipart) => {
                let mut multipart_output = String::from("\n$options = [\n    'multipart' => [");

                for key_value in multipart {
                    if !key_value.enabled {
                        continue;
                    }

                    let key = self.replace_env_keys_by_value(&key_value.data.0);
                    let value = self.replace_env_keys_by_value(&key_value.data.1);

                    if value.starts_with("!!") {
                        let file_path = &value[2..];
                        multipart_output += &format!(
                            "\n        [\n            'name' => '{}',\n            'contents' => fopen('{}', 'r'),\n            'filename' => basename('{}')\n        ],",
                            key, file_path, file_path
                        );
                    } else {
                        multipart_output += &format!(
                            "\n        [\n            'name' => '{}',\n            'contents' => '{}'\n        ],",
                            key, value
                        );
                    }
                }

                multipart_output += "\n    ]\n];\n";
                options_str = Some(multipart_output);
            }
            Form(form_data) => {
                let mut form_output = String::from("\n$options = [\n    'form_params' => [");

                let form = self.key_value_vec_to_tuple_vec(form_data);

                for (key, value) in form {
                    form_output += &format!("\n        '{}' => '{}',", key, value);
                }

                form_output += "\n    ]\n];\n";
                options_str = Some(form_output);
            }
            Raw(body) | Json(body) | Xml(body) | Html(body) | Javascript(body) => {
                has_body = true;
                output += &format!("\n$body = '{}';\n", body);

                if matches!(http_request.body, Json(_)) {
                    options_str = Some("\n$options = [\n    'json' => json_decode($body, true)\n];\n".to_string());
                    has_body = false;
                } else {
                    output += &format!("\n$headers['Content-Type'] = '{}';\n", http_request.body.to_content_type());
                }
            }
        };

        if let Some(options) = options_str {
            output += &options;
            output += &format!("\n$response = $client->request('{}', '{}', [", http_request.method, url);
            output += "\n    'headers' => $headers,";
            if has_body {
                output += "\n    'body' => $body,";
            }
            output += "\n    ...$options";
            output += "\n]);";
        } else {
            output += &format!("\n$request = new GuzzleHttp\\Psr7\\Request('{}', '{}', $headers", http_request.method, url);
            if has_body {
                output += ", $body";
            }
            output += ");\n$response = $client->send($request);";
        }

        output += "\n\necho $response->getBody();";

        output
    }
    
    fn node_axios(&self, mut output: String, request: &Request, url: Url, headers: Vec<(String, String)>) -> String {
        let http_request = request.get_http_request().unwrap();

        output += "const axios = require('axios');\n";

        /* Headers and Config */
        output += "const config = {\n";
        output += &format!("  method: '{}',\n", http_request.method.to_string().to_lowercase());
        output += &format!("  url: '{}',\n", url);

        /* Headers */
        output += "  headers: { \n";

        /* Auth */
        match &request.auth {
            Auth::NoAuth => {},
            Auth::BasicAuth { username, password } => {
                let username = self.replace_env_keys_by_value(username);
                let password = self.replace_env_keys_by_value(password);
                output += &format!("    'Authorization': '{}',\n", encode_basic_auth(&username, &password));
            },
            Auth::BearerToken { token } => {
                let bearer_token = self.replace_env_keys_by_value(token);
                output += &format!("    'Authorization': 'Bearer {}',\n", bearer_token);
            },
            Auth::JwtToken { algorythm, secret, payload } => {
                let algorythm = self.replace_env_keys_by_value(algorythm);
                let secret = self.replace_env_keys_by_value(secret);
                let payload = self.replace_env_keys_by_value(payload);

                let token = do_jaat(algorythm, secret, payload);
                output += &format!("    'Authorization': 'Bearer {}',\n", token);
            }
        };

        /* Regular Headers */
        for (header, value) in &headers {
            output += &format!("    '{}': '{}',\n", header, value);
        }

        output += "  }";

        /* Body */
        match &http_request.body {
            NoBody => {},
            File(file_path) => {
                let file_path_with_env_values = self.replace_env_keys_by_value(file_path);
                output += ",\n  data: fs.readFileSync('";
                output += &file_path_with_env_values;
                output += "', 'utf8')";
            },
            Multipart(multipart) => {
                output += ",\n  data: new FormData()";

                // Need to initialize FormData first
                output += "\n\nconst FormData = require('form-data');\n";
                output += "const fs = require('fs');\n";
                output += "const formData = new FormData();\n";

                for key_value in multipart {
                    if !key_value.enabled {
                        continue;
                    }

                    let key = self.replace_env_keys_by_value(&key_value.data.0);
                    let value = self.replace_env_keys_by_value(&key_value.data.1);

                    if value.starts_with("!!") {
                        let file_path = &value[2..];
                        output += &format!("formData.append('{}', fs.createReadStream('{}'));\n", key, file_path);
                    } else {
                        output += &format!("formData.append('{}', '{}');\n", key, value);
                    }
                }

                output += "\nconfig.data = formData;\n";
            }
            Form(form_data) => {
                output += ",\n  data: new URLSearchParams()";

                // Need to initialize URLSearchParams
                output += "\n\nconst params = new URLSearchParams();\n";

                let form = self.key_value_vec_to_tuple_vec(form_data);
                for (key, value) in form {
                    output += &format!("params.append('{}', '{}');\n", key, value);
                }

                output += "\nconfig.data = params;\n";
            }
            Raw(body) => {
                output += ",\n  data: '";
                output += body;
                output += "'";
            }
            Json(body) => {
                output += ",\n  data: ";
                output += body;
            }
            Xml(body) | Html(body) | Javascript(body) => {
                output += ",\n  data: '";
                output += body;
                output += "'";
            }
        };

        /* Proxy */
        if request.settings.use_config_proxy.as_bool() {
            if let Some(proxy) = &self.config.proxy {
                output += ",\n  proxy: {";

                if let Some(http_proxy) = &proxy.http_proxy {
                    output += &format!("\n    http: '{}',", http_proxy);
                }

                if let Some(https_proxy) = &proxy.https_proxy {
                    output += &format!("\n    https: '{}',", https_proxy);
                }

                output += "\n  }";
            }
        }

        output += "\n};\n\n";

        /* Call and Response Handling */
        output += "axios(config)\n";
        output += "  .then(function (response) {\n";
        output += "    console.log(JSON.stringify(response.data));\n";
        output += "  })\n";
        output += "  .catch(function (error) {\n";
        output += "    console.log(error);\n";
        output += "  });";

        output
    }
    
    fn rust_request(&self, mut output: String, request: &Request, url: Url, headers: Vec<(String, String)>) -> String {
        let method = match &request.protocol {
            Protocol::HttpRequest(http_request) => http_request.method.to_string(),
            Protocol::WsRequest(_) => Method::GET.to_string()
        };

        /* Headers */
        let mut has_headers = false;
        let mut headers_str = String::new();

        for (header, value) in &headers {
            has_headers = true;
            headers_str += &format!("        .header(\"{}\", \"{}\")\n", header, value);
        }

        /* Auth */
        match &request.auth {
            Auth::NoAuth => {},
            Auth::BasicAuth { username, password } => {
                let username = self.replace_env_keys_by_value(username);
                let password = self.replace_env_keys_by_value(password);
                has_headers = true;
                headers_str += &format!("        .header(\"Authorization\", \"{}\")\n", encode_basic_auth(&username, &password));
            },
            Auth::BearerToken { token } => {
                let bearer_token = self.replace_env_keys_by_value(token);
                has_headers = true;
                headers_str += &format!("        .header(\"Authorization\", \"Bearer {}\")\n", bearer_token);
            },
            Auth::JwtToken { algorythm, secret, payload }=>{
                let algorythm = self.replace_env_keys_by_value(algorythm);
                let secret = self.replace_env_keys_by_value(secret);
                let payload = self.replace_env_keys_by_value(payload);

                let token = do_jaat(algorythm, secret, payload);
                headers_str += &format!("        .header(\"Authorization\", \"Bearer {}\")\n", token);
            }
        };

        /* Imports */

        output += "use std::error::Error;\n";
        output += "use reqwest::Client;\n";

        match &request.protocol {
            Protocol::HttpRequest(http_request) => match &http_request.body {
                NoBody => {},
                File(_) => {
                    output += "use std::fs;\n";
                },
                Multipart(_) => {
                    output += "use reqwest::multipart::{Form, Part};\n";
                    output += "use std::fs;\n";
                    output += "use std::path::Path;\n";
                },
                Form(_) => {
                    output += "use std::collections::HashMap;\n";
                },
                Json(_) => {
                    output += "use serde_json::json;\n";
                },
                _ => {}
            }
            Protocol::WsRequest(_) => {
                output += "use reqwest_websocket::{Error, Message, RequestBuilderExt};\nuse futures_util::{SinkExt, StreamExt, TryStreamExt};\n";
            }
        }

        /* Main function */
        output += "\n#[tokio::main]\nasync fn main() -> Result<(), Box<dyn Error>> {\n";
        output += "    let client = Client::new();\n\n";

        /* Body preparation */
        let mut body_str = String::new();

        match &request.protocol {
            Protocol::HttpRequest(http_request) => match &http_request.body {
                NoBody => {},
                File(file_path) => {
                    let file_path_with_env_values = self.replace_env_keys_by_value(file_path);
                    output += &format!("    let body = fs::read_to_string(\"{}\")?;\n\n", file_path_with_env_values);
                    body_str += "    .body(body)\n";
                },
                Multipart(multipart) => {
                    output += "    let form = Form::new()";

                    for key_value in multipart {
                        if !key_value.enabled {
                            continue;
                        }

                        let key = self.replace_env_keys_by_value(&key_value.data.0);
                        let value = self.replace_env_keys_by_value(&key_value.data.1);

                        if value.starts_with("!!") {
                            let file_path = &value[2..];
                            output += &format!("\n        .part(\"{}\", Part::file(\"{}\")?)", key, file_path);
                        } else {
                            output += &format!("\n        .text(\"{}\", \"{}\")", key, value);
                        }
                    }

                    output += ";\n\n";
                    body_str += "    .multipart(form)\n";
                },
                Form(form_data) => {
                    output += "    let mut form = HashMap::new();\n";

                    let form = self.key_value_vec_to_tuple_vec(form_data);
                    for (key, value) in form {
                        output += &format!("    form.insert(\"{}\", \"{}\");\n", key, value);
                    }

                    output += "\n";
                    body_str += "        .form(&form)\n";
                },
                Raw(body) => {
                    body_str += &format!("        .body(\"{}\")\n", body);
                },
                Json(body) => {
                    body_str += &format!("        .json(&{})\n", body);
                },
                Xml(body) | Html(body) | Javascript(body) => {
                    has_headers = true;
                    headers_str += &format!("            .header(\"Content-Type\", \"{}\")\n", http_request.body.to_content_type());
                    body_str += &format!("        .body(\"{}\")\n", body);
                }
            }
            Protocol::WsRequest(_) => {}
        }

        /* Request and response */
        output += &format!("    let response = client.{}(\"{}\")\n", method.to_lowercase(), url);

        if has_headers {
            output += &headers_str;
        }

        output += &body_str;

        if matches!(request.protocol, Protocol::WsRequest(_)) {
            output += "        .upgrade()\n";
        }

        output += "        .send()\n";
        output += "        .await?;\n\n";

        match request.protocol {
            Protocol::HttpRequest(_) => {
                output += "    let status = response.status();\n";
                output += "    let body = response.text().await?;\n\n";

                if request.settings.use_config_proxy.as_bool() && self.config.proxy.is_some() {
                    output += "    // Use reqwest::ClientBuilder with reqwest::Proxy\n";
                    output += "    // https://docs.rs/reqwest/latest/reqwest/struct.Proxy.html\n\n";
                }

                output += "    println!(\"Status: {}\", status);\n";
                output += "    println!(\"Body: {}\", body);\n\n";

            }
            Protocol::WsRequest(_) => {
                output += r#"    let websocket = response.into_websocket().await?;
    let (mut tx, mut rx) = websocket.split();

    futures_util::future::join(
        async move {
            tx.send(Message::Text("Hello, World!")).await.unwrap();
        },
        async move {
            while let Some(message) = rx.try_next().await.unwrap() {
                if let Message::Text(text) = message {
                    println!("received: {text}");
                }
            }
        },
    )
    .await;

"#;

            }
        }

        output += "    Ok(())\n";
        output += "}";

        output
    }
}

fn encode_basic_auth(username: &String, password: &String) -> String {
    use std::io::Write;

    let mut buf = b"Basic ".to_vec();
    {
        let mut encoder = EncoderWriter::new(&mut buf, &BASE64_STANDARD);
        write!(encoder, "{}:{}", username, password).ok();
    }

    String::from_utf8_lossy(&buf).to_string()
}
