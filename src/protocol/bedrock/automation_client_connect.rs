use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AutomationClientConnect {
    pub server_uri: String,
}

impl Packet for AutomationClientConnect {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAutomationClientConnect.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.server_uri.clone());
    }

    fn decode(stream: &mut Reader) -> AutomationClientConnect {
        let server_uri = PacketSerializer::get_string(stream);

        AutomationClientConnect { server_uri }
    }
}
