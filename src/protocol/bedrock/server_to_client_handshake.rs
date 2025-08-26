use binary_utils::binary::Stream;

pub struct ServerToClientHandshake {
    pub jwt: Vec<u8>,
}

impl ServerToClientHandshake {
    pub fn debug(&self) {
        println!("JWT: {}", String::from_utf8(self.jwt.clone()).unwrap());
    }
}

pub fn decode(bytes: Vec<u8>) -> ServerToClientHandshake {
    let mut stream = Stream::new(bytes, 0);

    let length = stream.get_unsigned_var_int();
    let jwt = stream.get(length).expect("ServerToClientHandshake JWT Error");

    ServerToClientHandshake { jwt }
}