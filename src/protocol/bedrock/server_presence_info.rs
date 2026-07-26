use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::presence_info::PresenceInfo;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct ServerPresenceInfo {
    pub presence_info: Option<PresenceInfo>,
}

impl Packet for ServerPresenceInfo {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerPresenceInfo.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::write_optional(&mut stream, &self.presence_info, |s, v| v.write(s));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ServerPresenceInfo {
        let presence_info = PacketSerializer::read_optional(stream, |s| PresenceInfo::read(s));
        ServerPresenceInfo { presence_info }
    }
}
