use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;

pub struct UpdateEquip {
    pub window_id: u8,
    pub window_type: u8,
    pub window_slot_count: i32,
    pub actor_unique_id: i64,
    pub nbt: CacheableNBT
}

pub fn new(window_id: u8, window_type: u8, window_slot_count: i32, actor_unique_id: i64, nbt: CacheableNBT) -> UpdateEquip {
    UpdateEquip { window_id, window_type, window_slot_count, actor_unique_id, nbt }
}

impl Packet for UpdateEquip {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateEquip.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_byte(self.window_id);
        stream.put_byte(self.window_type);
        stream.put_var_int(self.window_slot_count);
        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);
        stream.put(self.nbt.get_encoded_nbt());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> UpdateEquip {
        let mut stream = Stream::new(bytes, 0);

        let window_id = stream.get_byte();
        let window_type = stream.get_byte();
        let window_slot_count = stream.get_var_int();
        let actor_unique_id = PacketSerializer::get_actor_unique_id(&mut stream);
        let nbt = CacheableNBT::new(Box::new(PacketSerializer::get_nbt_compound_root(&mut stream)));

        UpdateEquip { window_id, window_type, window_slot_count, actor_unique_id, nbt }
    }

    fn debug(&self) {
        println!("Window ID: {}", self.window_id);
        println!("Window Type: {}", self.window_type);
        println!("Window Slot Count: {}", self.window_slot_count);
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("NBT: {:?}", self.nbt);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
