use binary_utils::binary::Stream;
use crate::utils::{address, color_format};
use crate::utils::address::InternetAddress;
use crate::utils::color_format::COLOR_WHITE;

pub struct OpenConnReply2 {
    pub magic: [u8; 16],
    pub server_guid: i64,
    pub client_address: InternetAddress,
    pub mtu: u16,
    pub encryption_enabled: bool
}

impl OpenConnReply2 {
    pub fn decode(bytes: Vec<u8>) -> OpenConnReply2 {
        let mut stream = Stream::new(bytes, 0);

        let _ = stream.get_byte();
        let magic: [u8; 16] = stream.get(16).expect("Failed to get magic").try_into().expect("Invalid length for magic");

        let server_guid = stream.get_long();
        let (client_address, offset) = address::get_address(stream.get_remaining().unwrap()).unwrap();
        stream.set_offset(stream.get_offset() + offset);
        let mtu = stream.get_short();
        let encryption_enabled = stream.get_bool();

        OpenConnReply2 { magic, server_guid, client_address, mtu, encryption_enabled }
    }

    pub fn debug(&self) {
        println!("--- {}OpenConnReply2{} ---", color_format::COLOR_GOLD, COLOR_WHITE);
        println!("Magic: {:?}", self.magic);
        let guid_format = format!("{:x}", self.server_guid);
        println!("Server GUID (Format DecToHex): {}", guid_format);
        println!("Client Address: {}:{}", self.client_address.address, self.client_address.port);
        println!("MTU: {}", self.mtu);
        println!("Encryption Enabled: {}", self.encryption_enabled);
    }
}
