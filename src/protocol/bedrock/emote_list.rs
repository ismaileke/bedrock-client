use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct EmoteList {
    pub player_actor_runtime_id: u64,
    pub emote_ids: Vec<String>
}

pub fn new(player_actor_runtime_id: u64, emote_ids: Vec<String>) -> EmoteList {
    EmoteList { player_actor_runtime_id, emote_ids }
}

impl Packet for EmoteList {
    fn id(&self) -> u16 {
        BedrockPacketType::IDEmoteList.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.player_actor_runtime_id);
        stream.put_unsigned_var_int(self.emote_ids.len() as u32);
        for emote_id in self.emote_ids.iter() {
            PacketSerializer::put_uuid(&mut stream, emote_id.clone());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> EmoteList {
        let mut stream = Stream::new(bytes, 0);

        let player_actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let emote_ids_len = stream.get_unsigned_var_int() as usize;
        let mut emote_ids = Vec::new();
        for _ in 0..emote_ids_len {
            emote_ids.push(PacketSerializer::get_uuid(&mut stream));
        }

        EmoteList { player_actor_runtime_id, emote_ids }
    }

    fn debug(&self) {
        println!("Player Actor Runtime ID: {}", self.player_actor_runtime_id);
        println!("Emote IDs: {:?}", self.emote_ids);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
