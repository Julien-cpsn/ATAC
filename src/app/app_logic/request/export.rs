use std::path::PathBuf;
use arboard::Clipboard;
use base64::prelude::BASE64_STANDARD;
use base64::write::EncoderWriter;
use reqwest::Url;
use strum::VariantArray;

use crate::app::app::App;
use crate::app::app_logic::request::send::get_file_content_with_name;
use crate::request::auth::Auth;
use crate::request::body::ContentType::{File, Form, Html, Javascript, Json, Multipart, NoBody, Raw, Xml};
use crate::request::export::ExportFormat;
use crate::request::export::ExportFormat::{Curl, HTTP, NodeJsAxios, PhpGuzzle, RustReqwest};
use crate::request::request::Request;

impl App<'_> {
    pub fn export_request(&mut self) {
        let export_format = &ExportFormat::VARIANTS[self.export_request.selection];

        let local_selected_request = self.get_selected_request_as_local();

        {
            let selected_request = local_selected_request.read().unwrap();

            let export_result = self.export_request_to_string_with_format(export_format, &selected_request);


            self.display_request_export.vertical_scrollbar.set_scroll(export_result.lines().count());
            self.display_request_export.horizontal_scrollbar.set_scroll(App::get_max_str_len(export_result.lines()));

            self.display_request_export.content = export_result;
            self.display_request_export.title = export_format.to_string();
        }

        self.display_request_export_state();
    }

    fn export_request_to_string_with_format(&mut self, export_format: &ExportFormat, request: &Request) -> String {
        let mut output = String::new();

        let params = self.key_value_vec_to_tuple_vec(&request.params);
        let url = self.replace_env_keys_by_value(&request.url);

        let url = match Url::parse_with_params(&url, &params) {
            Ok(url) => url,
            Err(_) => {
                return String::from("Could not parse URL");
            }
        };

        let headers = self.key_value_vec_to_tuple_vec(&request.headers);

        match export_format {
            HTTP => {
                /* URL & Query params */

                output += &format!("{} {}{} HTTP/1.1",
                    request.method.to_string(),
                    url.path(),
                    match url.query() {
                        None => String::new(),
                        Some(query) => format!("?{query}")
                    }
                );

                output += &format!("\nHost: {}", url.host_str().unwrap());

                /* Headers */

                for (header, value) in &headers {
                    output += &format!("\n{}: {}", header, value);
                }

                /* Auth */

                output += &match &request.auth {
                    Auth::NoAuth => String::new(),
                    Auth::BasicAuth(username, password) => {
                        let username = self.replace_env_keys_by_value(username);
                        let password = self.replace_env_keys_by_value(password);

                        format!("\nAuthorization: {}", encode_basic_auth(&username, &password))
                    },
                    Auth::BearerToken(bearer_token) => {
                        let bearer_token = self.replace_env_keys_by_value(bearer_token);

                        format!("\nAuthorization: Bearer {}", bearer_token)
                    }
                };

                /* Body */

                output += &match &request.body {
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
                        let mut multipart_output = format!("\nContent-Type: {}; boundary={}\n", &request.body.to_content_type(), boundary);

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

                                value = format!("; filename=\"{file_name}\"\nContent-Type: {}\n\n{}", &request.body.to_content_type(), String::from_utf8_lossy(&file_content));
                            }
                            else {
                                value = format!("\n\n{}", &value);
                            }

                            multipart_output += &value;
                        }

                        multipart_output += &format!("\n--{boundary}");

                        multipart_output
                    }
                    Form(form_data) => {
                        let mut form_output = format!("\n--header 'Content-Type: {}' \\", &request.body.to_content_type());

                        let form = self.key_value_vec_to_tuple_vec(form_data);

                        for (key, value) in form {

                            form_output += &format!("\n--data-urlencode '{}={}' \\", key, value);
                        }

                        form_output
                    }
                    Raw(body) | Json(body) | Xml(body) | Html(body) | Javascript(body) => {
                        format!("\n--header 'Content-Type: {}' \\\n--data '{}' \\", &request.body.to_content_type(), body)
                    }
                };

                output
            },
            Curl => {
                /* URL & Query params */

                output += &format!("curl --location --request {} '{}' \\",
                   request.method.to_string(),
                   url
                );

                /* Auth */

                output += &match &request.auth {
                    Auth::NoAuth => String::new(),
                    Auth::BasicAuth(username, password) => {
                        let username = self.replace_env_keys_by_value(username);
                        let password = self.replace_env_keys_by_value(password);

                        format!("\n--header '{}' \\", encode_basic_auth(&username, &password))
                    },
                    Auth::BearerToken(bearer_token) => {
                        let bearer_token = self.replace_env_keys_by_value(bearer_token);

                        format!("\n--header 'Authorization: Bearer {}' \\", bearer_token)
                    }
                };

                /* Headers */

                for (header, value) in &headers {
                    output += &format!("\n--header '{}: {}' \\", header, value);
                }

                /* Body */

                output += &match &request.body {
                    NoBody => String::new(),
                    File(file_path) => {
                        let file_path_with_env_values = self.replace_env_keys_by_value(file_path);
                        
                        format!("\n--header 'Content-Type: {}' \\\n--data '@/{}' \\", &request.body.to_content_type(), file_path_with_env_values)
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
                        let mut form_output = format!("\n--header 'Content-Type: {}' \\", &request.body.to_content_type());

                        let form = self.key_value_vec_to_tuple_vec(form_data);

                        for (key, value) in form {

                            form_output += &format!("\n--data-urlencode '{}={}' \\", key, value);
                        }

                        form_output
                    }
                    Raw(body) | Json(body) | Xml(body) | Html(body) | Javascript(body) => {
                        format!("\n--header 'Content-Type: {}' \\\n--data '{}' \\", &request.body.to_content_type(), body)
                    }
                };
                
                if request.settings.use_config_proxy {
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
            },
            PhpGuzzle => {

                output
            },
            NodeJsAxios => {

                output
            },
            RustReqwest => {

                output
            },
        }
    }
    
    pub fn copy_request_export_to_clipboard(&mut self) {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(&self.display_request_export.content).expect("Could not copy request export to clipboard")
    }
}

fn encode_basic_auth(username: &String, password: &String) -> String {
    use std::io::Write;

    let mut buf = b"Basic ".to_vec();
    {
        let mut encoder = EncoderWriter::new(&mut buf, &BASE64_STANDARD);
        write!(encoder, "{username}").ok();
        write!(encoder, "{password}").ok();
    }

    return String::from_utf8_lossy(&buf).to_string();
}