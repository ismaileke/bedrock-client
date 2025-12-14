use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct RemoveVolumeEntity {
    pub entity_net_id: u32,
    pub dimension: i32,
}

pub fn new(entity_net_id: u32, dimension: i32) -> RemoveVolumeEntity {
    RemoveVolumeEntity {
        entity_net_id,
        dimension,
    }
}

impl Packet for RemoveVolumeEntity {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRemoveVolumeEntity.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.entity_net_id);
        stream.put_var_i32(self.dimension);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> RemoveVolumeEntity {
        let entity_net_id = stream.get_var_u32();
        let dimension = stream.get_var_i32();

        RemoveVolumeEntity {
            entity_net_id,
            dimension,
        }
    }

    fn debug(&self) {
        println!("Entity Net ID: {}", self.entity_net_id);
        println!("Dimension: {}", self.dimension);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
