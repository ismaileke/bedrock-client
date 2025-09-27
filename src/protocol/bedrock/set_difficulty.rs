use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct SetDifficulty {
    pub difficulty: u32
}

pub fn new(difficulty: u32) -> SetDifficulty {
    SetDifficulty { difficulty }
}

impl Packet for SetDifficulty {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetDifficulty.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.difficulty);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SetDifficulty {
        let mut stream = Stream::new(bytes, 0);

        let difficulty = stream.get_unsigned_var_int();

        SetDifficulty { difficulty }
    }

    fn debug(&self) {
        println!("Difficulty: {}", self.difficulty);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
