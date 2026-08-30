use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::player_action_types::PlayerActionTypes;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerBlockActionWithBlockInfo {
    action_type: i32,
    block_position: Vec<i32>,
    face: i32,
}

impl PlayerBlockActionWithBlockInfo {
    pub fn get_action_type(&self) -> i32 {
        self.action_type
    }

    pub fn new(action_type: i32, block_position: Vec<i32>, face: i32) -> PlayerBlockActionWithBlockInfo {
        PlayerBlockActionWithBlockInfo { action_type, block_position, face }
    }

    pub fn read(stream: &mut Reader, action_type: i32) -> PlayerBlockActionWithBlockInfo {
        let block_position = PacketSerializer::get_block_pos(stream);
        let face = stream.get_var_i32();

        PlayerBlockActionWithBlockInfo { action_type, block_position, face }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_block_pos(stream, &self.block_position);
        stream.put_var_i32(self.face);
    }

    pub fn is_valid_action_type(action_type: i32) -> bool {
        match action_type {
            PlayerActionTypes::ABORT_BREAK
            | PlayerActionTypes::START_BREAK
            | PlayerActionTypes::CRACK_BREAK
            | PlayerActionTypes::PREDICT_DESTROY_BLOCK
            | PlayerActionTypes::CONTINUE_DESTROY_BLOCK => true,
            _ => false,
        }
    }
}
