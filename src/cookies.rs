use std::net::TcpStream;
use std::io::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

// Función para generar un ID de sesión aleatorio
fn generate_session_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16) // Generar un ID de 16 caracteres
        .map(char::from)
        .collect()
}

// Función para enviar una cookie de sesión
pub fn send_cookie_response(stream: &mut TcpStream) {
    let session_id = generate_session_id(); // Genera un ID de sesión dinámico
    let cookie = format!("Set-Cookie: sessionId={}; Path=/; HttpOnly", session_id);

    let response = format!(
        "HTTP/1.1 200 OK\r\n{}\r\nContent-Length: 13\r\n\r\nHello, World!",
        cookie
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// Función para leer las cookies enviadas por el cliente
pub fn read_cookies(request: &str) -> Option<String> {
    if let Some(cookie_start) = request.find("Cookie:") {
        let cookies_str = &request[cookie_start..];
        let cookie_end = cookies_str.find("\r\n").unwrap_or(cookies_str.len());
        let cookies = &cookies_str[..cookie_end];

        // Buscar la cookie "sessionId"
        for cookie in cookies.split("; ") {
            if cookie.starts_with("sessionId=") {
                return Some(cookie["sessionId=".len()..].to_string());
            }
        }
    }
    None
}
