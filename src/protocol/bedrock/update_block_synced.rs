use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct UpdateBlockSynced {
    pub block_position: Vec<i32>,
    pub block_runtime_id: u32,
    pub flags: u32,
    pub layer: u32,
    pub actor_unique_id: u64,
    pub update_type: u64
}

pub fn new(block_position: Vec<i32>, block_runtime_id: u32, flags: u32, layer: u32, actor_unique_id: u64, update_type: u64) -> UpdateBlockSynced {
    UpdateBlockSynced { block_position, block_runtime_id, flags, layer, actor_unique_id, update_type }
}

impl Packet for UpdateBlockSynced {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateBlockSynced.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_block_pos(&mut stream, self.block_position.clone());
        stream.put_var_u32(self.block_runtime_id);
        stream.put_var_u32(self.flags);
        stream.put_var_u32(self.layer);
        stream.put_var_u64(self.actor_unique_id);
        stream.put_var_u64(self.update_type);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> UpdateBlockSynced {
        let block_position = PacketSerializer::get_block_pos(stream);
        let block_runtime_id = stream.get_var_u32();
        let flags = stream.get_var_u32();
        let layer = stream.get_var_u32();
        let actor_unique_id = stream.get_var_u64();
        let update_type = stream.get_var_u64();

        UpdateBlockSynced { block_position, block_runtime_id, flags, layer, actor_unique_id, update_type }
    }

    fn debug(&self) {
        println!("Block Position: {:?}", self.block_position);
        println!("Block Runtime ID: {}", self.block_runtime_id);
        println!("Flags: {}", self.flags);
        println!("Layer: {}", self.layer);
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("Actor Sync Message: {}", self.update_type);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl UpdateBlockSynced {
    pub const TYPE_NONE: u64 = 0;
    pub const TYPE_CREATE: u64 = 1;
    pub const TYPE_DESTROY: u64 = 2;
}
