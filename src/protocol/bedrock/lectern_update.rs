use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct LecternUpdate {
    pub page: u8,
    pub total_pages: u8,
    pub block_position: Vec<i32>,
}

pub fn new(page: u8, total_pages: u8, block_position: Vec<i32>) -> LecternUpdate {
    LecternUpdate {
        page,
        total_pages,
        block_position,
    }
}

impl Packet for LecternUpdate {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLecternUpdate.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.page);
        stream.put_byte(self.total_pages);
        PacketSerializer::put_block_pos(&mut stream, self.block_position.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> LecternUpdate {
        let page = stream.get_byte();
        let total_pages = stream.get_byte();
        let block_position = PacketSerializer::get_block_pos(stream);

        LecternUpdate {
            page,
            total_pages,
            block_position,
        }
    }

    fn debug(&self) {
        println!("Page: {}", self.page);
        println!("Total Pages: {}", self.total_pages);
        println!("Block Position: {:?}", self.block_position);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
