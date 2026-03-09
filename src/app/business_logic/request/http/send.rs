use std::sync::Arc;
use std::time::{Duration, Instant};
use std::error::Error;
use parking_lot::RwLock;

use rayon::prelude::*;
use reqwest::header::{CONTENT_TYPE};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, trace};
use crate::app::app::App;
use crate::app::business_logic::request::send::RequestResponseError;
use crate::app::business_logic::request::send::RequestResponseError::CouldNotDecodeResponse;
use crate::models::protocol::http::body::find_file_format_in_content_type;
use crate::models::environment::Environment;
use crate::models::request::Request;
use crate::models::response::{ImageResponse, RequestResponse, ResponseContent};


/// Analyzes a reqwest error and returns a more informative error message
fn format_error_message(error: &reqwest_middleware::Error) -> String {
    let error_string = error.to_string();
    
    // Check the error chain for more specific error types
    let mut source_chain = String::new();
    let mut current_source = error.source();
    while let Some(source) = current_source {
        let source_str = source.to_string();
        source_chain.push_str(&source_str);
        source_chain.push_str("\n");
        
        // Check for certificate errors in the chain
        if source_str.contains("certificate") || source_str.contains("Certificate") || 
           source_str.contains("UnknownIssuer") || source_str.contains("CertNotValidForName") ||
           source_str.contains("self-signed") || source_str.contains("TLS") || 
           source_str.contains("SSL") {
            
            if source_str.contains("UnknownIssuer") || source_str.contains("self-signed") || 
               (source_str.contains("certificate") && source_str.contains("invalid")) {
                return format!(
                    "{}\n\n💡 TLS Certificate Error: The server is using a self-signed or untrusted certificate.\n   Solution: Use --accept-invalid-certs flag or enable 'Accept invalid certs' in TUI settings",
                    error_string
                );
            }
            if source_str.contains("CertNotValidForName") || source_str.contains("hostname") {
                return format!(
                    "{}\n\n💡 TLS Hostname Mismatch: The certificate hostname does not match the server.\n   Solution: Use --accept-invalid-hostnames flag or enable 'Accept invalid hostnames' in TUI settings",
                    error_string
                );
            }
        }
        
        current_source = source.source();
    }
    
    // Also check the main error string for certificate-related keywords
    let error_lower = error_string.to_lowercase();
    if error_lower.contains("certificate") || error_lower.contains("tls") || error_lower.contains("ssl") {
        if error_lower.contains("unknownissuer") || error_lower.contains("self-signed") {
            return format!(
                "{}\n\n💡 TLS Certificate Error: The server is using a self-signed or untrusted certificate.\n   Solution: Use --accept-invalid-certs flag or enable 'Accept invalid certs' in TUI settings",
                error_string
            );
        }
        return format!(
            "{}\n\n💡 TLS Certificate Error: There is an issue with the SSL/TLS certificate.\n   Solution: Try using --accept-invalid-certs and/or --accept-invalid-hostnames flags",
            error_string
        );
    }
    
    if error.is_timeout() {
        return format!(
            "{}\n\n💡 Timeout Error: The request took too long to complete.\n   Solution: Increase the timeout value (current: check request settings)",
            error_string
        );
    }
    
    if error.is_connect() {
        if error_string.contains("Connection refused") {
            return format!(
                "{}\n\n💡 Connection Refused: The server is not accepting connections.\n   Solution: Check if the server is running and the port is correct",
                error_string
            );
        }
        if error_string.contains("No route to host") || error_string.contains("Network is unreachable") {
            return format!(
                "{}\n\n💡 Network Error: Cannot reach the server.\n   Solution: Check your network connection and the URL",
                error_string
            );
        }
        // For other connect errors, check if it might be certificate-related
        if error_lower.contains("invalid") || error_lower.contains("failed") {
            return format!(
                "{}\n\n💡 Connection Error: Could not establish a secure connection to the server.\n   This might be due to:\n   - Invalid SSL/TLS certificate (try --accept-invalid-certs)\n   - Certificate hostname mismatch (try --accept-invalid-hostnames)\n   - Network connectivity issues\n   - Server not running or wrong port",
                error_string
            );
        }
        return format!(
            "{}\n\n💡 Connection Error: Could not establish a connection to the server.\n   Solution: Check the URL, network connectivity, and server availability",
            error_string
        );
    }
    
    if error.is_redirect() {
        return format!(
            "{}\n\n💡 Redirect Error: Too many redirects or redirect issue.\n   Solution: Enable redirects in request settings or check the redirect chain",
            error_string
        );
    }
    
    if error.is_request() {
        return format!(
            "{}\n\n💡 Request Error: The request could not be constructed.\n   Solution: Check the URL format and request parameters",
            error_string
        );
    }
    
    if error.is_body() {
        return format!(
            "{}\n\n💡 Body Error: There was an issue with the request body.\n   Solution: Check the request body format and content",
            error_string
        );
    }
    
    if error.is_decode() {
        return format!(
            "{}\n\n💡 Decode Error: Could not decode the response body.\n   Solution: The response may be in an unexpected format",
            error_string
        );
    }
    
    // Default: return the original error with a generic hint
    format!(
        "{}\n\n💡 Tip: Use --verbose flag for more details or check request settings",
        error_string
    )
}


