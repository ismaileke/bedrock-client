use crate::protocol::packet_ids::PacketType;
use binary_utils::binary::Stream;

pub struct ConnReq {
    client_guid: i64,
    request_time: i64,
    secure: bool
}

pub fn new(client_guid: i64, request_time: i64, secure: bool) -> ConnReq {
    ConnReq{ client_guid, request_time, secure }
}

impl ConnReq {
    pub fn encode(&self) -> Vec<u8> {

        let mut stream = Stream::new(Vec::new(), 0);

        stream.put_byte(PacketType::get_byte(PacketType::ConnReq));
        stream.put_long(self.client_guid);
        stream.put_long(self.request_time);
        stream.put_bool(self.secure);

        stream.get_buffer()
    }
}