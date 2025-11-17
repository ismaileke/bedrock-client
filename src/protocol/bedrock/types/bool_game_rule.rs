use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::game_rule::GameRule;
use crate::protocol::bedrock::types::game_rule_types::GameRuleTypes;

#[derive(Debug)]
pub struct BoolGameRule {
    pub value: bool,
    pub is_player_modifiable: bool
}

impl BoolGameRule {
    pub fn new(value: bool, is_player_modifiable: bool) -> BoolGameRule {
        BoolGameRule{ value, is_player_modifiable }
    }

    pub fn read(stream: &mut Stream, is_player_modifiable: bool) -> BoolGameRule {
        BoolGameRule{ value: stream.get_bool(), is_player_modifiable }
    }
}

impl GameRule for BoolGameRule {
    fn id(&self) -> u32 {
        GameRuleTypes::BOOL
    }

    fn is_player_modifiable(&self) -> bool {
        self.is_player_modifiable
    }

    fn write(&mut self, stream: &mut Stream, _is_start_game: bool) {
        stream.put_bool(self.value);
    }
}


