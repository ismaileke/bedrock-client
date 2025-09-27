use std::any::Any;
use std::collections::HashMap;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::dimension_data_entry::DimensionDataEntry;
use crate::protocol::bedrock::types::dimension_name_ids::DimensionNameIds;

pub struct DimensionData {
    pub definitions: HashMap<String, DimensionDataEntry>

}

pub fn new(definitions: HashMap<String, DimensionDataEntry>) -> DimensionData {
    DimensionData { definitions }
}

impl Packet for DimensionData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDDimensionData.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.definitions.len() as u32);
        for (dimension_name_id, dimension_data) in &self.definitions {
            PacketSerializer::put_string(&mut stream, dimension_name_id.to_string());
            dimension_data.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> DimensionData {
        let mut stream = Stream::new(bytes, 0);

        let mut definitions = HashMap::new();
        let count = stream.get_unsigned_var_int();
        for _ in 0..count {
            let dimension_name_id = PacketSerializer::get_string(&mut stream);
            let dimension_data = DimensionDataEntry::read(&mut stream);

            if definitions.contains_key(&dimension_name_id) {
                panic!("Repeated dimension data for key \"{}\"", dimension_name_id);
            }
            if dimension_name_id != DimensionNameIds::OVERWORLD.to_string() && dimension_name_id != DimensionNameIds::NETHER.to_string() && dimension_name_id != DimensionNameIds::THE_END.to_string() {
                panic!("Invalid dimension name ID \"{}\"", dimension_name_id)
            }

            definitions.insert(dimension_name_id, dimension_data);
        }

        DimensionData { definitions }
    }

    fn debug(&self) {
        println!("Definitions: {:?}", self.definitions);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
