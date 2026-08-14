use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::player_list_entry::PlayerListEntry;
use crate::utils::color::Color;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerList {
    pub list_type: u8,
    pub entries: Vec<PlayerListEntry>,
}

impl PlayerList {
    pub fn add(entries: Vec<PlayerListEntry>) -> PlayerList {
        PlayerList{ list_type: PlayerList::TYPE_ADD, entries }
    }

    pub fn remove(entries: Vec<PlayerListEntry>) -> PlayerList {
        PlayerList{ list_type: PlayerList::TYPE_REMOVE, entries }
    }
}

impl Packet for PlayerList {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerList.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.list_type);
        stream.put_var_u32(self.entries.len() as u32);
        for entry in self.entries.iter() {
            if self.list_type == Self::TYPE_ADD {
                PacketSerializer::put_uuid(stream, &entry.uuid);
                PacketSerializer::put_actor_unique_id(stream, entry.actor_unique_id);
                PacketSerializer::put_string(stream, &entry.username);
                PacketSerializer::put_string(stream, &entry.xbox_user_id);
                PacketSerializer::put_string(stream, &entry.platform_chat_id);
                stream.put_i32_le(entry.build_platform);
                PacketSerializer::put_skin(stream, &entry.skin_data);
                stream.put_bool(entry.is_teacher);
                stream.put_bool(entry.is_host);
                stream.put_bool(entry.is_sub_client);
                stream.put_u32_le(entry.color.unwrap_or(Color::new(255, 255, 255, 255)).to_argb());
            } else {
                PacketSerializer::put_uuid(stream, &entry.uuid);
            }
        }
        if self.list_type == Self::TYPE_ADD {
            for entry in self.entries.iter() {
                stream.put_bool(entry.skin_data.is_verified);
            }
        }
    }

    fn decode(stream: &mut Reader) -> PlayerList {
        let list_type = stream.get_u8();
        let count = stream.get_var_u32();
        let mut entries: Vec<PlayerListEntry> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let uuid = PacketSerializer::get_uuid(stream);
            let mut player_list_entry = PlayerListEntry::create_removal_entry(uuid.clone());
            if list_type == Self::TYPE_ADD {
                player_list_entry = PlayerListEntry::create_addition_entry(
                    uuid,
                    PacketSerializer::get_actor_unique_id(stream),
                    PacketSerializer::get_string(stream),
                    PacketSerializer::get_string(stream),
                    PacketSerializer::get_string(stream),
                    stream.get_i32_le(),
                    PacketSerializer::get_skin(stream),
                    stream.get_bool(),
                    stream.get_bool(),
                    stream.get_bool(),
                    Some(Color::from_argb(stream.get_u32_le())),
                );
            }
            entries.push(player_list_entry);
        }

        PlayerList { list_type, entries }
    }
}

impl PlayerList {
    pub const TYPE_ADD: u8 = 0;
    pub const TYPE_REMOVE: u8 = 1;
}
