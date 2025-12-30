use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

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
        BedrockPacketType::IDEmote.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        PacketSerializer::put_string(&mut stream, self.emote_id.clone());
        stream.put_var_u32(self.emote_length_ticks);
        PacketSerializer::put_string(&mut stream, self.xbox_user_id.clone());
        PacketSerializer::put_string(&mut stream, self.platform_chat_id.clone());
        stream.put_byte(self.flags);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> Emote {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let emote_id = PacketSerializer::get_string(stream);
        let emote_length_ticks = stream.get_var_u32();
        let xbox_user_id = PacketSerializer::get_string(stream);
        let platform_chat_id = PacketSerializer::get_string(stream);
        let flags = stream.get_byte();

        Emote {
            actor_runtime_id,
            emote_id,
            emote_length_ticks,
            xbox_user_id,
            platform_chat_id,
            flags,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
