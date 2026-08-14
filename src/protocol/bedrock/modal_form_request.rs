use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ModalFormRequest {
    pub form_id: u32,
    pub form_data: String, // json
}

impl Packet for ModalFormRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDModalFormRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.form_id);
        PacketSerializer::put_string(stream, &self.form_data);
    }

    fn decode(stream: &mut Reader) -> ModalFormRequest {
        let form_id = stream.get_var_u32();
        let form_data = PacketSerializer::get_string(stream);

        ModalFormRequest { form_id, form_data }
    }
}
