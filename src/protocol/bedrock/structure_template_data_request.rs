use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::structure_settings::StructureSettings;

pub struct StructureTemplateDataRequest {
    pub structure_template_name: String,
    pub structure_block_position: Vec<i32>,
    pub structure_settings: StructureSettings,
    pub request_type: u8
}

pub fn new(structure_template_name: String, structure_block_position: Vec<i32>, structure_settings: StructureSettings, request_type: u8) -> StructureTemplateDataRequest {
    StructureTemplateDataRequest { structure_template_name, structure_block_position, structure_settings, request_type }
}

impl Packet for StructureTemplateDataRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDStructureTemplateDataRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.structure_template_name.clone());
        PacketSerializer::put_block_pos(&mut stream, self.structure_block_position.clone());
        PacketSerializer::put_structure_settings(&mut stream, &self.structure_settings);
        stream.put_byte(self.request_type);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> StructureTemplateDataRequest {
        let structure_template_name = PacketSerializer::get_string(stream);
        let structure_block_position = PacketSerializer::get_block_pos(stream);
        let structure_settings = PacketSerializer::get_structure_settings(stream);
        let request_type = stream.get_byte();

        StructureTemplateDataRequest { structure_template_name, structure_block_position, structure_settings, request_type }
    }

    fn debug(&self) {
        println!("Structure Template Name: {}", self.structure_template_name);
        println!("Structure Block Position: {:?}", self.structure_block_position.clone());
        println!("Structure Settings: {:?}", self.structure_settings);
        println!("Request Type: {}", self.request_type);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
