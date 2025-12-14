use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ModalFormRequest {
    pub form_id: u32,
    pub form_data: String, // json
}

pub fn new(form_id: u32, form_data: String) -> ModalFormRequest {
    ModalFormRequest { form_id, form_data }
}

impl Packet for ModalFormRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDModalFormRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.form_id);
        PacketSerializer::put_string(&mut stream, self.form_data.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ModalFormRequest {
        let form_id = stream.get_var_u32();
        let form_data = PacketSerializer::get_string(stream);

        ModalFormRequest { form_id, form_data }
    }

    fn debug(&self) {
        println!("Form ID: {}", self.form_id);
        println!("Form Data: {}", self.form_data);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
