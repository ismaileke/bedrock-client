use std::any::Any;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;

pub struct RequestNetworkSettings {
    pub protocol_version: u32
}

pub fn new(protocol_version: u32) -> RequestNetworkSettings {
    RequestNetworkSettings{ protocol_version }
}

impl Packet for RequestNetworkSettings {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRequestNetworkSettings.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);
        stream.put_int(self.protocol_version);

        let mut main_stream = Stream::new(Vec::new(), 0);
        main_stream.put_byte(0xfe);
        main_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        main_stream.put(stream.get_buffer());
        main_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> RequestNetworkSettings {
        let mut stream = Stream::new(bytes, 0);

        let protocol_version = stream.get_int();

        RequestNetworkSettings { protocol_version }
    }

    fn debug(&self) {
        println!("Protocol Version: {}", self.protocol_version);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
