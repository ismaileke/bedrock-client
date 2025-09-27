use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct SetLastHurtBy {
    pub entity_type_id: i32
}

pub fn new(entity_type_id: i32) -> SetLastHurtBy {
    SetLastHurtBy { entity_type_id }
}

impl Packet for SetLastHurtBy {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetLastHurtBy.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.entity_type_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SetLastHurtBy {
        let mut stream = Stream::new(bytes, 0);

        let entity_type_id = stream.get_var_int();

        SetLastHurtBy { entity_type_id }
    }

    fn debug(&self) {
        println!("Entity Type ID: {}", self.entity_type_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
