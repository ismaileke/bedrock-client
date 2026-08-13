use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::structure_editor_data::StructureEditorData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct StructureBlockUpdate {
    pub block_position: Vec<i32>,
    pub structure_editor_data: StructureEditorData,
    pub is_powered: bool,
    pub water_logged: bool,
}

impl Packet for StructureBlockUpdate {
    fn id(&self) -> u16 {
        BedrockPacketType::IDStructureBlockUpdate.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_block_pos(stream, self.block_position.clone());
        PacketSerializer::put_structure_editor_data(stream, &self.structure_editor_data);
        stream.put_bool(self.is_powered);
        stream.put_bool(self.water_logged);
    }

    fn decode(stream: &mut Reader) -> StructureBlockUpdate {
        let block_position = PacketSerializer::get_block_pos(stream);
        let structure_editor_data = PacketSerializer::get_structure_editor_data(stream);
        let is_powered = stream.get_bool();
        let water_logged = stream.get_bool();

        StructureBlockUpdate { block_position, structure_editor_data, is_powered, water_logged }
    }
}
