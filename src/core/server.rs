use super::{
    packet::{
        LevelFinalizePacket, LevelInitializePacket, Packet, Serializable,
        ServerIdentificationPacket,
    },
    response::Response,
    world::World,
};
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

pub const BRAND: &str = "Blaze";

pub struct Server {
    pub listener: TcpListener,
    pub name: String,
    pub motd: String,
    pub clients: Vec<TcpStream>,
    pub world: World,
}

impl Server {
    pub fn new(name: String, motd: String) -> Self {
        let listener =
            TcpListener::bind("127.0.0.1:25565").expect("Failed to initialize the listener.");
        let world = World::from_file("/home/mooncell07/dev/blaze/main.gz");
        Self {
            listener,
            name,
            motd,
            clients: Vec::new(),
            world,
        }
    }

    fn send_packet(&self, packet: &impl Serializable, mut stream: &TcpStream) {
        let serialized_packet = &packet.build().expect("Couldn't build packet.");
        let _ = stream
            .write(serialized_packet)
            .expect("Couldn't send packet.");
    }

    fn run_player_handshake(&self, stream: &TcpStream){
        let server_identification_packet = ServerIdentificationPacket::new(&self.name, &self.motd, 0x64);
        self.send_packet(&server_identification_packet, stream);
    }

    fn generate_world(&self, stream: &TcpStream) {
        let level_initialize_packet = LevelInitializePacket::new();
        self.send_packet(&level_initialize_packet, stream);
        for packet in self.world.packets.as_slice() {
            self.send_packet(packet, stream);
        }
        let level_finalize_packet = LevelFinalizePacket::new();
        self.send_packet(&level_finalize_packet, stream);
    }

    pub fn run(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let resp = Response::from_stream(&mut stream).to_packet();

                    match resp {
                        Packet::PlayerIdentification(_) => {
                            self.run_player_handshake(&stream);
                            self.generate_world(&stream);
                            self.clients.push(stream);
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
