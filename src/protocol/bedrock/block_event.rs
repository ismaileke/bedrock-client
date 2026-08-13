use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct BlockEvent {
    pub block_position: Vec<i32>,
    pub event_type: i32,
    pub event_data: i32,
}

impl Packet for BlockEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDBlockEvent.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_block_pos(stream, self.block_position.clone());
        stream.put_var_i32(self.event_type);
        stream.put_var_i32(self.event_data);
    }

    fn decode(stream: &mut Reader) -> BlockEvent {
        let block_position = PacketSerializer::get_block_pos(stream);
        let event_type = stream.get_var_i32();
        let event_data = stream.get_var_i32();

        BlockEvent { block_position, event_type, event_data }
    }
}
