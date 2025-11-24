use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::player_action_types::PlayerActionTypes;

#[derive(serde::Serialize, Debug)]
pub struct PlayerBlockActionStopBreak {}

impl PlayerBlockActionStopBreak {
    pub fn get_action_type(&self) -> i32 {
        PlayerActionTypes::STOP_BREAK
    }

    pub fn read(_stream: &mut Stream, _action_type: i32) -> PlayerBlockActionStopBreak {
        PlayerBlockActionStopBreak {}
    }

    pub fn write(&mut self, _stream: &mut Stream) {
        //NOOP
    }
}
