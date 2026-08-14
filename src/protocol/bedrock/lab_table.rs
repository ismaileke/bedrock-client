use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct LabTable {
    pub action_type: u8,
    pub block_position: Vec<i32>,
    pub reaction_type: u8,
}

impl Packet for LabTable {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLabTable.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.action_type);
        PacketSerializer::put_block_pos(stream, &self.block_position);
        stream.put_u8(self.reaction_type);
    }

    fn decode(stream: &mut Reader) -> LabTable {
        let action_type = stream.get_u8();
        let block_position = PacketSerializer::get_block_pos(stream);
        let reaction_type = stream.get_u8();

        LabTable {
            action_type,
            block_position,
            reaction_type,
        }
    }
}

impl LabTable {
    pub const TYPE_START_COMBINE: u8 = 0;
    pub const TYPE_START_REACTION: u8 = 1;
    pub const TYPE_RESET: u8 = 2;
}
