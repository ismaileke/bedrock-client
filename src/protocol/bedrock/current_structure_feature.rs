use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct CurrentStructureFeature {
    pub current_structure_feature: String
}

pub fn new(current_structure_feature: String) -> CurrentStructureFeature {
    CurrentStructureFeature { current_structure_feature }
}

impl Packet for CurrentStructureFeature {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCurrentStructureFeature.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.current_structure_feature.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> CurrentStructureFeature {
        let mut stream = Stream::new(bytes, 0);

        let current_structure_feature = PacketSerializer::get_string(&mut stream);

        CurrentStructureFeature { current_structure_feature }
    }

    fn debug(&self) {
        println!("Current Structure Feature: {}", self.current_structure_feature);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
