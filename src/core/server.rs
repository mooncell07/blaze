use super::{
    packet::{
        LevelInitializePacket, Packet, Serializable, ServerIdentificationPacket
    },
    response::Response,
};
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

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

    fn send_packet(&self, packet: &impl Serializable, mut stream: &TcpStream) {
        let serialized_packet = &packet.build().expect("Couldn't build packet.");
        stream
            .write(serialized_packet)
            .expect("Couldn't send packet.");
    }

    fn run_player_handshake(&self, stream: &TcpStream) -> i32 {
        let server_identification_packet = ServerIdentificationPacket::new("woo", "idk", 0x64);
        self.send_packet(&server_identification_packet, stream);
        0
    }

    fn generate_world(&self, stream: &TcpStream){
        let level_initialize_packet = LevelInitializePacket::new();
        self.send_packet(&level_initialize_packet, stream);
    }

    pub fn run(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let resp = Response::from_stream(&mut stream).to_packet();

                    match resp {
                        Packet::PlayerIdentification(_) => {
                            let status = self.run_player_handshake(&stream);
                            if status == 0 {
                                self.generate_world(&stream);
                                self.clients.push(stream);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to connect with the client: {}", e);
                }
            }
        }
    }
}
