use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::inventory::stack_response::item_stack_response_entry::ItemStackResponseEntry;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ItemStackResponse {
    pub responses: Vec<ItemStackResponseEntry>,
}

impl Packet for ItemStackResponse {
    fn id(&self) -> u16 {
        BedrockPacketType::IDItemStackResponse.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.responses.len() as u32);
        for response in self.responses.iter() {
            response.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ItemStackResponse {
        let response_count = stream.get_var_u32() as usize;
        let mut responses = Vec::new();
        for _ in 0..response_count {
            responses.push(ItemStackResponseEntry::read(stream));
        }

        ItemStackResponse { responses }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
