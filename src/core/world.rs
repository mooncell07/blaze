use std::{fs::File, io::Read};

use super::packet::LevelDataChunkPacket;

pub struct World {
    pub packets: Vec<LevelDataChunkPacket>
}

impl World{
    pub fn from_file(path: &str)-> Self { 
        let mut file = File::open(path).expect("Couldn't locate the world file.");
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).expect("Couldn't read the world file.");
        let packets = Self::generate_packets(&buffer);
        Self{packets}
    }
    fn generate_packets(buffer: &Vec<u8>) -> Vec<LevelDataChunkPacket>{
        let chunks = buffer.chunks(1024).map(|chunk| {
            if chunk.len() < 1024 {
                let mut padded_chunk = chunk.to_vec();
                padded_chunk.resize(1024, 0x0);
                padded_chunk
        }else{
            chunk.to_vec()
        }});
        let total_chunks = chunks.len() as f32;
        let mut packets: Vec<LevelDataChunkPacket> = vec![];
        for (i, chunk) in chunks.enumerate(){
            let packet = LevelDataChunkPacket::new(1024, chunk, (((i as f32 + 1.0)/total_chunks) * 100.0) as u8);
            packets.push(packet);
        }

        packets
    }
}