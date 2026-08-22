use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::scoreboard_identity_entry::ScoreboardIdentityEntry;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetScoreboardIdentity {
    pub action_type: u8,
    pub entries: Vec<ScoreboardIdentityEntry>,
}

impl Packet for SetScoreboardIdentity {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetScoreboardIdentity.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.action_type);
        stream.put_var_u32(self.entries.len() as u32);
        for entry in &self.entries {
            stream.put_var_i64(entry.scoreboard_id);
            PacketSerializer::put_actor_unique_id(stream, entry.actor_unique_id);
        }
    }

    fn decode(stream: &mut Reader) -> SetScoreboardIdentity {
        let action_type = stream.get_u8();
        let mut entries: Vec<ScoreboardIdentityEntry> = Vec::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            let scoreboard_id = stream.get_var_i64();
            let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
            entries.push(ScoreboardIdentityEntry { scoreboard_id, actor_unique_id });
        }

        SetScoreboardIdentity { action_type, entries }
    }
}

impl SetScoreboardIdentity {
    pub const TYPE_REGISTER_IDENTITY: u8 = 0;
    pub const TYPE_CLEAR_IDENTITY: u8 = 1;
}
