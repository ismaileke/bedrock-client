use crate::protocol::game::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;

pub struct ModalFormRequest {
    form_id: u32,
    form_data: String // json
}

pub fn new(form_id: u32, form_data: String) -> ModalFormRequest {
    ModalFormRequest { form_id, form_data }
}

impl ModalFormRequest {
    pub fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(BedrockPacketType::get_byte(BedrockPacketType::ModalFormRequest) as u32);

        stream.put_unsigned_var_int(self.form_id);
        stream.put_l_int(self.form_data.len() as u32);
        stream.put(self.form_data.clone().into_bytes());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }
}
