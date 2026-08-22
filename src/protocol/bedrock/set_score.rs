use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::score_entry::ScoreEntry;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetScore {
    pub entries: Vec<ScoreEntry>,
}

impl Packet for SetScore {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetScore.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.entries.len() as u32);
        for entry in &self.entries {
            stream.put_var_u32(entry.entity_type);
            PacketSerializer::put_string(stream, Self::ACTION_IDS[entry.entity_type as usize]);
            match entry.entity_type {
                Self::TYPE_REMOVE => {
                    stream.put_var_i64(entry.scoreboard_id);
                    PacketSerializer::write_optional(stream, &entry.objective_name, |s, v| PacketSerializer::put_string(s, v));
                },
                Self::TYPE_PLAYER| Self::TYPE_ENTITY => {
                    stream.put_var_i64(entry.scoreboard_id);
                    if let Some(objective_name) = &entry.objective_name {
                        PacketSerializer::put_string(stream, objective_name);
                    } else { panic!("objectiveName must be set for this entry type");}
                    stream.put_i32_le(entry.score);
                    PacketSerializer::put_actor_unique_id(stream, entry.actor_unique_id.expect("actorUniqueId must be set for this entry type"));
                },
                Self::TYPE_FAKE_PLAYER => {
                    stream.put_var_i64(entry.scoreboard_id);
                    if let Some(objective_name) = &entry.objective_name {
                        PacketSerializer::put_string(stream, objective_name);
                    } else { panic!("objectiveName must be set for this entry type");}
                    stream.put_i32_le(entry.score);
                    if let Some(custom_name) = &entry.custom_name {
                        PacketSerializer::put_string(stream, custom_name);
                    } else { panic!("customName must be set for this entry type");}
                },
                _ => {
                    panic!("Unknown entry type {}", entry.entity_type);
                }
            }
        }
    }

    fn decode(stream: &mut Reader) -> SetScore {
        let count = stream.get_var_u32();
        let mut entries: Vec<ScoreEntry> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let entity_type = stream.get_var_u32();
            let _ = PacketSerializer::get_string(stream); //actionId
            let entry = match entity_type {
                Self::TYPE_REMOVE => {
                    let scoreboard_id = stream.get_var_i64();
                    let objective_name = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));
                    ScoreEntry {
                        scoreboard_id,
                        objective_name,
                        score: 0,
                        entity_type,
                        actor_unique_id: None,
                        custom_name: None,
                    }
                },
                Self::TYPE_PLAYER| Self::TYPE_ENTITY => {
                    let scoreboard_id = stream.get_var_i64();
                    let objective_name = PacketSerializer::get_string(stream);
                    let score = stream.get_i32_le();
                    let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
                    ScoreEntry {
                        scoreboard_id,
                        objective_name: Some(objective_name),
                        score,
                        entity_type,
                        actor_unique_id: Some(actor_unique_id),
                        custom_name: None,
                    }
                },
                Self::TYPE_FAKE_PLAYER => {
                    let scoreboard_id = stream.get_var_i64();
                    let objective_name = PacketSerializer::get_string(stream);
                    let score = stream.get_i32_le();
                    let custom_name = PacketSerializer::get_string(stream);
                    ScoreEntry {
                        scoreboard_id,
                        objective_name: Some(objective_name),
                        score,
                        entity_type,
                        actor_unique_id: None,
                        custom_name: Some(custom_name),
                    }
                },
                _ => {
                    panic!("Unknown entry type {}", entity_type);
                }
            };
            entries.push(entry);
        }

        SetScore { entries }
    }
}

impl SetScore {
    pub const TYPE_REMOVE: u32 = 0;
    pub const TYPE_PLAYER: u32 = 1;
    pub const TYPE_ENTITY: u32 = 2;
    pub const TYPE_FAKE_PLAYER: u32 = 3;
    pub const ACTION_IDS: [&str; 4] = ["remove", "changeplayer", "changeentity", "changefakeplayer"];
}
