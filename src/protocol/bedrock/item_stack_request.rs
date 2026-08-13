use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_entry::ItemStackRequestEntry;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ItemStackRequest {
    pub requests: Vec<ItemStackRequestEntry>,
}

impl Packet for ItemStackRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDItemStackRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.requests.len() as u32);
        for request in self.requests.iter_mut() {
            request.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> ItemStackRequest {
        let request_count = stream.get_var_u32() as usize;
        let mut requests = Vec::new();
        for _ in 0..request_count {
            requests.push(ItemStackRequestEntry::read(stream));
        }

        ItemStackRequest { requests }
    }
}
