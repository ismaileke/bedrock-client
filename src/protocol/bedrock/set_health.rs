use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct SetHealth {
    pub health: i32
}

pub fn new(health: i32) -> SetHealth {
    SetHealth { health }
}

impl Packet for SetHealth {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetHealth.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.health);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SetHealth {
        let mut stream = Stream::new(bytes, 0);

        let health = stream.get_var_int();

        SetHealth { health }
    }

    fn debug(&self) {
        println!("Health: {}", self.health);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
