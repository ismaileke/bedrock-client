use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::player_list_entry::PlayerListEntry;
use crate::utils::color::Color;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerList {
    pub entries: Vec<PlayerListEntry>,
}

impl Packet for PlayerList {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerList.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.entries.len() as u32);
        for entry in self.entries.iter_mut() {
            Self::player_list_action(stream, &mut entry.action_type);
            PacketSerializer::put_uuid(stream, &entry.uuid);
            if entry.action_type == Self::TYPE_REMOVE { continue; }
            PacketSerializer::put_actor_unique_id(stream, entry.actor_unique_id);
            PacketSerializer::put_string(stream, &entry.username);
            PacketSerializer::put_string(stream, &entry.xbox_user_id);
            PacketSerializer::put_string(stream, &entry.platform_chat_id);
            stream.put_i32_le(entry.build_platform);
            PacketSerializer::put_skin(stream, &entry.skin_data);
            stream.put_bool(entry.is_teacher);
            stream.put_bool(entry.is_host);
            stream.put_bool(entry.is_sub_client);
            stream.put_u32_le(entry.color.to_argb());
        }
    }

    fn decode(stream: &mut Reader) -> PlayerList {
        let count = stream.get_var_u32();
        let mut entries: Vec<PlayerListEntry> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let _variant = stream.get_var_u32();
            let action_type = stream.get_u8();
            let uuid = PacketSerializer::get_uuid(stream);
            let entry = if action_type == Self::TYPE_ADD {
                PlayerListEntry {
                    action_type,
                    uuid,
                    actor_unique_id: PacketSerializer::get_actor_unique_id(stream),
                    username: PacketSerializer::get_string(stream),
                    xbox_user_id: PacketSerializer::get_string(stream),
                    platform_chat_id: PacketSerializer::get_string(stream),
                    build_platform: stream.get_i32_le(),
                    skin_data: PacketSerializer::get_skin(stream),
                    is_teacher: stream.get_bool(),
                    is_host: stream.get_bool(),
                    is_sub_client: stream.get_bool(),
                    color: Color::from_argb(stream.get_u32_le()),
                }
            } else if action_type == Self::TYPE_REMOVE {
                PlayerListEntry::create_removal_entry(uuid)
            } else {
                panic!("Unknown player list entry action type {}", action_type);
            };
            entries.push(entry);
        }
        PlayerList { entries }
    }
}

impl PlayerList {
    pub const TYPE_REMOVE: u8 = 0;
    pub const TYPE_ADD: u8 = 1;

    pub fn player_list_action(stream: &mut Writer, action: &mut u8) {
        let mut variant: u32 = 0;
        if *action == Self::TYPE_ADD {
            variant = 1
        }
        stream.put_var_u32(variant);
        let legacy_action = *action;
        stream.put_u8(legacy_action);
        *action = Self::TYPE_REMOVE;
        if variant == 1 {
            *action = Self::TYPE_ADD;
        } else if variant != 0 {
            panic!("unknown player list entry variant {}", variant);
        }
    }
}
