pub struct PacketBase{
    pub protocol_id: u8
}

pub trait Packet {
    fn serialize(&self) -> [u8; 1024];
    fn deserialize(data: &[u8; 1024]) -> Self;
}

pub struct PlayerIdentificationPacket{
    pub base: PacketBase,
    pub protocol_version: u8,
    pub username: String,
    pub verification_key: String,
}

impl Packet for PlayerIdentificationPacket {
    fn deserialize(data: &[u8; 1024]) -> Self {
        let protocol_id = u8::from_le_bytes([data[0]]);
        let protocol_version = u8::from_le_bytes([data[1]]);
        let username = String::from_utf8_lossy(&data[2..66]).to_string();
        let verification_key = String::from_utf8_lossy(&data[66..120]).to_string();

        Self {base: PacketBase{protocol_id}, protocol_version, username, verification_key}
    }

    fn serialize(&self) -> [u8; 1024] {
        
        [0; 1024]
    }
}