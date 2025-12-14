use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct SetPlayerGameType {
    pub game_mode: i32,
}

pub fn new(game_mode: i32) -> SetPlayerGameType {
    SetPlayerGameType { game_mode }
}

impl Packet for SetPlayerGameType {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetPlayerGameType.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.game_mode);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SetPlayerGameType {
        let game_mode = stream.get_var_i32();

        SetPlayerGameType { game_mode }
    }

    fn debug(&self) {
        println!("Game Mode: {}", self.game_mode);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
