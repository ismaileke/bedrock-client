use crate::utils::address::InternetAddress;
use crate::utils::color_format::COLOR_WHITE;
use crate::utils::{address, color_format};
use crate::protocol::raknet::packet_ids::PacketType;
use binary_utils::binary::Stream;

pub struct OpenConnReply2 {
    pub magic: [u8; 16],
    pub server_guid: u64,
    pub client_address: InternetAddress,
    pub mtu: u16,
    pub encryption_enabled: bool,
}

impl OpenConnReply2 {
    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);

        stream.put_byte(PacketType::get_byte(PacketType::OpenConnReply2));
        stream.put(Vec::from(self.magic));
        stream.put_u64_be(self.server_guid);
        stream.put(self.client_address.put_address());
        stream.put_u16_be(self.mtu);
        stream.put_bool(self.encryption_enabled);

        Vec::from(stream.get_buffer())
    }

    pub fn decode(bytes: Vec<u8>) -> OpenConnReply2 {
        let mut stream = Stream::new(bytes, 0);

        let _ = stream.get_byte();
        let magic: [u8; 16] = stream.get(16).try_into().expect("Invalid length for magic");
        let server_guid = stream.get_u64_be();
        let (client_address, offset) = address::get_address(stream.get_remaining()).unwrap();
        stream.set_offset(stream.get_offset() + offset);
        let mtu = stream.get_u16_be();
        let encryption_enabled = stream.get_bool();

        OpenConnReply2 { magic, server_guid, client_address, mtu, encryption_enabled }
    }

    pub fn debug(&self) {
        println!("--- {}OpenConnReply2{} ---", color_format::COLOR_GOLD, COLOR_WHITE);
        println!("Magic: {:?}", self.magic);
        println!("Server GUID (Format DecToHex): {}", format!("{:x}", self.server_guid));
        println!("Client Address: {}:{}", self.client_address.address, self.client_address.port);
        println!("MTU: {}", self.mtu);
        println!("Encryption Enabled: {}", self.encryption_enabled);
    }
}
