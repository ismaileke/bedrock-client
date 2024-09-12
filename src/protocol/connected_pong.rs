use crate::protocol::packet_ids::PacketType;
use binary_utils::binary::Stream;

pub struct ConnectedPong {
    pub ping_time: i64,
    pub pong_time: i64
}

pub fn create(ping_time: i64, pong_time: i64) -> ConnectedPong {
    ConnectedPong { ping_time, pong_time }
}

impl ConnectedPong {
    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_byte(PacketType::get_byte(PacketType::ConnectedPong));
        stream.put_long(self.ping_time);
        stream.put_long(self.pong_time);
        stream.get_buffer()
    }
}

pub fn decode(bytes: Vec<u8>) -> ConnectedPong {
    let mut stream = Stream::new(bytes, 0);

    let _ = stream.get_byte();
    let ping_time = stream.get_long();
    let pong_time = stream.get_long();
    ConnectedPong{ ping_time, pong_time }
}