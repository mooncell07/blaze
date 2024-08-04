use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fmt::Debug;
use std::io::{Cursor, Read, Result, Write};

#[derive(Debug)]
pub enum PackeType {
    DOWNSTREAM,
    UPSTREAM,
}
#[derive(Debug)]
pub struct BasePacket {
    pub packet_type: PackeType,
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

#[derive(Debug)]
pub struct PlayerIdentificationPacket {
    pub base: BasePacket,
    pub protocol_version: u8,
    pub username: String,
    pub verification_key: String,
}

impl Deserializable for PlayerIdentificationPacket {
    fn build(mut reader: Cursor<[u8; 1024]>, base: BasePacket) -> Result<Self> {
        Ok(Self {
            base,
            protocol_version: reader.read_u8()?,
            username: { read_next_string(&mut reader)? },
            verification_key: { read_next_string(&mut reader)? },
        })
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

pub enum Packet {
    PlayerIdentification(PlayerIdentificationPacket),
}
