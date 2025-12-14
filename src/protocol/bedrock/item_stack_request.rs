use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_entry::ItemStackRequestEntry;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ItemStackRequest {
    pub requests: Vec<ItemStackRequestEntry>,
}

pub fn new(requests: Vec<ItemStackRequestEntry>) -> ItemStackRequest {
    ItemStackRequest { requests }
}

impl Packet for ItemStackRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDItemStackRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.requests.len() as u32);
        for request in self.requests.iter_mut() {
            request.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ItemStackRequest {
        let request_count = stream.get_var_u32() as usize;
        let mut requests = Vec::new();
        for _ in 0..request_count {
            requests.push(ItemStackRequestEntry::read(stream));
        }

        ItemStackRequest { requests }
    }

    fn debug(&self) {
        println!("Requests: {:?}", self.requests);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
