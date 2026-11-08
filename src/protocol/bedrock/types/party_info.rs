use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PartyInfo {
    pub party_id: String,
    pub party_leader: bool
}

impl PartyInfo {
    pub fn new(party_id: String, party_leader: bool) -> PartyInfo {
        PartyInfo { party_id, party_leader }
    }

    pub fn read(stream: &mut Reader) -> PartyInfo {
        let party_id = PacketSerializer::get_string(stream);
        let party_leader = stream.get_bool();

        PartyInfo { party_id, party_leader }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.party_id);
        stream.put_bool(self.party_leader);
    }
}
