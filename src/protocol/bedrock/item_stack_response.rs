use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::stack_response::item_stack_response_entry::ItemStackResponseEntry;

pub struct ItemStackResponse {
    pub responses: Vec<ItemStackResponseEntry>
}

pub fn new(responses: Vec<ItemStackResponseEntry>) -> ItemStackResponse {
    ItemStackResponse { responses }
}

impl Packet for ItemStackResponse {
    fn id(&self) -> u16 {
        BedrockPacketType::IDItemStackResponse.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.responses.len() as u32);
        for response in self.responses.iter() {
            response.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ItemStackResponse {
        let mut stream = Stream::new(bytes, 0);

        let response_count = stream.get_unsigned_var_int() as usize;
        let mut responses = Vec::new();
        for _ in 0..response_count {
            responses.push(ItemStackResponseEntry::read(&mut stream));
        }

        ItemStackResponse { responses }
    }

    fn debug(&self) {
        println!("Responses: {:?}", self.responses);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
