use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ModalFormResponse {
    pub form_id: u32,
    pub form_data: Option<String>,
    pub cancel_reason: Option<u8>,
}

impl Packet for ModalFormResponse {
    fn id(&self) -> u16 {
        BedrockPacketType::IDModalFormResponse.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.form_id);
        PacketSerializer::write_optional(stream, &self.form_data, |s, v| PacketSerializer::put_string(s, v));
        PacketSerializer::write_optional(stream, &self.cancel_reason, |s, v| s.put_u8(*v));
    }

    fn decode(stream: &mut Reader) -> ModalFormResponse {
        let form_id = stream.get_var_u32();
        let form_data =
            PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));
        let cancel_reason = PacketSerializer::read_optional(stream, |s| s.get_u8());

        ModalFormResponse {
            form_id,
            form_data,
            cancel_reason,
        }
    }
}

impl ModalFormResponse {
    pub const CANCEL_REASON_CLOSED: u8 = 0;
    /** Sent if a form is sent when the player is on a loading screen */
    pub const CANCEL_REASON_USER_BUSY: u8 = 1;
}
