use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct MapCreateLockedCopy {
    pub original_map_id: i64,
    pub new_map_id: i64
}

pub fn new(original_map_id: i64, new_map_id: i64) -> MapCreateLockedCopy {
    MapCreateLockedCopy { original_map_id, new_map_id }
}

impl Packet for MapCreateLockedCopy {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMapCreateLockedCopy.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_unique_id(&mut stream, self.original_map_id);
        PacketSerializer::put_actor_unique_id(&mut stream, self.new_map_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> MapCreateLockedCopy {
        let original_map_id = PacketSerializer::get_actor_unique_id(stream);
        let new_map_id = PacketSerializer::get_actor_unique_id(stream);

        MapCreateLockedCopy { original_map_id, new_map_id }
    }

    fn debug(&self) {
        println!("Original Map ID: {}", self.original_map_id);
        println!("New Map ID: {}", self.new_map_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
