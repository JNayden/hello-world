use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::Arc;

// A simple function to handle each client connection
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Basic response
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello world!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    // Create a TCP listener bound to localhost on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    // Wrap the listener in an Arc for safe multithreaded access
    let listener = Arc::new(listener);

    // Accept incoming connections and spawn a new thread for each
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        // Spawn a thread to handle the connection
        let listener = Arc::clone(&listener);
        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}
