use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerAction {
    pub actor_runtime_id: u64,
    pub action: i32, //see types/player_action_types.rs
    pub block_position: Vec<i32>,
    pub result_position: Vec<i32>,
    pub face: i32,
}

impl Packet for PlayerAction {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerAction.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        stream.put_var_i32(self.action);
        PacketSerializer::put_block_pos(stream, &self.block_position);
        PacketSerializer::put_block_pos(stream, &self.result_position);
        stream.put_var_i32(self.face);
    }

    fn decode(stream: &mut Reader) -> PlayerAction {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let action = stream.get_var_i32();
        let block_position = PacketSerializer::get_block_pos(stream);
        let result_position = PacketSerializer::get_block_pos(stream);
        let face = stream.get_var_i32();

        PlayerAction {
            actor_runtime_id,
            action,
            block_position,
            result_position,
            face,
        }
    }
}
