use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundDataDrivenUICloseScreen {
    pub form_id: Option<u32>
}

impl Packet for ClientBoundDataDrivenUICloseScreen {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundDataDrivenUICloseScreen.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::write_optional(stream, &self.form_id, |s, v| s.put_u32_le(*v));
    }

    fn decode(stream: &mut Reader) -> ClientBoundDataDrivenUICloseScreen {
        let form_id = PacketSerializer::read_optional(stream, |s| s.get_u32_le());

        ClientBoundDataDrivenUICloseScreen { form_id}
    }
}
