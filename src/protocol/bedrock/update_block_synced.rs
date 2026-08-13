use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct UpdateBlockSynced {
    pub block_position: Vec<i32>,
    pub block_runtime_id: u32,
    pub flags: u32,
    pub layer: u32,
    pub actor_unique_id: u64,
    pub update_type: u64,
}

impl Packet for UpdateBlockSynced {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateBlockSynced.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_block_pos(stream, self.block_position.clone());
        stream.put_var_u32(self.block_runtime_id);
        stream.put_var_u32(self.flags);
        stream.put_var_u32(self.layer);
        stream.put_var_u64(self.actor_unique_id);
        stream.put_var_u64(self.update_type);
    }

    fn decode(stream: &mut Reader) -> UpdateBlockSynced {
        let block_position = PacketSerializer::get_block_pos(stream);
        let block_runtime_id = stream.get_var_u32();
        let flags = stream.get_var_u32();
        let layer = stream.get_var_u32();
        let actor_unique_id = stream.get_var_u64();
        let update_type = stream.get_var_u64();

        UpdateBlockSynced {
            block_position,
            block_runtime_id,
            flags,
            layer,
            actor_unique_id,
            update_type,
        }
    }
}

impl UpdateBlockSynced {
    pub const TYPE_NONE: u64 = 0;
    pub const TYPE_CREATE: u64 = 1;
    pub const TYPE_DESTROY: u64 = 2;
}
