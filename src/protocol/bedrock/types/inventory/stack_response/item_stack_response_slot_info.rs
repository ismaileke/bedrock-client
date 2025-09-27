use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(Debug)]
pub struct ItemStackResponseSlotInfo {
    slot: u8,
    hotbar_slot: u8,
    count: u8,
    item_stack_id: i32,
    custom_name: String,
    filtered_custom_name: String,
    durability_correction: i32
}

impl ItemStackResponseSlotInfo {
    pub fn new(
        slot: u8,
        hotbar_slot: u8,
        count: u8,
        item_stack_id: i32,
        custom_name: String,
        filtered_custom_name: String,
        durability_correction: i32
    ) -> ItemStackResponseSlotInfo {
        ItemStackResponseSlotInfo { slot, hotbar_slot, count, item_stack_id, custom_name, filtered_custom_name, durability_correction }
    }

    pub fn read(stream: &mut Stream) -> ItemStackResponseSlotInfo {
        let slot = stream.get_byte();
        let hotbar_slot = stream.get_byte();
        let count = stream.get_byte();
        let item_stack_id = PacketSerializer::read_server_item_stack_id(stream);
        let custom_name = PacketSerializer::get_string(stream);
        let filtered_custom_name = PacketSerializer::get_string(stream);
        let durability_correction = stream.get_var_int();

        ItemStackResponseSlotInfo{ slot, hotbar_slot, count, item_stack_id, custom_name, filtered_custom_name, durability_correction }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_byte(self.slot);
        stream.put_byte(self.hotbar_slot);
        stream.put_byte(self.count);
        PacketSerializer::write_server_item_stack_id(stream, self.item_stack_id);
        PacketSerializer::put_string(stream, self.custom_name.clone());
        PacketSerializer::put_string(stream, self.filtered_custom_name.clone());
        stream.put_var_int(self.durability_correction);
    }
}