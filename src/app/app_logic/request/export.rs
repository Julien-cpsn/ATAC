use arboard::Clipboard;
use base64::prelude::BASE64_STANDARD;
use base64::write::EncoderWriter;
use reqwest::Url;
use strum::VariantArray;

use crate::app::app::App;
use crate::request::auth::Auth;
use crate::request::export::ExportFormat;
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


        match export_format {
            ExportFormat::Raw => {
                output += &format!("{} {} HTTP/1.1",
                    request.method.to_string(),
                    url.path(),
                );

                output += &format!("\nHost: {}", url.host_str().unwrap());

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

                output
            },
            ExportFormat::Curl => {
                output += &format!("curl --location --request {} '{}'",
                   request.method.to_string(),
                   url
                );

                output += &match &request.auth {
                    Auth::NoAuth => String::new(),
                    Auth::BasicAuth(username, password) => {
                        let username = self.replace_env_keys_by_value(username);
                        let password = self.replace_env_keys_by_value(password);

                        format!("\n--header '{}'", encode_basic_auth(&username, &password))
                    },
                    Auth::BearerToken(bearer_token) => {
                        let bearer_token = self.replace_env_keys_by_value(bearer_token);

                        format!("\n--header 'Authorization: Bearer {}'", bearer_token)
                    }
                };

                output
            },
            ExportFormat::PhpGuzzle => {

                output
            },
            ExportFormat::NodeJsAxios => {

                output
            },
            ExportFormat::RustReqwest => {

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