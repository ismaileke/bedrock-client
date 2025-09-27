use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct UpdateBlockSynced {
    pub block_position: Vec<i32>,
    pub block_runtime_id: u32,
    pub flags: u32,
    pub layer: u32,
    pub actor_unique_id: u64,
    pub actor_sync_message: u64
}

pub fn new(block_position: Vec<i32>, block_runtime_id: u32, flags: u32, layer: u32, actor_unique_id: u64, actor_sync_message: u64) -> UpdateBlockSynced {
    UpdateBlockSynced { block_position, block_runtime_id, flags, layer, actor_unique_id, actor_sync_message }
}

impl Packet for UpdateBlockSynced {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateBlockSynced.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_block_pos(&mut stream, self.block_position.clone());
        stream.put_unsigned_var_int(self.block_runtime_id);
        stream.put_unsigned_var_int(self.flags);
        stream.put_unsigned_var_int(self.layer);
        stream.put_unsigned_var_long(self.actor_unique_id);
        stream.put_unsigned_var_long(self.actor_sync_message);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> UpdateBlockSynced {
        let mut stream = Stream::new(bytes, 0);

        let block_position = PacketSerializer::get_block_pos(&mut stream);
        let block_runtime_id = stream.get_unsigned_var_int();
        let flags = stream.get_unsigned_var_int();
        let layer = stream.get_unsigned_var_int();
        let actor_unique_id = stream.get_unsigned_var_long();
        let actor_sync_message = stream.get_unsigned_var_long();

        UpdateBlockSynced { block_position, block_runtime_id, flags, layer, actor_unique_id, actor_sync_message }
    }

    fn debug(&self) {
        println!("Block Position: {:?}", self.block_position);
        println!("Block Runtime ID: {}", self.block_runtime_id);
        println!("Flags: {}", self.flags);
        println!("Layer: {}", self.layer);
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("Actor Sync Message: {}", self.actor_sync_message);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
