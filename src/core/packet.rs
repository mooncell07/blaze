use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt::Debug;
use std::io::{Cursor, Read, Result, Write};

pub const CPE: u8 = 0x42;

#[derive(Debug)]
enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
    Cobblestone = 4,
    WoodPlank = 5,
    Sapling = 6,
    Bedrock = 7,
    Water = 8,
    StationaryWater = 9,
    Lava = 10,
    StationaryLava = 11,
    Sand = 12,
    Gravel = 13,
    GoldOre = 14,
    IronOre = 15,
    CoalOre = 16,
    Wood = 17,
    Leaves = 18,
    Sponge = 19,
    Glass = 20,
    RedCloth = 21,
    OrangeCloth = 22,
    YellowCloth = 23,
    LimeCloth = 24,
    GreenCloth = 25,
    AquaGreenCloth = 26,
    CyanCloth = 27,
    BlueCloth = 28,
    PurpleCloth = 29,
    IndigoCloth = 30,
    VioletCloth = 31,
    MagentaCloth = 32,
    PinkCloth = 33,
    BlackCloth = 34,
    GrayCloth = 35,
    WhiteCloth = 36,
    YellowFlower = 37,
    RedFlower = 38,
    BrownMushroom = 39,
    RedMushroom = 40,
    GoldBlock = 41,
    IronBlock = 42,
    DoubleSlab = 43,
    Slab = 44,
    BrickBlock = 45,
    TNT = 46,
    Bookshelf = 47,
    MossyCobblestone = 48,
    Obsidian = 49,
}


#[derive(Debug)]
pub enum PacketType {
    Downstream,
    Upstream,
}

#[derive(Debug)]
pub struct BasePacket {
    pub packet_type: PacketType,
    pub packet_id: u8,
}

pub trait Serializable {
    fn build(&self) -> Result<Vec<u8>>;
}

pub trait Deserializable: Sized + Debug {
    fn build(reader: Cursor<[u8; 1024]>, base: BasePacket) -> Result<Self>;
}

fn read_next_string(reader: &mut Cursor<[u8; 1024]>) -> Result<String> {
    let mut char_buffer: [u8; 64] = [42; 64];
    reader.read_exact(&mut char_buffer)?;
    Ok(String::from_utf8_lossy(&char_buffer).to_string())
}

pub fn get_qualified_string(string: &str) -> String {
    let padding_size = 64 - string.len();
    let padding = " ".repeat(padding_size);
    string.to_owned() + &padding
}

#[derive(Debug)]
pub struct PlayerIdentificationPacket {
    pub base: BasePacket,
    pub protocol_version: u8,
    pub username: String,
    pub verification_key: String,
    pub unused: u8,
}

impl Deserializable for PlayerIdentificationPacket {
    fn build(mut reader: Cursor<[u8; 1024]>, base: BasePacket) -> Result<Self> {
        let packet = Self {
            base,
            protocol_version: reader.read_u8()?,
            username: { read_next_string(&mut reader)? },
            verification_key: { read_next_string(&mut reader)? },
            unused: reader.read_u8()?
        };

        if packet.unused == CPE {
            // cpe
        }

        Ok(packet)
    }
}

pub struct ServerIdentificationPacket {
    pub base: BasePacket,
    pub protocol_version: u8,
    pub server_name: String,
    pub server_motd: String,
    pub user_type: u8,
}

impl Serializable for ServerIdentificationPacket {
    fn build(&self) -> Result<Vec<u8>> {
        let mut writer = vec![];
        writer.write_u8(self.base.packet_id)?;
        writer.write_u8(self.protocol_version)?;
        writer.write(self.server_name.as_bytes())?;
        writer.write(self.server_motd.as_bytes())?;
        writer.write_u8(self.user_type)?;
        Ok(writer)
    }
}

impl ServerIdentificationPacket {
    pub fn new(server_name: &str, server_motd: &str, user_type: u8) -> Self {
        Self {
            base: BasePacket {
                packet_type: PacketType::Downstream,
                packet_id: 0x00,
            },
            protocol_version: 0x07,
            server_name: get_qualified_string(server_name),
            server_motd: get_qualified_string(server_motd),
            user_type,
        }
    }
}

