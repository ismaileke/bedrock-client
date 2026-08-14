use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct UpdateBlock {
    pub block_position: Vec<i32>,
    pub block_runtime_id: u32,
    pub flags: u32,
    pub data_layer_id: u32,
}

impl Packet for UpdateBlock {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateBlock.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_block_pos(stream, &self.block_position);
        stream.put_var_u32(self.block_runtime_id);
        stream.put_var_u32(self.flags);
        stream.put_var_u32(self.data_layer_id);
    }

    fn decode(stream: &mut Reader) -> UpdateBlock {
        let block_position = PacketSerializer::get_block_pos(stream);
        let block_runtime_id = stream.get_var_u32();
        let flags = stream.get_var_u32();
        let data_layer_id = stream.get_var_u32();

        UpdateBlock {
            block_position,
            block_runtime_id,
            flags,
            data_layer_id,
        }
    }
}

impl UpdateBlock {
    pub const FLAG_NONE: u32 = 0b0000;
    pub const FLAG_NEIGHBORS: u32 = 0b0001;
    pub const FLAG_NETWORK: u32 = 0b0010;
    pub const FLAG_NOGRAPHIC: u32 = 0b0100;
    pub const FLAG_PRIORITY: u32 = 0b1000;

    pub const DATA_LAYER_NORMAL: u32 = 0;
    pub const DATA_LAYER_LIQUID: u32 = 1;
}
