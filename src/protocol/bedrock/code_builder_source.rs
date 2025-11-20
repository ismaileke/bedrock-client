use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct CodeBuilderSource {
    pub operation: u8,
    pub category: u8,
    pub code_status: u8
}

pub fn new(operation: u8, category: u8, code_status: u8) -> CodeBuilderSource {
    CodeBuilderSource { operation, category, code_status }
}

impl Packet for CodeBuilderSource {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCodeBuilderSource.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.operation);
        stream.put_byte(self.category);
        stream.put_byte(self.code_status);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CodeBuilderSource {
        let operation = stream.get_byte();
        let category = stream.get_byte();
        let code_status = stream.get_byte();

        CodeBuilderSource { operation, category, code_status }
    }

    fn debug(&self) {
        println!("Operation: {}", self.operation);
        println!("Category: {}", self.category);
        println!("Code Status: {}", self.code_status);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
