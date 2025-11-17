use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;

#[derive(Debug)]
pub struct NetworkInventoryAction {
    source_type: u32,
    window_id: i32,
    source_flags: u32,
    inventory_slot: u32,
    old_item: ItemStackWrapper,
    new_item: ItemStackWrapper
}

impl NetworkInventoryAction {
    pub const SOURCE_CONTAINER: u32 = 0;

    pub const SOURCE_WORLD: u32 = 2; //drop/pickup item entity
    pub const SOURCE_CREATIVE: u32 = 3;
    pub const SOURCE_TODO: u32 = 99999;

    pub const WINDOW_ID_INVENTORY: i32 = 0;
    pub const WINDOW_ID_OFF_HAND: i32 = 119;
    pub const WINDOW_ID_ARMOUR: i32 = 120;
    pub const WINDOW_ID_UI: i32 = 124;

    pub fn new(source_type: u32, window_id: i32, source_flags: u32, inventory_slot: u32, old_item: ItemStackWrapper, new_item: ItemStackWrapper) -> NetworkInventoryAction {
        NetworkInventoryAction{ source_type, window_id, source_flags, inventory_slot, old_item, new_item }
    }

    pub fn read(stream: &mut Stream) -> NetworkInventoryAction {
        let source_type = stream.get_var_u32();

        let mut window_id = 0;
        let mut source_flags = 0;
        match source_type {
            Self::SOURCE_CONTAINER => {
                window_id = stream.get_var_i32();
            },
            Self::SOURCE_WORLD => {
                source_flags = stream.get_var_u32();
            },
            Self::SOURCE_CREATIVE => {},
            Self::SOURCE_TODO => {
                window_id = stream.get_var_i32();
            },
            _ => {
                panic!("Unknown inventory action source type: {}", source_type);
            }
        }

        let inventory_slot = stream.get_var_u32();
        let old_item = PacketSerializer::get_item_stack_wrapper(stream);
        let new_item = PacketSerializer::get_item_stack_wrapper(stream);

        NetworkInventoryAction{ source_type, window_id, source_flags, inventory_slot, old_item, new_item }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.source_type);

        match self.source_type {
            Self::SOURCE_CONTAINER => {
                stream.put_var_i32(self.window_id);
            },
            Self::SOURCE_WORLD => {
                stream.put_var_u32(self.source_flags);
            },
            Self::SOURCE_CREATIVE => {},
            Self::SOURCE_TODO => {
                stream.put_var_i32(self.window_id);
            },
            _ => {
                panic!("Unknown source type: {}", self.source_type);
            }
        }

        stream.put_var_u32(self.inventory_slot);
        PacketSerializer::put_item_stack_wrapper(stream, self.old_item.clone());
        PacketSerializer::put_item_stack_wrapper(stream, self.new_item.clone());
    }
}