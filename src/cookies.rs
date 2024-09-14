use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;
use rand::Rng;

/// Parse the incoming request headers and extract the session cookie, if present.
pub fn get_session_cookie(request: &str) -> Option<String> {
    for line in request.lines() {
        if line.starts_with("Cookie:") {
            let cookies = line.replace("Cookie: ", "");
            for cookie in cookies.split(';') {
                let parts: Vec<&str> = cookie.trim().split('=').collect();
                if parts.len() == 2 && parts[0] == "session_id" {
                    return Some(parts[1].to_string());
                }
            }
        }
    }
    None
}

/// Generate a unique session ID.
pub fn generate_session_id() -> String {
    let mut rng = rand::thread_rng();
    let random_number: u64 = rng.gen();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    format!("{}-{}", random_number, current_time)
}

/// Set a new session cookie in the response headers.
pub fn set_session_cookie(stream: &mut TcpStream, session_id: &str) {
    let set_cookie_header = format!(
        "Set-Cookie: session_id={}; Path=/; HttpOnly\r\n",
        session_id
    );
    let response_headers = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n{}\r\n", set_cookie_header);
    stream.write(response_headers.as_bytes()).unwrap();
}
