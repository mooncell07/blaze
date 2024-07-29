use std::{io::Read, net::TcpStream};

use super::packet::{self};
use crate::core::packet::Packet;

pub struct Response {
    pub data: [u8; 1024]
}

impl Response{
    pub fn from_stream(stream: &mut TcpStream) -> Self {
        let mut data = [0; 1024];
        stream.read(&mut data).expect("Could't read stream data.");
        Self { data }
    }

    pub fn to_packet(&self) -> impl Packet{
        packet::PlayerIdentificationPacket::deserialize(&self.data)
    }

}