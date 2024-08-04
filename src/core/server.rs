use super::{
    packet::{
        BasePacket, PackeType, Packet, Serializable,
        ServerIdentificationPacket,
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
    pub server_identification_packet: ServerIdentificationPacket,
}

impl Server {
    pub fn new() -> Self {
        let listener =
            TcpListener::bind("127.0.0.1:25565").expect("Failed to initialize the listener.");
        Self {
            listener,
            clients: Vec::new(),
            server_identification_packet: ServerIdentificationPacket {
                base: BasePacket {
                    packet_type: PackeType::DOWNSTREAM,
                    packet_id: 0x00,
                },
                protocol_version: 0x07,
                server_name: "wooo".to_string(),
                server_motd: "idk".to_string(),
                user_type: 0x64,
            },
        }
    }

    fn send_packet(&self, packet: &impl Serializable, mut stream: &TcpStream) {
        let serialized_packet = &packet.build().expect("Couldn't build packet.");
        stream
            .write(serialized_packet)
            .expect("Couldn't send packet.");
    }

    fn run_player_handshake(&self, stream: &TcpStream) -> i32 {
        self.send_packet(&self.server_identification_packet, stream);
        0
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
