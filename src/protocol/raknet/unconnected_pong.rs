use crate::protocol::raknet::packet_ids::PacketType;
use crate::utils::color_format;
use crate::utils::color_format::COLOR_WHITE;
use crate::protocol::raknet::packet_ids;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

pub struct UnconnectedPong {
    pub pong_time: u64,
    pub server_id: u64,
    pub server_name: String,
}

impl UnconnectedPong {
    pub fn create(pong_time: u64, server_id: u64, server_name: String) -> UnconnectedPong {
        UnconnectedPong { pong_time, server_id, server_name }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_byte(PacketType::get_byte(PacketType::UnconnectedPong));
        stream.put_u64_be(self.pong_time);
        stream.put_u64_be(self.server_id);
        stream.put(Vec::from(packet_ids::MAGIC));
        PacketSerializer::put_string(&mut stream, self.server_name.clone());
        Vec::from(stream.get_buffer())
    }

    pub fn decode(bytes: Vec<u8>) -> UnconnectedPong {
        let mut stream = Stream::new(bytes, 0);

        let _ = stream.get_byte();
        let pong_time = stream.get_u64_be();
        let server_id = stream.get_u64_be();
        let _ = stream.get(16);
        let server_name = PacketSerializer::get_string(&mut stream);

        UnconnectedPong { pong_time, server_id, server_name }
    }

    pub fn debug(&self) {
        println!("--- {}UnconnectedPong{} ---", color_format::COLOR_GOLD, COLOR_WHITE);
        println!("Pong Time: {:?}", self.pong_time);
        println!("Server ID: {:?}", self.server_id);
        println!("Server Name: {:?}", self.server_name);
    }
}
