use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct SetDefaultGameType {
    pub game_mode: i32
}

pub fn new(game_mode: i32) -> SetDefaultGameType {
    SetDefaultGameType { game_mode }
}

impl Packet for SetDefaultGameType {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetDefaultGameType.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.game_mode);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SetDefaultGameType {
        let mut stream = Stream::new(bytes, 0);

        let game_mode = stream.get_var_int();

        SetDefaultGameType { game_mode }
    }

    fn debug(&self) {
        println!("Game Mode: {}", self.game_mode);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
