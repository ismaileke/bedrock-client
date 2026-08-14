use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct Emote {
    pub actor_runtime_id: u64,
    pub emote_id: String,
    pub emote_length_ticks: u32,
    pub xbox_user_id: String,
    pub platform_chat_id: String,
    pub flags: u8,
}

impl Emote {
    pub const FLAG_SERVER: u8 = 1 << 0;
    pub const FLAG_MUTE_ANNOUNCEMENT: u8 = 1 << 1;
}

impl Packet for Emote {
    fn id(&self) -> u16 {
        BedrockPacketType::IDEmote.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        PacketSerializer::put_string(stream, &self.emote_id);
        stream.put_var_u32(self.emote_length_ticks);
        PacketSerializer::put_string(stream, &self.xbox_user_id);
        PacketSerializer::put_string(stream, &self.platform_chat_id);
        stream.put_u8(self.flags);
    }

    fn decode(stream: &mut Reader) -> Emote {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let emote_id = PacketSerializer::get_string(stream);
        let emote_length_ticks = stream.get_var_u32();
        let xbox_user_id = PacketSerializer::get_string(stream);
        let platform_chat_id = PacketSerializer::get_string(stream);
        let flags = stream.get_u8();

        Emote {
            actor_runtime_id,
            emote_id,
            emote_length_ticks,
            xbox_user_id,
            platform_chat_id,
            flags,
        }
    }
}
