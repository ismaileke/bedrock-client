use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct CommandOutput {}

pub fn new() -> CommandOutput {
    CommandOutput { }
}

impl Packet for CommandOutput {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCommandOutput.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        // TODO

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(_bytes: Vec<u8>) -> CommandOutput {
        //let mut stream = Stream::new(bytes, 0);

        // TODO

        CommandOutput { }
    }

    fn debug(&self) {
        // TODO
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
