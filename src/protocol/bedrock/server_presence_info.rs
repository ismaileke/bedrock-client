use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::presence_info::PresenceInfo;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerPresenceInfo {
    pub presence_info: Option<PresenceInfo>,
}

impl Packet for ServerPresenceInfo {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerPresenceInfo.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::write_optional(stream, &self.presence_info, |s, v| v.write(s));
    }

    fn decode(stream: &mut Reader) -> ServerPresenceInfo {
        let presence_info = PacketSerializer::read_optional(stream, |s| PresenceInfo::read(s));
        ServerPresenceInfo { presence_info }
    }
}
