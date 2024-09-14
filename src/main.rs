use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

mod cookies;
mod thread_pool;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    
    // Check if the request contains a session cookie
    let response = if let Some(session_cookie) = cookies::get_session_cookie(&request) {
        println!("Client with session cookie: {}", session_cookie); // Depuración
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\nHello, session: {}!",
            session_cookie
        )
    } else {
        // No session cookie, generate a new one and send it back
        let session_id = cookies::generate_session_id();
        println!("Generating new session id: {}", session_id); // Depuración
        cookies::set_session_cookie(&mut stream, &session_id);
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\nHello, new session: {}!",
            session_id
        )
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = thread_pool::ThreadPool::new(4); // Tamaño del pool de threads

    println!("Listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established!"); // Depuración
                pool.execute(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
