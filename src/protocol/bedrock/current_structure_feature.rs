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
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.current_structure_feature.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CurrentStructureFeature {
        let current_structure_feature = PacketSerializer::get_string(stream);

        CurrentStructureFeature { current_structure_feature }
    }

    fn debug(&self) {
        println!("Current Structure Feature: {}", self.current_structure_feature);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
