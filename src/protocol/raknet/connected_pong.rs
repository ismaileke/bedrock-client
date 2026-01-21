use crate::protocol::raknet::packet_ids::PacketType;
use crate::utils::color_format;
use crate::utils::color_format::COLOR_WHITE;
use binary_utils::binary::Stream;

pub struct ConnectedPong {
    pub ping_time: u64,
    pub pong_time: u64,
}

impl ConnectedPong {
    pub fn create(ping_time: u64, pong_time: u64) -> ConnectedPong {
        ConnectedPong { ping_time, pong_time }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_byte(PacketType::get_byte(PacketType::ConnectedPong));
        stream.put_u64_be(self.ping_time);
        stream.put_u64_be(self.pong_time);
        Vec::from(stream.get_buffer())
    }

    pub fn decode(bytes: Vec<u8>) -> ConnectedPong {
        let mut stream = Stream::new(bytes, 0);

        let _ = stream.get_byte();
        let ping_time = stream.get_u64_be();
        let pong_time = stream.get_u64_be();

        ConnectedPong { ping_time, pong_time }
    }

    pub fn debug(&self) {
        println!("--- {}ConnectedPong{} ---", color_format::COLOR_GOLD, COLOR_WHITE);
        println!("Ping Time: {:?}", self.ping_time);
        println!("Pong Time: {:?}", self.pong_time);
    }
}
