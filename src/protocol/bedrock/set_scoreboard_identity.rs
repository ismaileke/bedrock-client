use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::scoreboard_identity_entry::ScoreboardIdentityEntry;

pub struct SetScoreboardIdentity {
    pub action_type: u8,
    pub entries: Vec<ScoreboardIdentityEntry>
}

pub fn new(action_type: u8, entries: Vec<ScoreboardIdentityEntry>) -> SetScoreboardIdentity {
    SetScoreboardIdentity { action_type, entries }
}

impl Packet for SetScoreboardIdentity {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetScoreboardIdentity.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.action_type);
        stream.put_var_u32(self.entries.len() as u32);
        for entry in &self.entries {
            stream.put_var_i64(entry.scoreboard_id);
            if self.action_type == SetScoreboardIdentity::TYPE_REGISTER_IDENTITY {
                PacketSerializer::put_actor_unique_id(&mut stream, entry.actor_unique_id.unwrap());
            }
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> SetScoreboardIdentity {
        let mut stream = Stream::new(bytes, 0);

        let action_type = stream.get_byte();
        let mut entries: Vec<ScoreboardIdentityEntry> = Vec::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            let scoreboard_id = stream.get_var_i64();
            let mut actor_unique_id = None;
            if action_type == SetScoreboardIdentity::TYPE_REGISTER_IDENTITY {
                actor_unique_id = Some(PacketSerializer::get_actor_unique_id(&mut stream));
            }
            entries.push(ScoreboardIdentityEntry{ scoreboard_id, actor_unique_id });
        }

        SetScoreboardIdentity { action_type, entries }
    }

    fn debug(&self) {
        println!("Action Type: {:?}", self.action_type);
        println!("Entries: {:?}", self.entries);

    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl SetScoreboardIdentity {
    pub const TYPE_REGISTER_IDENTITY: u8 = 0;
    pub const TYPE_CLEAR_IDENTITY: u8 = 1;
}
