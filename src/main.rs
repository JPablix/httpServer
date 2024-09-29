use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufRead};
use std::sync::Arc;
use std::time::Duration;


mod cookies;
mod thread_pool;
mod person;

use person::{Database, create_database};

fn handle_client(mut stream: TcpStream, db: person::Database) {
    stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();  // Timeout de 10 segundos

    let mut reader = BufReader::new(&stream);
    let mut request = String::new();

    // Leer los headers
    while let Ok(bytes_read) = reader.read_line(&mut request) {
        if bytes_read == 0 || request.ends_with("\r\n\r\n") {
            break;  // Terminamos de leer los headers
        }
    }

    if request.is_empty() {
        eprintln!("No request data found.");
        return;
    }

    println!("Request headers: {}", request);  // Depuración: Ver los headers recibidos

    // Encontrar el método de la solicitud
    let method = request.lines().next().unwrap_or("").split_whitespace().next().unwrap_or("");

    // Extraer el valor de Content-Length para solicitudes PATCH/POST/PUT
    let content_length = request.lines()
        .find(|line| line.starts_with("Content-Length:"))
        .and_then(|line| line.split(": ").nth(1))
        .and_then(|len| len.trim().parse::<usize>().ok())
        .unwrap_or(0);

    println!("Content-Length (parsed): {}", content_length);  // Depuración: Verificar si se detecta Content-Length

    let mut body = String::new();

    if content_length > 0 {
        let mut buffer = vec![0; content_length];
        reader.read_exact(&mut buffer).unwrap();  // Leer exactamente el número de bytes esperados
        body = String::from_utf8_lossy(&buffer).to_string();
        println!("Body received: {}", body);  // Depuración: Ver el cuerpo recibido
    }

    let response = match method {
        "POST" => person::handle_post(&body, &db),              // Post crea una nueva persona
        "GET" => person::handle_get(&request, &db),             // Get busca la key en la URL y devuelve el valor
        "PUT" => person::handle_put(&request, &body, &db),      // Put busca la key en la URL y reemplaza el valor por el body
        "PATCH" => person::handle_patch(&request, &body, &db),  // Patch busca la key en el body y actualiza el valor
        "DELETE" => person::handle_delete(&request, &db),       // Delete busca la key en la URL y elimina la entrada
        _ => "HTTP/1.1 405 Method Not Allowed\r\n\r\n".to_string(),
    };

    // Enviar respuesta
    if let Err(e) = stream.write(response.as_bytes()) {
        eprintln!("Failed to send response: {}", e);
    }
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush stream: {}", e);
    }
}



fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let db = create_database();

    println!("Listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db = Arc::clone(&db);
                std::thread::spawn(move || {
                    handle_client(stream, db);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
