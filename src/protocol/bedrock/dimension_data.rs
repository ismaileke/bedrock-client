use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::dimension_data_entry::DimensionDataEntry;
use crate::protocol::bedrock::types::dimension_name_ids::DimensionNameIds;
use binary_utils::binary::{Reader, Writer};
use std::collections::HashMap;

#[derive(serde::Serialize, Debug)]
pub struct DimensionData {
    pub definitions: HashMap<String, DimensionDataEntry>,
}

impl Packet for DimensionData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDDimensionData.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.definitions.len() as u32);
        for (dimension_name_id, dimension_data) in &self.definitions {
            PacketSerializer::put_string(stream, dimension_name_id);
            dimension_data.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> DimensionData {
        let mut definitions = HashMap::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            let dimension_name_id = PacketSerializer::get_string(stream);
            let dimension_data = DimensionDataEntry::read(stream);

            if definitions.contains_key(&dimension_name_id) {
                panic!("Repeated dimension data for key \"{}\"", dimension_name_id);
            }
            if dimension_name_id != DimensionNameIds::OVERWORLD.to_string()
                && dimension_name_id != DimensionNameIds::NETHER.to_string()
                && dimension_name_id != DimensionNameIds::THE_END.to_string()
            {
                panic!("Invalid dimension name ID \"{}\"", dimension_name_id)
            }

            definitions.insert(dimension_name_id, dimension_data);
        }

        DimensionData { definitions }
    }
}
