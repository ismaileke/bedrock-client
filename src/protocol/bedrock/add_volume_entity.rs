use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;

pub struct AddVolumeEntity {
    pub entity_net_id: u32,
    pub data: CacheableNBT,
    pub json_identifier: String,
    pub instance_name: String,
    pub min_bound: Vec<i32>,
    pub max_bound: Vec<i32>,
    pub dimension: i32,
    pub engine_version: String
}

pub fn new(
    entity_net_id: u32,
    data: CacheableNBT,
    json_identifier: String,
    instance_name: String,
    min_bound: Vec<i32>,
    max_bound: Vec<i32>,
    dimension: i32,
    engine_version: String
) -> AddVolumeEntity {
    AddVolumeEntity { entity_net_id, data, json_identifier, instance_name, min_bound, max_bound, dimension, engine_version }
}

impl Packet for AddVolumeEntity {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAddVolumeEntity.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.entity_net_id);
        stream.put(self.data.get_encoded_nbt());
        PacketSerializer::put_string(&mut stream, self.json_identifier.clone());
        PacketSerializer::put_string(&mut stream, self.instance_name.clone());
        PacketSerializer::put_block_pos(&mut stream, self.min_bound.clone());
        PacketSerializer::put_block_pos(&mut stream, self.max_bound.clone());
        stream.put_var_int(self.dimension);
        PacketSerializer::put_string(&mut stream, self.engine_version.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> AddVolumeEntity {
        let mut stream = Stream::new(bytes, 0);

        let entity_net_id = stream.get_unsigned_var_int();
        let data = CacheableNBT::new(Box::new(PacketSerializer::get_nbt_compound_root(&mut stream)));
        let json_identifier = PacketSerializer::get_string(&mut stream);
        let instance_name = PacketSerializer::get_string(&mut stream);
        let min_bound = PacketSerializer::get_block_pos(&mut stream);
        let max_bound = PacketSerializer::get_block_pos(&mut stream);
        let dimension = stream.get_var_int();
        let engine_version = PacketSerializer::get_string(&mut stream);

        AddVolumeEntity { entity_net_id, data, json_identifier, instance_name, min_bound, max_bound, dimension, engine_version }
    }

    fn debug(&self) {
        println!("Entity Net ID: {}", self.entity_net_id);
        println!("Data(NBT): {:?}", self.data);
        println!("JSON Identifier: {}", self.json_identifier);
        println!("Instance Name: {}", self.instance_name);
        println!("Min Bound: {:?}", self.min_bound);
        println!("Max Bound: {:?}", self.max_bound);
        println!("Dimension: {}", self.dimension);
        println!("Engine Version: {}", self.engine_version);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
