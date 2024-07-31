use super::response::Response;
use std::net::{TcpListener, TcpStream};

pub struct Server {
    pub listener: TcpListener,
    pub clients: Vec<TcpStream>,
}

impl Server {
    pub fn new() -> Self {
        let listener =
            TcpListener::bind("127.0.0.1:25565").expect("Failed to initialize the listener.");
        Self {
            listener,
            clients: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let _ = Response::from_stream(&mut stream);
                    self.clients.push(stream);
                }
                Err(e) => {
                    eprintln!("Failed to connect with the client: {}", e);
                }
            }
        }
    }
}
