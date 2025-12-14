use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct LessonProgress {
    pub action: i32,
    pub score: i32,
    pub activity_id: String,
}

pub fn new(action: i32, score: i32, activity_id: String) -> LessonProgress {
    LessonProgress {
        action,
        score,
        activity_id,
    }
}

impl Packet for LessonProgress {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLessonProgress.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.action);
        stream.put_var_i32(self.score);
        PacketSerializer::put_string(&mut stream, self.activity_id.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> LessonProgress {
        let action = stream.get_var_i32();
        let score = stream.get_var_i32();
        let activity_id = PacketSerializer::get_string(stream);

        LessonProgress {
            action,
            score,
            activity_id,
        }
    }

    fn debug(&self) {
        println!("Action: {}", self.action);
        println!("Score: {}", self.score);
        println!("Activity ID: {}", self.activity_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl LessonProgress {
    pub const ACTION_START: i32 = 0;
    pub const ACTION_FINISH: i32 = 1;
    pub const ACTION_RESTART: i32 = 2;
}
