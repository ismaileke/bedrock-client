use crate::protocol::raknet::packet_ids::PacketType;
use crate::utils::color_format;
use crate::utils::color_format::COLOR_WHITE;
use crate::protocol::raknet::packet_ids;
use binary_utils::binary::Stream;

pub struct UnconnectedPing {
    pub ping_time: u64,
    pub client_guid: u64,
}

impl UnconnectedPing {
    pub fn create(ping_time: u64, client_guid: u64) -> UnconnectedPing {
        UnconnectedPing { ping_time, client_guid }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_byte(PacketType::get_byte(PacketType::UnconnectedPing));
        stream.put_u64_be(self.ping_time);
        stream.put(Vec::from(packet_ids::MAGIC));
        stream.put_u64_be(self.client_guid);
        Vec::from(stream.get_buffer())
    }

    pub fn decode(bytes: Vec<u8>) -> UnconnectedPing {
        let mut stream = Stream::new(bytes, 0);

        let _ = stream.get_byte();
        let ping_time = stream.get_u64_be();
        let _ = stream.get(16);
        let client_guid = stream.get_u64_be();

        UnconnectedPing { ping_time, client_guid }
    }

    pub fn debug(&self) {
        println!("--- {}UnconnectedPing{} ---", color_format::COLOR_GOLD, COLOR_WHITE);
        println!("Ping Time: {:?}", self.ping_time);
        println!("Client GUID: {:?}", self.client_guid);
    }
}
