use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct AwardAchievement {
    pub achievement_id: i32
}

pub fn new(achievement_id: i32) -> AwardAchievement {
    AwardAchievement { achievement_id }
}

impl Packet for AwardAchievement {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAwardAchievement.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_i32_le(self.achievement_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> AwardAchievement {
        let mut stream = Stream::new(bytes, 0);

        let achievement_id = stream.get_i32_le();

        AwardAchievement { achievement_id }
    }

    fn debug(&self) {
        println!("Achievement ID: {}", self.achievement_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
