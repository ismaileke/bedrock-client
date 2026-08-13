use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ShowCredits {
    pub player_actor_runtime_id: u64,
    pub status: i32,
}

impl Packet for ShowCredits {
    fn id(&self) -> u16 {
        BedrockPacketType::IDShowCredits.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.player_actor_runtime_id);
        stream.put_var_i32(self.status);
    }

    fn decode(stream: &mut Reader) -> ShowCredits {
        let player_actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let status = stream.get_var_i32();

        ShowCredits {
            player_actor_runtime_id,
            status,
        }
    }
}

impl ShowCredits {
    pub const STATUS_START_CREDITS: i32 = 0;
    pub const STATUS_END_CREDITS: i32 = 1;
}
