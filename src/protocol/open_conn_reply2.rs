use binary_utils::binary::Stream;
use crate::utils::address;
use crate::utils::address::InternetAddress;

pub struct OpenConnReply2 {
    pub magic: [u8; 16],
    pub server_guid: i64,
    pub client_address: InternetAddress,
    pub mtu: u16,
    pub encryption_enabled: bool
}

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