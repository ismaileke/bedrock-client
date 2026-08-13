use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::biome::biome_definition_data::BiomeDefinitionData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct BiomeDefinitionList {
    pub definition_data: Vec<BiomeDefinitionData>,
    pub strings: Vec<String>,
}

impl Packet for BiomeDefinitionList {
    fn id(&self) -> u16 {
        BedrockPacketType::IDBiomeDefinitionList.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.definition_data.len() as u32);
        for definition_data in &self.definition_data {
            definition_data.write(stream);
        }
        stream.put_var_u32(self.strings.len() as u32);
        for string in &self.strings {
            PacketSerializer::put_string(stream, string.clone());
        }
    }

    fn decode(stream: &mut Reader) -> BiomeDefinitionList {
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
}
