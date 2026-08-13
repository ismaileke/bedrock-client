use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SendPartyDestinationCookie {
    pub cookie: String,
    pub intent: String,
    pub destination_name: String,
}

impl Packet for SendPartyDestinationCookie {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSendPartyDestinationCookie.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.cookie.clone());
        PacketSerializer::put_string(stream, self.intent.clone());
        PacketSerializer::put_string(stream, self.destination_name.clone());
    }

    fn decode(stream: &mut Reader) -> SendPartyDestinationCookie {
        let cookie = PacketSerializer::get_string(stream);
        let intent = PacketSerializer::get_string(stream);
        let destination_name = PacketSerializer::get_string(stream);

        SendPartyDestinationCookie { cookie, intent, destination_name }
    }
}
