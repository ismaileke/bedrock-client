use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct CodeBuilder {
    pub url: String,
    pub open_code_builder: bool
}

pub fn new(url: String, open_code_builder: bool) -> CodeBuilder {
    CodeBuilder { url, open_code_builder }
}

impl Packet for CodeBuilder {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCodeBuilder.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.url.clone());
        stream.put_bool(self.open_code_builder);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> CodeBuilder {
        let mut stream = Stream::new(bytes, 0);

        let url = PacketSerializer::get_string(&mut stream);
        let open_code_builder = stream.get_bool();

        CodeBuilder { url, open_code_builder }
    }

    fn debug(&self) {
        println!("URL: {}", self.url);
        println!("Open Code Builder: {}", self.open_code_builder);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
