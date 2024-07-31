use byteorder::{BigEndian, ReadBytesExt};
use std::fmt::Debug;
use std::io::{Read, Result};
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
    fn build(&self) -> &[u8; 1024];
}

pub trait Deserializable: Sized + Debug {
    fn build(reader: impl Read, base: BasePacket) -> Result<Self>;
}

#[derive(Debug)]
pub struct PlayerIdentificationPacket {
    pub base: BasePacket,
    pub protocol_version: u8,
    pub username: u64,
    pub verification_key: u64,
}

impl Deserializable for PlayerIdentificationPacket {
    fn build(mut reader: impl Read, base: BasePacket) -> Result<Self> {
        Ok(Self {
            base: base,
            protocol_version: reader.read_u8()?,
            username: reader.read_u64::<BigEndian>()?,
            verification_key: reader.read_u64::<BigEndian>()?,
        })
    }
}
