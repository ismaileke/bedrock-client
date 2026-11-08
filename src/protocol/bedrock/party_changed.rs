use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::party_info::PartyInfo;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PartyChanged {
    pub party_info: Option<PartyInfo>,
}

impl Packet for PartyChanged {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPartyChanged.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::write_optional(stream, &self.party_info, |s, v| v.write(s))
    }

    fn decode(stream: &mut Reader) -> PartyChanged {
        let party_info = PacketSerializer::read_optional(stream, |s| PartyInfo::read(s));

        PartyChanged { party_info }
    }
}
