use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;

#[derive(Debug)]
pub struct CreativeItemEntry {
    entry_id: u32,
    item: ItemStack,
    group_id: u32
}

impl CreativeItemEntry {
    pub fn new(entry_id: u32, item: ItemStack, group_id: u32) -> CreativeItemEntry {
        CreativeItemEntry{ entry_id, item, group_id }
    }

    pub fn read(stream: &mut Stream) -> CreativeItemEntry {
        let entry_id = PacketSerializer::read_creative_item_net_id(stream);
        let item = PacketSerializer::get_item_stack_without_stack_id(stream);
        let group_id = stream.get_var_u32();

        CreativeItemEntry{ entry_id, item, group_id }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::write_creative_item_net_id(stream, self.entry_id);
        PacketSerializer::put_item_stack_without_stack_id(stream, &self.item);
        stream.put_var_u32(self.group_id);
    }
}