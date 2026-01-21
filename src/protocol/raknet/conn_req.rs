use crate::protocol::raknet::packet_ids::PacketType;
use binary_utils::binary::Stream;

pub struct ConnReq {
    client_guid: u64,
    request_time: u64,
    secure: bool,
}

impl ConnReq {
    pub fn new(client_guid: u64, request_time: u64, secure: bool) -> ConnReq {
        ConnReq { client_guid, request_time, secure }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);

        stream.put_byte(PacketType::get_byte(PacketType::ConnReq));
        stream.put_u64_be(self.client_guid);
        stream.put_u64_be(self.request_time);
        stream.put_bool(self.secure);

        Vec::from(stream.get_buffer())
    }

    pub fn decode(bytes: Vec<u8>) -> ConnReq {
        let mut stream = Stream::new(bytes, 0);

        let _ = stream.get_byte();
        let client_guid = stream.get_u64_be();
        let request_time = stream.get_u64_be();
        let secure = stream.get_bool();

        ConnReq { client_guid, request_time, secure }
    }
}
