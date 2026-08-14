use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct LecternUpdate {
    pub page: u8,
    pub total_pages: u8,
    pub block_position: Vec<i32>,
}

impl Packet for LecternUpdate {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLecternUpdate.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.page);
        stream.put_u8(self.total_pages);
        PacketSerializer::put_block_pos(stream, &self.block_position);
    }

    fn decode(stream: &mut Reader) -> LecternUpdate {
        let page = stream.get_u8();
        let total_pages = stream.get_u8();
        let block_position = PacketSerializer::get_block_pos(stream);

        LecternUpdate { page, total_pages, block_position }
    }
}
