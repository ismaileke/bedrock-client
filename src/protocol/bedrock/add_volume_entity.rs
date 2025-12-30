use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use binary_utils::binary::Stream;
use mojang_nbt::tag::tag::Tag;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct AddVolumeEntity {
    pub entity_net_id: u32,
    pub data: CacheableNBT,
    pub json_identifier: String,
    pub instance_name: String,
    pub min_bound: Vec<i32>,
    pub max_bound: Vec<i32>,
    pub dimension: i32,
    pub engine_version: String,
}

impl Packet for AddVolumeEntity {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAddVolumeEntity.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.entity_net_id);
        stream.put(self.data.get_encoded_nbt());
        PacketSerializer::put_string(&mut stream, self.json_identifier.clone());
        PacketSerializer::put_string(&mut stream, self.instance_name.clone());
        PacketSerializer::put_block_pos(&mut stream, self.min_bound.clone());
        PacketSerializer::put_block_pos(&mut stream, self.max_bound.clone());
        stream.put_var_i32(self.dimension);
        PacketSerializer::put_string(&mut stream, self.engine_version.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> AddVolumeEntity {
        let entity_net_id = stream.get_var_u32();
        let data = CacheableNBT::new(Tag::Compound(PacketSerializer::get_nbt_compound_root(stream)));
        let json_identifier = PacketSerializer::get_string(stream);
        let instance_name = PacketSerializer::get_string(stream);
        let min_bound = PacketSerializer::get_block_pos(stream);
        let max_bound = PacketSerializer::get_block_pos(stream);
        let dimension = stream.get_var_i32();
        let engine_version = PacketSerializer::get_string(stream);

        AddVolumeEntity {
            entity_net_id,
            data,
            json_identifier,
            instance_name,
            min_bound,
            max_bound,
            dimension,
            engine_version,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
