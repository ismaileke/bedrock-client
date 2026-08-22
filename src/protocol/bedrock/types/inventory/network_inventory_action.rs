use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug, Clone)]
pub struct NetworkInventoryAction {
    source_type: u32,
    window_id: Option<i8>,
    source_flags: Option<u32>,
    inventory_slot: u32,
    old_item: ItemStackWrapper,
    new_item: ItemStackWrapper,
}

impl NetworkInventoryAction {
    pub const SOURCE_CONTAINER: u32 = 0;
    pub const SOURCE_GLOBAL: u32 = 1;
    pub const SOURCE_WORLD: u32 = 2; //drop/pickup item entity
    pub const SOURCE_CREATIVE: u32 = 3;
    pub const SOURCE_TODO: u32 = 99999;

    pub const WINDOW_ID_INVENTORY: i32 = 0;
    pub const WINDOW_ID_OFF_HAND: i32 = 119;
    pub const WINDOW_ID_ARMOUR: i32 = 120;
    pub const WINDOW_ID_UI: i32 = 124;

    pub fn new(
        source_type: u32,
        window_id: Option<i8>,
        source_flags: Option<u32>,
        inventory_slot: u32,
        old_item: ItemStackWrapper,
        new_item: ItemStackWrapper,
    ) -> NetworkInventoryAction {
        NetworkInventoryAction {
            source_type,
            window_id,
            source_flags,
            inventory_slot,
            old_item,
            new_item,
        }
    }

    pub fn read(stream: &mut Reader) -> NetworkInventoryAction {
        let source_type = stream.get_var_u32();
        let window_id = PacketSerializer::read_double_optional(stream, |s| s.get_i8());
        let source_flags = PacketSerializer::read_double_optional(stream, |s| s.get_var_u32());
        let inventory_slot = stream.get_var_u32();
        let old_item = PacketSerializer::get_network_item_stack_descriptor(stream);
        let new_item = PacketSerializer::get_network_item_stack_descriptor(stream);

        NetworkInventoryAction {
            source_type,
            window_id,
            source_flags,
            inventory_slot,
            old_item,
            new_item,
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u32(self.source_type);
        PacketSerializer::write_double_optional(stream, &self.window_id, |s, v| s.put_i8(*v));
        PacketSerializer::write_double_optional(stream, &self.source_flags, |s, v| s.put_var_u32(*v));
        stream.put_var_u32(self.inventory_slot);
        PacketSerializer::put_network_item_stack_descriptor(stream, &self.old_item);
        PacketSerializer::put_network_item_stack_descriptor(stream, &self.new_item);
    }
}
