use std::fmt::Debug;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::player_block_action_stop_break::PlayerBlockActionStopBreak;
use crate::protocol::bedrock::types::player_block_action_with_block_info::PlayerBlockActionWithBlockInfo;

#[derive(serde::Serialize, Debug)]
pub enum PlayerBlockAction {
    StopBreak(PlayerBlockActionStopBreak),
    WithBlockInfo(PlayerBlockActionWithBlockInfo),
}

impl PlayerBlockAction {
    pub fn get_action_type(&self) -> i32 {
        match self {
            PlayerBlockAction::StopBreak(r) => r.get_action_type(),
            PlayerBlockAction::WithBlockInfo(r) => r.get_action_type()
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        match self {
            PlayerBlockAction::StopBreak(r) => r.write(stream),
            PlayerBlockAction::WithBlockInfo(r) => r.write(stream)
        }
    }
}