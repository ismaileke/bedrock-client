use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::score_entry::ScoreEntry;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct SetScore {
    pub action_type: u8,
    pub entries: Vec<ScoreEntry>,
}

pub fn new(action_type: u8, entries: Vec<ScoreEntry>) -> SetScore {
    SetScore {
        action_type,
        entries,
    }
}

impl Packet for SetScore {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetScore.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.action_type);
        stream.put_var_u32(self.entries.len() as u32);
        for entry in &self.entries {
            stream.put_var_i64(entry.scoreboard_id);
            PacketSerializer::put_string(&mut stream, entry.objective_name.clone());
            stream.put_i32_le(entry.score);
            if self.action_type != Self::TYPE_REMOVE {
                stream.put_byte(entry.entity_type);
                match entry.entity_type {
                    ScoreEntry::TYPE_PLAYER | ScoreEntry::TYPE_ENTITY => {
                        PacketSerializer::put_actor_unique_id(
                            &mut stream,
                            entry.actor_unique_id.unwrap(),
                        );
                    }
                    ScoreEntry::TYPE_FAKE_PLAYER => {
                        PacketSerializer::put_string(
                            &mut stream,
                            entry.custom_name.clone().unwrap(),
                        );
                    }
                    _ => {
                        panic!("Unknown entry type {}", entry.entity_type);
                    }
                }
            }
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SetScore {
        let action_type = stream.get_byte();
        let count = stream.get_var_u32();
        let mut entries: Vec<ScoreEntry> = Vec::new();
        for _ in 0..count {
            let scoreboard_id = stream.get_var_i64();
            let objective_name = PacketSerializer::get_string(stream);
            let score = stream.get_i32_le();
            let mut entity_type = 0; // Why I did IDK
            let mut actor_unique_id = None;
            let mut custom_name = None;
            if action_type != Self::TYPE_REMOVE {
                entity_type = stream.get_byte();
                match entity_type {
                    ScoreEntry::TYPE_PLAYER | ScoreEntry::TYPE_ENTITY => {
                        actor_unique_id = Some(PacketSerializer::get_actor_unique_id(stream));
                    }
                    ScoreEntry::TYPE_FAKE_PLAYER => {
                        custom_name = Some(PacketSerializer::get_string(stream));
                    }
                    _ => {
                        panic!("Unknown entry type {}", entity_type);
                    }
                }
            }
            entries.push(ScoreEntry {
                scoreboard_id,
                objective_name,
                score,
                entity_type,
                actor_unique_id,
                custom_name,
            });
        }

        SetScore {
            action_type,
            entries,
        }
    }

    fn debug(&self) {
        println!("Action type: {}", self.action_type);
        println!("Entries: {:?}", &self.entries);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl SetScore {
    pub const TYPE_CHANGE: u8 = 0;
    pub const TYPE_REMOVE: u8 = 1;
}
