use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct UpdateBlock {
    pub block_position: Vec<i32>,
    pub block_runtime_id: u32,
    pub flags: u32,
    pub data_layer_id: u32
}

pub fn new(block_position: Vec<i32>, block_runtime_id: u32, flags: u32, data_layer_id: u32) -> UpdateBlock {
    UpdateBlock { block_position, block_runtime_id, flags, data_layer_id }
}

impl Packet for UpdateBlock {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateBlock.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_block_pos(&mut stream, self.block_position.clone());
        stream.put_var_u32(self.block_runtime_id);
        stream.put_var_u32(self.flags);
        stream.put_var_u32(self.data_layer_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> UpdateBlock {
        let mut stream = Stream::new(bytes, 0);

        let block_position = PacketSerializer::get_block_pos(&mut stream);
        let block_runtime_id = stream.get_var_u32();
        let flags = stream.get_var_u32();
        let data_layer_id = stream.get_var_u32();

        UpdateBlock { block_position, block_runtime_id, flags, data_layer_id }
    }

    fn debug(&self) {
        println!("Block Position: {:?}", self.block_position);
        println!("Block Runtime ID: {}", self.block_runtime_id);
        println!("Flags: {}", self.flags);
        println!("Data Layer ID: {}", self.data_layer_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
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
