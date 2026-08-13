use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::game_rule::GameRule;
use binary_utils::binary::{Reader, Writer};
use std::collections::HashMap;

#[derive(serde::Serialize, Debug)]
pub struct GameRulesChanged {
    pub game_rules: HashMap<String, GameRule>,
}

impl Packet for GameRulesChanged {
    fn id(&self) -> u16 {
        BedrockPacketType::IDGameRulesChanged.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_game_rules(stream, &mut self.game_rules, false);
    }

    fn decode(stream: &mut Reader) -> GameRulesChanged {
        let game_rules = PacketSerializer::get_game_rules(stream, false);

        GameRulesChanged { game_rules }
    }
}
