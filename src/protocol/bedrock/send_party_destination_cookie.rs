use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct SendPartyDestinationCookie {
    pub cookie: String,
    pub intent: String,
    pub destination_name: String,
}

impl Packet for SendPartyDestinationCookie {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSendPartyDestinationCookie.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.cookie.clone());
        PacketSerializer::put_string(&mut stream, self.intent.clone());
        PacketSerializer::put_string(&mut stream, self.destination_name.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SendPartyDestinationCookie {
        let cookie = PacketSerializer::get_string(stream);
        let intent = PacketSerializer::get_string(stream);
        let destination_name = PacketSerializer::get_string(stream);

        SendPartyDestinationCookie { cookie, intent, destination_name }
    }
}
