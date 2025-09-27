use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct AutomationClientConnect {
    pub server_uri: String
}

pub fn new(server_uri: String) -> AutomationClientConnect {
    AutomationClientConnect { server_uri }
}

impl Packet for AutomationClientConnect {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAutomationClientConnect.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.server_uri.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> AutomationClientConnect {
        let mut stream = Stream::new(bytes, 0);

        let server_uri = PacketSerializer::get_string(&mut stream);

        AutomationClientConnect { server_uri }
    }

    fn debug(&self) {
        println!("Server URI: {}", self.server_uri);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
