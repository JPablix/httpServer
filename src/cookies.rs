use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;
use rand::Rng;

// Parsea los headers de la solicitud entrante y extrae la cookie de sesión, si está presente.
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

// Genera un ID de sesión único.
pub fn generate_session_id() -> String {
    let mut rng = rand::thread_rng();
    let random_number: u64 = rng.gen();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    format!("{}-{}", random_number, current_time)
}

// Establece una nueva cookie de sesión en los headers de la respuesta.
pub fn set_session_cookie(stream: &mut TcpStream, session_id: &str) {
    let set_cookie_header = format!(
        "Set-Cookie: session_id={}; Path=/; HttpOnly\r\n",
        session_id
    );
    let response_headers = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n{}\r\n", set_cookie_header);
    stream.write(response_headers.as_bytes()).unwrap();
}

use std::io::Read;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_session_cookie() {
        let request = "GET / HTTP/1.1\r\nCookie: session_id=abc123; other_cookie=xyz456\r\n\r\n";
        let cookie = get_session_cookie(request);
        assert_eq!(cookie, Some("abc123".to_string()));
    }

    #[test]
    fn test_get_session_cookie_not_found() {
        let request = "GET / HTTP/1.1\r\nCookie: other_cookie=xyz456\r\n\r\n";
        let cookie = get_session_cookie(request);
        assert_eq!(cookie, None);
    }

    #[test]
    fn test_generate_session_id() {
        let session_id = generate_session_id();
        // Verificamos que el ID tenga el formato esperado: un número seguido de un guion y un timestamp
        let parts: Vec<&str> = session_id.split('-').collect();
        assert_eq!(parts.len(), 2);
        assert!(parts[0].parse::<u64>().is_ok()); // El primer valor debe ser un número
        assert!(parts[1].parse::<u64>().is_ok()); // El segundo valor debe ser un timestamp
    }

    #[test]
    fn test_set_session_cookie() {
        // Creamos un TcpListener temporal para obtener un TcpStream para las pruebas
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap(); // Asignamos un puerto libre
        let listener_addr = listener.local_addr().unwrap(); // Guardamos la dirección del listener para usarla en el cliente
    
        let handle = std::thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let session_id = "abc123";
                set_session_cookie(&mut stream, session_id); // Llamamos a la función con el TcpStream
            }
        });
    
        // Nos conectamos al servidor desde la misma prueba para obtener el TcpStream
        let mut client_stream = std::net::TcpStream::connect(listener_addr).unwrap();
        
        handle.join().unwrap(); // Esperamos a que el hilo termine
    
        // Leemos el resultado del stream del lado del cliente para verificar si se envió correctamente
        let mut buffer = Vec::new();
        client_stream.read_to_end(&mut buffer).unwrap(); // Ahora puedes usar read_to_end
        let result = String::from_utf8(buffer).unwrap();
    
        assert!(result.contains("Set-Cookie: session_id=abc123"));
        assert!(result.contains("HTTP/1.1 200 OK"));
    }
}
