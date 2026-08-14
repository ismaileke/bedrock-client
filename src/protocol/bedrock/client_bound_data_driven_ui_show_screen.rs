use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundDataDrivenUIShowScreen {
    pub screen_id: String,
    pub form_id: u32,
    pub data_instance_id: Option<u32>
}

impl Packet for ClientBoundDataDrivenUIShowScreen {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundDataDrivenUIShowScreen.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.screen_id);
        stream.put_u32_le(self.form_id);
        PacketSerializer::write_optional(stream, &self.data_instance_id, |s, v| s.put_u32_le(*v));
    }

    fn decode(stream: &mut Reader) -> ClientBoundDataDrivenUIShowScreen {
        let screen_id = PacketSerializer::get_string(stream);
        let form_id = stream.get_u32_le();
        let data_instance_id = PacketSerializer::read_optional(stream, |s| s.get_u32_le());

        ClientBoundDataDrivenUIShowScreen { screen_id, form_id, data_instance_id }
    }
}
