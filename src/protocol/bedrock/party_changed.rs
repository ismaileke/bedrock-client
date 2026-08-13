use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PartyChanged {
    pub party_id: String,
    pub party_leader: bool,
}

impl Packet for PartyChanged {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPartyChanged.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.party_id.clone());
        stream.put_bool(self.party_leader);
    }

    fn decode(stream: &mut Reader) -> PartyChanged {
        let party_id = PacketSerializer::get_string(stream);
        let party_leader = stream.get_bool();

        PartyChanged { party_id, party_leader }
    }
}
