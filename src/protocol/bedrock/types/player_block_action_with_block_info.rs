use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::player_action_types::PlayerActionTypes;
use crate::protocol::bedrock::types::player_block_action::PlayerBlockAction;

#[derive(Debug)]
pub struct PlayerBlockActionWithBlockInfo {
    action_type: i32,
    block_position: Vec<i32>,
    face: i32
}

impl PlayerBlockActionWithBlockInfo {
    pub fn new(action_type: i32, block_position: Vec<i32>, face: i32) -> PlayerBlockActionWithBlockInfo {
        if !Self::is_valid_action_type(action_type) {
            panic!("Invalid action type for PlayerBlockActionWithBlockInfo");
        }
        PlayerBlockActionWithBlockInfo { action_type, block_position, face }
    }

    pub fn read(stream: &mut Stream, action_type: i32) -> PlayerBlockActionWithBlockInfo {
        let block_position = PacketSerializer::get_signed_block_pos(stream);
        let face = stream.get_var_int();

        PlayerBlockActionWithBlockInfo{ action_type, block_position, face }
    }

    pub fn is_valid_action_type(action_type: i32) -> bool {
        match action_type {
            PlayerActionTypes::ABORT_BREAK | PlayerActionTypes::START_BREAK | PlayerActionTypes::CRACK_BREAK | PlayerActionTypes::PREDICT_DESTROY_BLOCK | PlayerActionTypes::CONTINUE_DESTROY_BLOCK => true,
            _ => false
        }
    }
}

impl PlayerBlockAction for PlayerBlockActionWithBlockInfo {
    fn get_action_type(&self) -> i32 {
        PlayerActionTypes::STOP_BREAK
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_signed_block_pos(stream, self.block_position.clone());
        stream.put_var_int(self.face);
    }
}
