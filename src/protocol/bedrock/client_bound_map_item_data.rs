use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct ClientBoundMapItemData {}

pub fn new() -> ClientBoundMapItemData {
    ClientBoundMapItemData { }
}

impl Packet for ClientBoundMapItemData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundMapItemData.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        // TODO

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(_bytes: Vec<u8>) -> ClientBoundMapItemData {
        //let mut stream = Stream::new(bytes, 0);

        // TODO

        ClientBoundMapItemData { }
    }

    fn debug(&self) {
        // TODO
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
