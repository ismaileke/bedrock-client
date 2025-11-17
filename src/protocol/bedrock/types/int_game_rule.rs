use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::game_rule::GameRule;
use crate::protocol::bedrock::types::game_rule_types::GameRuleTypes;

#[derive(Debug)]
pub struct IntGameRule {
    pub value: u32,
    pub is_player_modifiable: bool
}

impl IntGameRule {
    pub fn new(value: u32, is_player_modifiable: bool) -> IntGameRule {
        IntGameRule{ value, is_player_modifiable }
    }

    pub fn read(stream: &mut Stream, is_player_modifiable: bool, is_start_game: bool) -> IntGameRule {
        IntGameRule{ value: if is_start_game { stream.get_unsigned_var_int() } else { stream.get_l_int() }, is_player_modifiable }
    }
}

impl GameRule for IntGameRule {
    fn id(&self) -> u32 {
        GameRuleTypes::INT
    }

    fn is_player_modifiable(&self) -> bool {
        self.is_player_modifiable
    }

    fn write(&mut self, stream: &mut Stream, is_start_game: bool) {
        if is_start_game {
            stream.put_unsigned_var_int(self.value);
        } else {
            stream.put_l_int(self.value);
        }
    }
}