pub struct LevelInitializePacket {
    pub base: BasePacket,
}

impl Serializable for LevelInitializePacket {
    fn build(&self) -> Result<Vec<u8>> {
        let mut writer = vec![];
        writer.write_u8(self.base.packet_id)?;
        Ok(writer)
    }
}

impl LevelInitializePacket {
    pub fn new() -> Self {
        Self {
            base: BasePacket {
                packet_type: PacketType::Downstream,
                packet_id: 0x02,
            },
        }
    }
}

pub struct PingPacket {
    pub base: BasePacket,
}

impl Serializable for PingPacket {
    fn build(&self) -> Result<Vec<u8>> {
        let mut writer = vec![];
        writer.write_u8(self.base.packet_id)?;
        Ok(writer)
    }
}

impl PingPacket {
    pub fn new() -> Self {
        Self {
            base: BasePacket {
                packet_type: PacketType::Downstream,
                packet_id: 0x01,
            },
        }
    }
}
#[derive(Debug)]
pub struct LevelDataChunkPacket{
    pub base: BasePacket,
    pub chunk_length: i16,
    pub chunk_data: Vec<u8>,
    pub percent_complete: u8
}

impl Serializable for LevelDataChunkPacket {
    fn build(&self) -> Result<Vec<u8>> {
        let mut writer = vec![];
        writer.write_u8(self.base.packet_id)?;
        writer.write_i16::<BigEndian>(self.chunk_length)?;
        writer.write_all(&self.chunk_data)?;
        writer.write_u8(self.percent_complete)?;
        Ok(writer)
    }
}

impl LevelDataChunkPacket {
    pub fn new(chunk_length: i16, chunk_data: Vec<u8>, percent_complete: u8) -> Self {
        Self {
            base: BasePacket {
                packet_type: PacketType::Downstream,
                packet_id: 0x03,
            },
            chunk_length,
            chunk_data,
            percent_complete,
        }
    }
}


pub struct LevelFinalizePacket{
    pub base: BasePacket,
    pub x: i16,
    pub y: i16,
    pub z: i16
}

impl Serializable for LevelFinalizePacket{
    fn build(&self) -> Result<Vec<u8>> {
        let mut writer = vec![];
        writer.write_u8(self.base.packet_id)?;
        writer.write_i16::<BigEndian>(self.x)?;
        writer.write_i16::<BigEndian>(self.y)?;
        writer.write_i16::<BigEndian>(self.z)?;
        Ok(writer)
    }
}

impl LevelFinalizePacket {
    pub fn new() -> Self {
        Self {
            base: BasePacket{
                packet_type: PacketType::Downstream, 
                packet_id: 0x04},
            x: 256, y:64, z: 256
        }
    }
}

#[derive(Debug)]
pub struct SpawnPlayerPacket{
    pub base: BasePacket,
    pub player_id: i8,
    pub player_name: String,
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub yaw: u8,
    pub pitch: u8
}

impl Serializable for SpawnPlayerPacket {
    fn build(&self) -> Result<Vec<u8>> {
        let mut writer = vec![];
        writer.write_u8(self.base.packet_id)?;
        writer.write_i8(self.player_id)?;
        writer.write(self.player_name.as_bytes())?;
        writer.write_i16::<BigEndian>(self.x)?;
        writer.write_i16::<BigEndian>(self.y)?;
        writer.write_i16::<BigEndian>(self.z)?;
        writer.write_u8(self.yaw)?;
        writer.write_u8(self.pitch)?;
        Ok(writer)
    }
}

impl SpawnPlayerPacket{
    pub fn new(player_name: &str) -> Self {
        Self { base: BasePacket{packet_type: PackeType::DOWNSTREAM, packet_id: 0x07}, player_id: -1, player_name: get_qualified_string(player_name), x: 256, y: 1090, z: 256, yaw: 90, pitch: 30 }
    }
}


pub enum Packet {
    PlayerIdentification(PlayerIdentificationPacket),
}
