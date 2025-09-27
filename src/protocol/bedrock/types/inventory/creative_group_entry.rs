use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;

#[derive(Debug)]
pub struct CreativeGroupEntry {
    category_id: u32,
    category_name: String,
    icon: ItemStack
}

impl CreativeGroupEntry {
    pub fn new(category_id: u32, category_name: String, icon: ItemStack) -> CreativeGroupEntry {
        CreativeGroupEntry{ category_id, category_name, icon }
    }

    pub fn read(stream: &mut Stream) -> CreativeGroupEntry {
        let category_id = stream.get_l_int();
        let category_name = PacketSerializer::get_string(stream);
        let icon = PacketSerializer::get_item_stack_without_stack_id(stream);

        CreativeGroupEntry{ category_id, category_name, icon }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_int(self.category_id);
        PacketSerializer::put_string(stream, self.category_name.clone());
        PacketSerializer::put_item_stack_without_stack_id(stream, &self.icon);
    }
}