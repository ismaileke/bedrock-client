use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ContainerOpen {
    pub window_id: u8,
    pub window_type: u8,
    pub block_position: Vec<i32>,
    pub actor_unique_id: i64,
}

impl Packet for ContainerOpen {
    fn id(&self) -> u16 {
        BedrockPacketType::IDContainerOpen.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.window_id);
        stream.put_u8(self.window_type);
        PacketSerializer::put_block_pos(stream, &self.block_position);
        PacketSerializer::put_actor_unique_id(stream, self.actor_unique_id);
    }

    fn decode(stream: &mut Reader) -> ContainerOpen {
        let window_id = stream.get_u8();
        let window_type = stream.get_u8();
        let block_position = PacketSerializer::get_block_pos(stream);
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);

        ContainerOpen { window_id, window_type, block_position, actor_unique_id }
    }
}
