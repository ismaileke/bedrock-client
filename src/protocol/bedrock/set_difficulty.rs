use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
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
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.difficulty);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SetDifficulty {
        let difficulty = stream.get_var_u32();

        SetDifficulty { difficulty }
    }

    fn debug(&self) {
        println!("Difficulty: {}", self.difficulty);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
