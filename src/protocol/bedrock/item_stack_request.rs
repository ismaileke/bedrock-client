use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_entry::ItemStackRequestEntry;

pub struct ItemStackRequest {
    pub requests: Vec<ItemStackRequestEntry>
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
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.requests.len() as u32);
        for request in self.requests.iter_mut() {
            request.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ItemStackRequest {
        let mut stream = Stream::new(bytes, 0);

        let request_count = stream.get_unsigned_var_int() as usize;
        let mut requests = Vec::new();
        for _ in 0..request_count {
            requests.push(ItemStackRequestEntry::read(&mut stream));
        }

        ItemStackRequest { requests }
    }

    fn debug(&self) {
        println!("Requests: {:?}", self.requests);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
