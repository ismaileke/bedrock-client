use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::inventory::creative_group_entry::CreativeGroupEntry;
use crate::protocol::bedrock::types::inventory::creative_item_entry::CreativeItemEntry;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct CreativeContent {
    pub groups: Vec<CreativeGroupEntry>,
    pub items: Vec<CreativeItemEntry>,
}

impl CreativeContent {
    pub const CATEGORY_CONSTRUCTION: u8 = 1;
    pub const CATEGORY_NATURE: u8 = 2;
    pub const CATEGORY_EQUIPMENT: u8 = 3;
    pub const CATEGORY_ITEMS: u8 = 4;
}

impl Packet for CreativeContent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCreativeContent.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.groups.len() as u32);
        for group in &self.groups {
            group.write(&mut stream);
        }
        stream.put_var_u32(self.items.len() as u32);
        for item in &self.items {
            item.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CreativeContent {
        let groups_count = stream.get_var_u32() as usize;
        let mut groups = Vec::new();
        for _ in 0..groups_count {
            groups.push(CreativeGroupEntry::read(stream));
        }
        let items_count = stream.get_var_u32() as usize;
        let mut items = Vec::new();
        for _ in 0..items_count {
            items.push(CreativeItemEntry::read(stream));
        }

        CreativeContent { groups, items }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
