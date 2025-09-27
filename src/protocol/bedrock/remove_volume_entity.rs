use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct RemoveVolumeEntity {
    pub entity_net_id: u32,
    pub dimension: i32
}

pub fn new(entity_net_id: u32, dimension: i32) -> RemoveVolumeEntity {
    RemoveVolumeEntity { entity_net_id, dimension }
}

impl Packet for RemoveVolumeEntity {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRemoveVolumeEntity.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.entity_net_id);
        stream.put_var_int(self.dimension);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> RemoveVolumeEntity {
        let mut stream = Stream::new(bytes, 0);

        let entity_net_id = stream.get_unsigned_var_int();
        let dimension = stream.get_var_int();

        RemoveVolumeEntity { entity_net_id, dimension }
    }

    fn debug(&self) {
        println!("Entity Net ID: {}", self.entity_net_id);
        println!("Dimension: {}", self.dimension);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
