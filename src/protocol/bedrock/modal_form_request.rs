use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct ModalFormRequest {
    pub form_id: u32,
    pub form_data: String // json
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
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.form_id);
        PacketSerializer::put_string(&mut stream, self.form_data.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ModalFormRequest {
        let mut stream = Stream::new(bytes, 0);

        let form_id = stream.get_unsigned_var_int();
        let form_data = PacketSerializer::get_string(&mut stream);

        ModalFormRequest { form_id, form_data }
    }

    fn debug(&self) {
        println!("Form ID: {}", self.form_id);
        println!("Form Data: {}", self.form_data);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
