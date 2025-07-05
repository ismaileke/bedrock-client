use crate::protocol::raknet::packet_ids::PacketType;
use binary_utils::binary::Stream;
use crate::utils::color_format;
use crate::utils::color_format::COLOR_WHITE;

pub struct ConnectedPong {
    pub ping_time: i64,
    pub pong_time: i64
}

impl ConnectedPong {

    pub fn create(ping_time: i64, pong_time: i64) -> ConnectedPong {
        ConnectedPong { ping_time, pong_time }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_byte(PacketType::get_byte(PacketType::ConnectedPong));
        stream.put_long(self.ping_time);
        stream.put_long(self.pong_time);
        stream.get_buffer()
    }

    pub fn decode(bytes: Vec<u8>) -> ConnectedPong {
        let mut stream = Stream::new(bytes, 0);

        let _ = stream.get_byte();
        let ping_time = stream.get_long();
        let pong_time = stream.get_long();
        ConnectedPong{ ping_time, pong_time }
    }

    pub fn debug(&self) {
        println!("--- {}ConnectedPong{} ---", color_format::COLOR_GOLD, COLOR_WHITE);
        println!("Ping Time: {:?}", self.ping_time);
        println!("Pong Time: {:?}", self.pong_time);
    }
}
