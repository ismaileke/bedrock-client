use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct LessonProgress {
    pub action: i32,
    pub score: i32,
    pub activity_id: String,
}

impl Packet for LessonProgress {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLessonProgress.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.action);
        stream.put_var_i32(self.score);
        PacketSerializer::put_string(stream, self.activity_id.clone());
    }

    fn decode(stream: &mut Reader) -> LessonProgress {
        let action = stream.get_var_i32();
        let score = stream.get_var_i32();
        let activity_id = PacketSerializer::get_string(stream);

        LessonProgress { action, score, activity_id }
    }
}

impl LessonProgress {
    pub const ACTION_START: i32 = 0;
    pub const ACTION_FINISH: i32 = 1;
    pub const ACTION_RESTART: i32 = 2;
}
