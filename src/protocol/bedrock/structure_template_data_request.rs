use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::structure_settings::StructureSettings;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct StructureTemplateDataRequest {
    pub structure_template_name: String,
    pub structure_block_position: Vec<i32>,
    pub structure_settings: StructureSettings,
    pub request_type: u8,
}

impl Packet for StructureTemplateDataRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDStructureTemplateDataRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.structure_template_name.clone());
        PacketSerializer::put_block_pos(stream, self.structure_block_position.clone());
        PacketSerializer::put_structure_settings(stream, &self.structure_settings);
        stream.put_u8(self.request_type);
    }

    fn decode(stream: &mut Reader) -> StructureTemplateDataRequest {
        let structure_template_name = PacketSerializer::get_string(stream);
        let structure_block_position = PacketSerializer::get_block_pos(stream);
        let structure_settings = PacketSerializer::get_structure_settings(stream);
        let request_type = stream.get_u8();

        StructureTemplateDataRequest {
            structure_template_name,
            structure_block_position,
            structure_settings,
            request_type,
        }
    }
}
