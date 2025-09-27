use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct PlayerAction {
    pub actor_runtime_id: u64,
    pub action: i32, //see types/player_action_types.rs
    pub block_position: Vec<i32>,
    pub result_position: Vec<i32>,
    pub face: i32
}

pub fn new(actor_runtime_id: u64, action: i32, block_position: Vec<i32>, result_position: Vec<i32>, face: i32) -> PlayerAction {
    PlayerAction { actor_runtime_id, action, block_position, result_position, face }
}

impl Packet for PlayerAction {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerAction.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_var_int(self.action);
        PacketSerializer::put_block_pos(&mut stream, self.block_position.clone());
        PacketSerializer::put_block_pos(&mut stream, self.result_position.clone());
        stream.put_var_int(self.face);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> PlayerAction {
        let mut stream = Stream::new(bytes, 0);

        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let action = stream.get_var_int();
        let block_position = PacketSerializer::get_block_pos(&mut stream);
        let result_position = PacketSerializer::get_block_pos(&mut stream);
        let face = stream.get_var_int();

        PlayerAction { actor_runtime_id, action, block_position, result_position, face }
    }

    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Action: {}", self.action);
        println!("Block Position: {:?}", self.block_position);
        println!("Result Position: {:?}", self.result_position);
        println!("Face: {}", self.face);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
