use binary_utils::binary::Stream;

pub struct ServerToClientHandshake {
    pub jwt: Vec<u8>,
}

pub fn decode(bytes: Vec<u8>) -> ServerToClientHandshake {
    let mut stream = Stream::new(bytes, 0);

    let length = stream.get_unsigned_var_int();
    let jwt = stream.get(length).expect("ServerToClientHandshake JWT Error");

    ServerToClientHandshake { jwt }
}