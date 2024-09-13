//Modulos
mod cookies;
// Librerias
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use cookies::{send_cookie_response, read_cookies};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    // Leer las cookies enviadas por el cliente
    let session_cookie = read_cookies(&request);

    match session_cookie {
        Some(session_id) => {
            // Si la cookie existe, reutilizar la sesión
            println!("Sesión activa: {}", session_id);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: 23\r\n\r\nWelcome back, session {}!",
                session_id
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        None => {
            // Si no hay cookie, enviar una nueva
            println!("Iniciando nueva sesión");
            send_cookie_response(&mut stream);  // Solo genera una nueva cookie si no existe
        }
    }

    // Operaciones HTTP

    let get = b"GET / HTTP/1.1\r\n";
    let post = b"POST / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = "Hello, this is a GET request!";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(post) {
        let contents = "Hello, this is a POST request!";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
        let response = format!("{}{}", status_line, "405 Method Not Allowed");

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
