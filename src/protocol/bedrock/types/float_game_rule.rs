use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::game_rule::GameRule;
use crate::protocol::bedrock::types::game_rule_types::GameRuleTypes;

#[derive(Debug)]
pub struct FloatGameRule {
    pub value: f32,
    pub is_player_modifiable: bool
}

impl FloatGameRule {
    pub fn new(value: f32, is_player_modifiable: bool) -> FloatGameRule {
        FloatGameRule{ value, is_player_modifiable }
    }

    pub fn read(stream: &mut Stream, is_player_modifiable: bool) -> FloatGameRule {
        FloatGameRule{ value: stream.get_f32_le(), is_player_modifiable }
    }
}

impl GameRule for FloatGameRule {
    fn id(&self) -> u32 {
        GameRuleTypes::FLOAT
    }

    fn is_player_modifiable(&self) -> bool {
        self.is_player_modifiable
    }

    fn write(&mut self, stream: &mut Stream, _is_start_game: bool) {
        stream.put_f32_le(self.value);
    }
}


