use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::Command;

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

#[test]
fn console_flag_shows_pre_and_post_request_script_output() {
    let url = one_response_server();

    let directory = std::env::temp_dir().join(format!("atac-cli-console-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();

    let collection = format!(r#"{{
        "name": "console",
        "last_position": 0,
        "requests": [
            {{
                "name": "scripts",
                "url": "{url}",
                "params": [],
                "headers": [],
                "auth": "no_auth",
                "scripts": {{
                    "pre_request_script": "console.log('pre-request output');",
                    "post_request_script": "console.log('post-request output');"
                }},
                "settings": {{
                    "use_config_proxy": false,
                    "allow_redirects": true,
                    "timeout": 30000,
                    "store_received_cookies": true,
                    "pretty_print_response_content": true,
                    "accept_invalid_certs": false,
                    "accept_invalid_hostnames": false
                }},
                "protocol": {{ "type": "http", "method": "GET", "body": "no_body" }}
            }}
        ]
    }}"#);
    std::fs::write(directory.join("console.json"), collection).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_atac"))
        .arg("--directory").arg(&directory)
        .args(["request", "send", "console/scripts", "--console", "--hide-content", "--dry-run"])
        .output()
        .unwrap();

    std::fs::remove_dir_all(&directory).unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success(), "{stdout}{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(stdout.matches("pre-request output").count(), 1, "{stdout:?}");
    assert_eq!(stdout.matches("post-request output").count(), 1, "{stdout:?}");
}
