use std::any::Any;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;

pub struct GraphicsOverrideParameter {}

pub fn new() -> GraphicsOverrideParameter {
    GraphicsOverrideParameter {}
}

impl Packet for GraphicsOverrideParameter {
    fn id(&self) -> u16 {
        BedrockPacketType::IDGraphicsOverrideParameter.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        // No payload

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(_bytes: Vec<u8>) -> GraphicsOverrideParameter {
        //let mut stream = Stream::new(bytes, 0);

        // No Payload
        GraphicsOverrideParameter {}
    }

    fn debug(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
