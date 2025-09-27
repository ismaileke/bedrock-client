use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::player_action_types::PlayerActionTypes;
use crate::protocol::bedrock::types::player_block_action::PlayerBlockAction;

#[derive(Debug)]
pub struct PlayerBlockActionStopBreak {}

impl PlayerBlockAction for PlayerBlockActionStopBreak {
    fn get_action_type(&self) -> i32 {
        PlayerActionTypes::STOP_BREAK
    }

    fn write(&mut self, _stream: &mut Stream) {
        //NOOP
    }
}