pub async fn send_http_request(prepared_request: reqwest_middleware::RequestBuilder, local_request: Arc<RwLock<Request>>, env: &Option<Arc<RwLock<Environment>>>) -> Result<RequestResponse, RequestResponseError> {
    info!("Sending request");

    local_request.write().is_pending = true;

    let request = local_request.read();

    let cancellation_token = request.cancellation_token.clone();
    let timeout = tokio::time::sleep(Duration::from_millis(request.settings.timeout.as_u32() as u64));

    let request_start = Instant::now();
    let elapsed_time: Duration;

    let mut response = tokio::select! {
        _ = cancellation_token.cancelled() => {
            elapsed_time = request_start.elapsed();
            
            RequestResponse {
                duration: None,
                status_code: Some(String::from("CANCELED")),
                content: None,
                cookies: None,
                headers: vec![]
            }
        },
        _ = timeout => {
            elapsed_time = request_start.elapsed();

            RequestResponse {
                duration: None,
                status_code: Some(String::from("TIMEOUT")),
                content: None,
                cookies: None,
                headers: vec![]
            }
        },
        response = prepared_request.send() => match response {
            Ok(response) => {
                info!("Response received");

                elapsed_time = request_start.elapsed();

                let status_code = response.status().to_string();

                let mut is_image = false;

                let headers: Vec<(String, String)> = response.headers().clone()
                    .iter()
                    .map(|(header_name, header_value)| {
                        let value = header_value.to_str().unwrap_or("").to_string();

                        if header_name == CONTENT_TYPE && value.starts_with("image/") {
                            is_image = true;
                        }

                        (header_name.to_string(), value)
                    })
                    .collect();

                let cookies = response.cookies()
                    .par_bridge()
                    .map(|cookie| {
                        format!("{}: {}", cookie.name(), cookie.value())
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                let response_content = match is_image {
                    true => {
                        let content = response.bytes().await.unwrap();
                        let image = image::load_from_memory(content.as_ref());

                        ResponseContent::Image(ImageResponse {
                            data: content.to_vec(),
                            image: image.ok(),
                        })
                    },
                    false => match response.bytes().await {
                        Ok(bytes) => match String::from_utf8(bytes.to_vec()) {
                            Ok(mut result_body) => {
                                // If a file format has been found in the content-type header
                                if let Some(file_format) = find_file_format_in_content_type(&headers) {
                                    // If the request response content can be pretty printed
                                    if request.settings.pretty_print_response_content.as_bool() {
                                        // Match the file format
                                        match file_format.as_str() {
                                            "json" => {
                                                result_body = jsonxf::pretty_print(&result_body).unwrap_or(result_body);
                                            },
                                            _ => {}
                                        }
                                    }
                                }
    
                                ResponseContent::Body(result_body)
                            },
                            Err(_) => ResponseContent::Body(format!("{:#X?}", bytes))
                        },
                        Err(_) => return Err(CouldNotDecodeResponse)
                    }
                };

                RequestResponse {
                    duration: None,
                    status_code: Some(status_code),
                    content: Some(response_content),
                    cookies: Some(cookies),
                    headers
                }
            },
            Err(error) => {
                let formatted_error = format_error_message(&error);
                error!("Sending error: {}", formatted_error.replace('\n', " | "));

                elapsed_time = request_start.elapsed();

                let response_status_code;

                if let Some(status_code) = error.status() {
                    response_status_code = Some(status_code.to_string());
                } else {
                    response_status_code = None;
                }

                let result_body = ResponseContent::Body(formatted_error);

                RequestResponse {
                    duration: None,
                    status_code: response_status_code,
                    content: Some(result_body),
                    cookies: None,
                    headers: vec![]
                }
            }
        }
    };

    response.duration = Some(format!("{:?}", elapsed_time));

    trace!("Request sent");

    /* POST-REQUEST SCRIPT */

    let (modified_response, post_request_output) = App::handle_post_request_script(&request, response, env)?;

    drop(request);

    {
        let mut request = local_request.write();

        request.console_output.post_request_output = post_request_output;
        request.is_pending = false;
        request.cancellation_token = CancellationToken::new();
    }
        
    return Ok(modified_response);
}
