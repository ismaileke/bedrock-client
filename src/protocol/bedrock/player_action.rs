use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

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
        BedrockPacketType::IDPlayerAction.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_var_i32(self.action);
        PacketSerializer::put_block_pos(&mut stream, self.block_position.clone());
        PacketSerializer::put_block_pos(&mut stream, self.result_position.clone());
        stream.put_var_i32(self.face);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PlayerAction {
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
