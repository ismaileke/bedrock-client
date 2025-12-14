use crate::protocol::bedrock::types::bool_game_rule::BoolGameRule;
use crate::protocol::bedrock::types::float_game_rule::FloatGameRule;
use crate::protocol::bedrock::types::game_rule_types::GameRuleTypes;
use crate::protocol::bedrock::types::int_game_rule::IntGameRule;
use binary_utils::binary::Stream;
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum GameRule {
    Bool(BoolGameRule),
    Int(IntGameRule),
    Float(FloatGameRule),
}

impl GameRule {
    pub fn id(&self) -> u32 {
        match self {
            GameRule::Bool(_) => GameRuleTypes::BOOL,
            GameRule::Int(_) => GameRuleTypes::INT,
            GameRule::Float(_) => GameRuleTypes::FLOAT,
        }
    }

    pub fn is_player_modifiable(&self) -> bool {
        match self {
            GameRule::Bool(r) => r.is_player_modifiable(),
            GameRule::Int(r) => r.is_player_modifiable(),
            GameRule::Float(r) => r.is_player_modifiable(),
        }
    }

    pub fn write(&mut self, stream: &mut Stream, is_start_game: bool) {
        match self {
            GameRule::Bool(r) => r.write(stream, is_start_game),
            GameRule::Int(r) => r.write(stream, is_start_game),
            GameRule::Float(r) => r.write(stream, is_start_game),
        }
    }
}
