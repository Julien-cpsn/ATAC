use std::sync::Arc;
use std::time::{Duration, Instant};
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
                error!("Sending error: {}", error);

                elapsed_time = request_start.elapsed();

                let response_status_code;

                if let Some(status_code) = error.status() {
                    response_status_code = Some(status_code.to_string());
                } else {
                    response_status_code = None;
                }

                let result_body = ResponseContent::Body(error.to_string());

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

    let (modified_response, post_request_output) = App::handle_post_request_script(&request, response, env);

    drop(request);

    {
        let mut request = local_request.write();

        request.console_output.post_request_output = post_request_output;
        request.is_pending = false;
        request.cancellation_token = CancellationToken::new();
    }
        
    return modified_response;
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::Arc;
    use parking_lot::RwLock;
    use crate::app::business_logic::request::http::send::send_http_request;
    use crate::app::business_logic::request::send::RequestResponseError;
    use crate::models::request::Request;

    /// Answers a single request with a small 200 response, then closes the connection
    fn one_response_server() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();

        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buffer = [0; 4096];
            let mut received = Vec::new();

            while !received.windows(4).any(|window| window == b"\r\n\r\n") {
                let read = stream.read(&mut buffer).unwrap();

                if read == 0 {
                    break;
                }

                received.extend_from_slice(&buffer[..read]);
            }

            stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok").unwrap();
        });

        format!("http://{address}/")
    }

    #[tokio::test]
    async fn failing_post_request_script_does_not_leave_the_request_pending() {
        let url = one_response_server();

        let mut request = Request {
            url: url.clone(),
            ..Default::default()
        };
        request.scripts.post_request_script = Some(String::from(r#"throw new Error("post-request script failure");"#));

        let local_request = Arc::new(RwLock::new(request));
        let prepared_request = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build().get(&url);

        let response = send_http_request(prepared_request, local_request.clone(), &None).await;

        assert!(matches!(response, Err(RequestResponseError::PostRequestScript)));

        let request = local_request.read();
        assert!(!request.is_pending);

        let console_output = request.console_output.post_request_output.as_deref().unwrap_or_default();
        assert!(console_output.contains("post-request script failure"), "{console_output:?}");
    }
}
