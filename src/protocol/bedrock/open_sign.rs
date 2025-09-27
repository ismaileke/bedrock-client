use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct OpenSign {
    pub block_position: Vec<i32>,
    pub front: bool
}

pub fn new(block_position: Vec<i32>, front: bool) -> OpenSign {
    OpenSign { block_position, front }
}

impl Packet for OpenSign {
    fn id(&self) -> u16 {
        BedrockPacketType::IDOpenSign.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_block_pos(&mut stream, self.block_position.clone());
        stream.put_bool(self.front);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> OpenSign {
        let mut stream = Stream::new(bytes, 0);

        let block_position = PacketSerializer::get_block_pos(&mut stream);
        let front = stream.get_bool();

        OpenSign { block_position, front }
    }

    fn debug(&self) {
        println!("Block Position: {:?}", self.block_position);
        println!("Front: {}", self.front);   
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
