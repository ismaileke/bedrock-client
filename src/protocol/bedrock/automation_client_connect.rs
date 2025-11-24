use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
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
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.server_uri.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> AutomationClientConnect {
        let server_uri = PacketSerializer::get_string(stream);

        AutomationClientConnect { server_uri }
    }

    fn debug(&self) {
        println!("Server URI: {}", self.server_uri);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
