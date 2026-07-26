use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct PartyChanged {
    pub party_id: String,
    pub party_leader: bool,
}

impl Packet for PartyChanged {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPartyChanged.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.party_id.clone());
        stream.put_bool(self.party_leader);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PartyChanged {
        let party_id = PacketSerializer::get_string(stream);
        let party_leader = stream.get_bool();

        PartyChanged { party_id, party_leader }
    }
}
