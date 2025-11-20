use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct BlockPickRequest {
    pub block_position: Vec<i32>,
    pub add_user_data: bool,
    pub hotbar_slot: u8
}

pub fn new(block_position: Vec<i32>, add_user_data: bool, hotbar_slot: u8) -> BlockPickRequest {
    BlockPickRequest { block_position, add_user_data, hotbar_slot }
}

impl Packet for BlockPickRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDBlockPickRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_signed_block_pos(&mut stream, self.block_position.clone());
        stream.put_bool(self.add_user_data);
        stream.put_byte(self.hotbar_slot);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> BlockPickRequest {
        let block_position = PacketSerializer::get_signed_block_pos(stream);
        let add_user_data = stream.get_bool();
        let hotbar_slot = stream.get_byte();

        BlockPickRequest { block_position, add_user_data, hotbar_slot }
    }

    fn debug(&self) {
        println!("Block Position: {:?}", self.block_position);
        println!("Add User Data: {}", self.add_user_data);
        println!("Hotbar Slot: {}", self.hotbar_slot);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
