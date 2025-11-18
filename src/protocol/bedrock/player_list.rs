use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::player_list_entry::PlayerListEntry;
use crate::utils::color::Color;

pub struct PlayerList {
    pub list_type: u8,
    pub entries: Vec<PlayerListEntry>
}

fn new(list_type: u8, entries: Vec<PlayerListEntry>) -> PlayerList {
    PlayerList { list_type, entries }
}

pub fn add(entries: Vec<PlayerListEntry>) -> PlayerList {
    new(PlayerList::TYPE_ADD, entries)
}

pub fn remove(entries: Vec<PlayerListEntry>) -> PlayerList {
    new(PlayerList::TYPE_REMOVE, entries)
}

impl Packet for PlayerList {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerList.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.list_type);
        stream.put_var_u32(self.entries.len() as u32);
        for entry in self.entries.iter() {
            if self.list_type == Self::TYPE_ADD {
                PacketSerializer::put_uuid(&mut stream, entry.uuid.clone());
                PacketSerializer::put_actor_unique_id(&mut stream, entry.actor_unique_id);
                PacketSerializer::put_string(&mut stream, entry.username.clone());
                PacketSerializer::put_string(&mut stream, entry.xbox_user_id.clone());
                PacketSerializer::put_string(&mut stream, entry.platform_chat_id.clone());
                stream.put_i32_le(entry.build_platform);
                PacketSerializer::put_skin(&mut stream, &entry.skin_data);
                stream.put_bool(entry.is_teacher);
                stream.put_bool(entry.is_host);
                stream.put_bool(entry.is_sub_client);
                stream.put_u32_le(entry.color.unwrap_or(Color::new(255, 255, 255, 255)).to_argb());
            } else {
                PacketSerializer::put_uuid(&mut stream, entry.uuid.clone());
            }
        }
        if self.list_type == Self::TYPE_ADD {
            for entry in self.entries.iter() {
                stream.put_bool(entry.skin_data.is_verified);
            }
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> PlayerList {
        let mut stream = Stream::new(bytes, 0);

        let list_type = stream.get_byte();
        let count = stream.get_var_u32();
        let mut entries: Vec<PlayerListEntry> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let mut player_list_entry = PlayerListEntry::create_removal_entry(PacketSerializer::get_uuid(&mut stream));
            if list_type == Self::TYPE_ADD {
                player_list_entry = PlayerListEntry::create_addition_entry(
                    PacketSerializer::get_uuid(&mut stream),
                    PacketSerializer::get_actor_unique_id(&mut stream),
                    PacketSerializer::get_string(&mut stream),
                    PacketSerializer::get_skin(&mut stream),
                    PacketSerializer::get_string(&mut stream),
                    PacketSerializer::get_string(&mut stream),
                    stream.get_i32_le(),
                    stream.get_bool(),
                    stream.get_bool(),
                    stream.get_bool(),
                    Some(Color::from_argb(stream.get_u32_le()))
                );
            }
            entries.push(player_list_entry);
        }

        PlayerList { list_type, entries }
    }

    fn debug(&self) {
        println!("List Type: {}", self.list_type);
        println!("Entries: {:?}", self.entries);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PlayerList {
    pub const TYPE_ADD: u8 = 0;
    pub const TYPE_REMOVE: u8 = 1;
}
