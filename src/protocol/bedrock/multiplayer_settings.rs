use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct MultiplayerSettings {
    pub action: i32
}

pub fn new(action: i32) -> MultiplayerSettings {
    MultiplayerSettings { action }
}

impl MultiplayerSettings {
    pub const ACTION_ENABLE_MULTIPLAYER: i32 = 0;
    pub const ACTION_DISABLE_MULTIPLAYER: i32 = 1;
    pub const ACTION_REFRESH_JOIN_CODE: i32 = 2;
}

impl Packet for MultiplayerSettings {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMultiplayerSettings.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.action);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> MultiplayerSettings {
        let mut stream = Stream::new(bytes, 0);

        let action = stream.get_var_int();

        MultiplayerSettings { action }
    }

    fn debug(&self) {
        println!("Action: {}", self.action);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
