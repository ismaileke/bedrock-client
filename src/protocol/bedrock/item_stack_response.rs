use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::inventory::stack_response::item_stack_response_entry::ItemStackResponseEntry;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ItemStackResponse {
    pub responses: Vec<ItemStackResponseEntry>,
}

impl Packet for ItemStackResponse {
    fn id(&self) -> u16 {
        BedrockPacketType::IDItemStackResponse.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.responses.len() as u32);
        for response in self.responses.iter() {
            response.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> ItemStackResponse {
        let response_count = stream.get_var_u32() as usize;
        let mut responses = Vec::with_capacity(response_count);
        for _ in 0..response_count {
            responses.push(ItemStackResponseEntry::read(stream));
        }

        ItemStackResponse { responses }
    }
}
