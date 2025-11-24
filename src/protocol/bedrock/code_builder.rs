use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
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
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.url.clone());
        stream.put_bool(self.open_code_builder);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CodeBuilder {
        let url = PacketSerializer::get_string(stream);
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

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
