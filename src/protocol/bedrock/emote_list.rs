use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct EmoteList {
    pub player_actor_runtime_id: u64,
    pub emote_ids: Vec<String>,
}

impl Packet for EmoteList {
    fn id(&self) -> u16 {
        BedrockPacketType::IDEmoteList.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.player_actor_runtime_id);
        stream.put_var_u32(self.emote_ids.len() as u32);
        for emote_id in self.emote_ids.iter() {
            PacketSerializer::put_uuid(stream, &emote_id);
        }
    }

    fn decode(stream: &mut Reader) -> EmoteList {
        let player_actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let emote_ids_len = stream.get_var_u32() as usize;
        let mut emote_ids = Vec::new();
        for _ in 0..emote_ids_len {
            emote_ids.push(PacketSerializer::get_uuid(stream));
        }

        EmoteList { player_actor_runtime_id, emote_ids }
    }
}
