use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::biome::biome_definition_data::BiomeDefinitionData;

#[derive(serde::Serialize, Debug)]
pub struct BiomeDefinitionList {
    pub definition_data: Vec<BiomeDefinitionData>,
    pub strings: Vec<String>
}

pub fn new(definition_data: Vec<BiomeDefinitionData>, strings: Vec<String>) -> BiomeDefinitionList {
    BiomeDefinitionList { definition_data, strings }
}

impl Packet for BiomeDefinitionList {
    fn id(&self) -> u16 {
        BedrockPacketType::IDBiomeDefinitionList.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.definition_data.len() as u32);
        for definition_data in &self.definition_data {
            definition_data.write(&mut stream);
        }
        stream.put_var_u32(self.strings.len() as u32);
        for string in &self.strings {
            PacketSerializer::put_string(&mut stream, string.clone());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> BiomeDefinitionList {
        let mut definition_data = Vec::new();
        let mut strings = Vec::new();
        let mut count = stream.get_var_u32();
        for _ in 0..count {
            definition_data.push(BiomeDefinitionData::read(stream));
        }
        count = stream.get_var_u32();
        for _ in 0..count {
            strings.push(PacketSerializer::get_string(stream));
        }

        BiomeDefinitionList { definition_data, strings }
    }

    fn debug(&self) {
        println!("Definition Data: {:?}", self.definition_data);
        println!("Strings: {:?}", self.strings);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
