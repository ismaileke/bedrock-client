use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CreativeGroupEntry {
    pub category_id: u8,
    pub category_name: String,
    pub icon: ItemStack,
}

impl CreativeGroupEntry {
    pub fn new(category_id: u8, category_name: String, icon: ItemStack) -> CreativeGroupEntry {
        CreativeGroupEntry { category_id, category_name, icon }
    }

    pub fn read(stream: &mut Reader) -> CreativeGroupEntry {
        let category_id = stream.get_u8();
        let category_name = PacketSerializer::get_string(stream);
        let icon = PacketSerializer::get_item_stack_without_stack_id(stream);

        CreativeGroupEntry { category_id, category_name, icon }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_u8(self.category_id);
        PacketSerializer::put_string(stream, &self.category_name);
        PacketSerializer::put_item_stack_without_stack_id(stream, &self.icon);
    }
}
