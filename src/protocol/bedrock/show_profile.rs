use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ShowProfile {
    pub xuid: String,
}

impl Packet for ShowProfile {
    fn id(&self) -> u16 {
        BedrockPacketType::IDShowProfile.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.xuid);
    }

    fn decode(stream: &mut Reader) -> ShowProfile {
        let xuid = PacketSerializer::get_string(stream);

        ShowProfile { xuid }
    }
}
