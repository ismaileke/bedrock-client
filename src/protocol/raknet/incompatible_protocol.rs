use binary_utils::binary::Stream;

pub struct IncompatibleProtocol {
    pub server_protocol: u8,
    pub magic: [u8; 16],
    pub server_guid: u64,
}

pub fn decode(bytes: Vec<u8>) -> IncompatibleProtocol {
    let mut stream = Stream::new(bytes, 0);

    let _ = stream.get_byte();
    let server_protocol = stream.get_byte();
    let magic: [u8; 16] = stream.get(16).try_into().expect("Invalid length for magic");
    let server_guid = stream.get_u64_be();

    IncompatibleProtocol {
        server_protocol,
        magic,
        server_guid,
    }
}
