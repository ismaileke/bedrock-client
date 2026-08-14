use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct BlockPickRequest {
    pub block_position: Vec<i32>,
    pub add_user_data: bool,
    pub hotbar_slot: u8,
}

impl Packet for BlockPickRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDBlockPickRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_block_pos(stream, &self.block_position);
        stream.put_bool(self.add_user_data);
        stream.put_u8(self.hotbar_slot);
    }

    fn decode(stream: &mut Reader) -> BlockPickRequest {
        let block_position = PacketSerializer::get_block_pos(stream);
        let add_user_data = stream.get_bool();
        let hotbar_slot = stream.get_u8();

        BlockPickRequest { block_position, add_user_data, hotbar_slot }
    }
}
