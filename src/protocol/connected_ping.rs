use binary_utils::binary::Stream;
use crate::protocol::packet_ids::PacketType;

pub struct ConnectedPing {
    pub ping_time: i64
}

pub fn create(ping_time: i64) -> ConnectedPing {
    ConnectedPing { ping_time }
}

impl ConnectedPing {
    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_byte(PacketType::get_byte(PacketType::ConnectedPing));
        stream.put_long(self.ping_time);
        stream.get_buffer()
    }
}

pub fn decode(bytes: Vec<u8>) -> ConnectedPing {
    let mut stream = Stream::new(bytes, 0);

    let _ = stream.get_byte();
    let ping_time = stream.get_long();
    ConnectedPing{ ping_time }
}