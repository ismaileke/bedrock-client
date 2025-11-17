
use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct ModalFormResponse {
    pub form_id: u32,
    pub form_data: Option<String>,
    pub cancel_reason: Option<u8>
}

pub fn new(form_id: u32, form_data: Option<String>, cancel_reason: Option<u8>) -> ModalFormResponse {
    ModalFormResponse { form_id, form_data, cancel_reason }
}

impl Packet for ModalFormResponse {
    fn id(&self) -> u16 {
        BedrockPacketType::IDModalFormResponse.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.form_id);
        PacketSerializer::write_optional(&mut stream, &self.form_data, |s, v| PacketSerializer::put_string(s, v.clone()));
        PacketSerializer::write_optional(&mut stream, &self.cancel_reason, |s, v| s.put_byte(*v));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> ModalFormResponse {
        let mut stream = Stream::new(bytes, 0);

        let form_id = stream.get_var_u32();
        let form_data = PacketSerializer::read_optional(&mut stream, |s| PacketSerializer::get_string(s));
        let cancel_reason = PacketSerializer::read_optional(&mut stream, |s| s.get_byte());

        ModalFormResponse { form_id, form_data, cancel_reason }
    }

    fn debug(&self) {
        println!("Form ID: {}", self.form_id);
        println!("Form Data: {:?}", self.form_data);
        println!("Cancel Reason: {:?}", self.cancel_reason);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ModalFormResponse {
    pub const CANCEL_REASON_CLOSED: u8 = 0;
    /** Sent if a form is sent when the player is on a loading screen */
    pub const CANCEL_REASON_USER_BUSY: u8 = 1;
}
