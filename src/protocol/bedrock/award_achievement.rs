use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AwardAchievement {
    pub achievement_id: i32,
}

impl Packet for AwardAchievement {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAwardAchievement.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_i32_le(self.achievement_id);
    }

    fn decode(stream: &mut Reader) -> AwardAchievement {
        let achievement_id = stream.get_i32_le();

        AwardAchievement { achievement_id }
    }
}
