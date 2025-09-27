use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct LecternUpdate {
    pub page: u8,
    pub total_pages: u8,
    pub block_position: Vec<i32>
}

pub fn new(page: u8, total_pages: u8, block_position: Vec<i32>) -> LecternUpdate {
    LecternUpdate { page, total_pages, block_position }
}

impl Packet for LecternUpdate {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLecternUpdate.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_byte(self.page);
        stream.put_byte(self.total_pages);
        PacketSerializer::put_block_pos(&mut stream, self.block_position.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> LecternUpdate {
        let mut stream = Stream::new(bytes, 0);

        let page = stream.get_byte();
        let total_pages = stream.get_byte();
        let block_position = PacketSerializer::get_block_pos(&mut stream);

        LecternUpdate { page, total_pages, block_position }
    }

    fn debug(&self) {
        println!("Page: {}", self.page);
        println!("Total Pages: {}", self.total_pages);
        println!("Block Position: {:?}", self.block_position);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
