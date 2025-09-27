use std::any::Any;
use std::collections::HashMap;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::game_rule::GameRule;

pub struct GameRulesChanged {
    pub game_rules: HashMap<String, Box<dyn GameRule>>
}

pub fn new(game_rules: HashMap<String, Box<dyn GameRule>>) -> GameRulesChanged {
    GameRulesChanged { game_rules }
}

impl Packet for GameRulesChanged {
    fn id(&self) -> u16 {
        BedrockPacketType::IDGameRulesChanged.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_game_rules(&mut stream, &mut self.game_rules);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> GameRulesChanged {
        let mut stream = Stream::new(bytes, 0);

        let game_rules = PacketSerializer::get_game_rules(&mut stream);

        GameRulesChanged { game_rules }
    }

    fn debug(&self) {
        println!("Game Rules: {:?}", self.game_rules);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
