use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PartyDestinationCookieResponse {
    pub cookie: String,
    pub accepted: bool,
}

impl Packet for PartyDestinationCookieResponse {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPartyDestinationCookieResponse.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.cookie.clone());
        stream.put_bool(self.accepted);
    }

    fn decode(stream: &mut Reader) -> PartyDestinationCookieResponse {
        let cookie = PacketSerializer::get_string(stream);
        let accepted = stream.get_bool();

        PartyDestinationCookieResponse { cookie, accepted }
    }
}
