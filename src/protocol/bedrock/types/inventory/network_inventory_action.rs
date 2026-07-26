use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug, Clone)]
pub struct NetworkInventoryAction {
    source_type: u32,
    window_id: Option<i32>,
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
        window_id: Option<i32>,
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

    pub fn read_auth_input(stream: &mut Stream) -> NetworkInventoryAction {
        let source_type = stream.get_var_u32();

        let mut window_id = None;
        let mut source_flags = None;
        match source_type {
            Self::SOURCE_CONTAINER => {
                window_id = Some(stream.get_var_i32());
            }
            Self::SOURCE_WORLD => {
                source_flags = Some(stream.get_var_u32());
            }
            Self::SOURCE_CREATIVE => {}
            Self::SOURCE_TODO => {
                window_id = Some(stream.get_var_i32());
            }
            _ => {
                panic!("Unknown inventory action source type: {}", source_type);
            }
        }

        let inventory_slot = stream.get_var_u32();
        let old_item = PacketSerializer::get_item_stack_wrapper(stream);
        let new_item = PacketSerializer::get_item_stack_wrapper(stream);

        NetworkInventoryAction {
            source_type,
            window_id,
            source_flags,
            inventory_slot,
            old_item,
            new_item,
        }
    }

    pub fn write_auth_input(&self, stream: &mut Stream) {
        stream.put_var_u32(self.source_type);

        match self.source_type {
            Self::SOURCE_CONTAINER => {
                if let Some(window_id) = self.window_id {
                    stream.put_var_i32(window_id);
                } else {
                    panic!("WindowID must be set for SOURCE_CONTAINER");
                }
            }
            Self::SOURCE_WORLD => {
                if let Some(source_flags) = self.source_flags {
                    stream.put_var_u32(source_flags);
                } else {
                    panic!("SourceFlags must be set for SOURCE_WORLD");
                }
            }
            Self::SOURCE_CREATIVE => {}
            Self::SOURCE_TODO => {
                if let Some(window_id) = self.window_id {
                    stream.put_var_i32(window_id);
                } else {
                    panic!("WindowID must be set for SOURCE_TODO");
                }
            }
            _ => {
                panic!("Unknown source type: {}", self.source_type);
            }
        }

        stream.put_var_u32(self.inventory_slot);
        PacketSerializer::put_item_stack_wrapper(stream, self.old_item.clone());
        PacketSerializer::put_item_stack_wrapper(stream, self.new_item.clone());
    }

    pub fn read_transaction(stream: &mut Stream) -> NetworkInventoryAction {
        let source_type = stream.get_var_u32();

        let mut byte = stream.get_byte();
        if byte != 1 {
            panic!("Inconsistent optional state for windowId");
        }

        let window_id = PacketSerializer::read_optional(stream, |s| (s.get_byte() as i8) as i32);

        byte = stream.get_byte();
        if byte != 1 {
            panic!("Inconsistent optional state for sourceFlags");
        }

        let source_flags = PacketSerializer::read_optional(stream, |s| s.get_var_u32());

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

    pub fn write_transaction(&self, stream: &mut Stream) {
        stream.put_var_u32(self.source_type);

        stream.put_byte(1);
        PacketSerializer::write_optional(stream, &self.window_id, |s, v| s.put_byte(*v as u8)); // check later u8/i8 conversion

        stream.put_byte(1);
        PacketSerializer::write_optional(stream, &self.source_flags, |s, v| s.put_var_u32(*v));

        stream.put_var_u32(self.inventory_slot);
        PacketSerializer::put_network_item_stack_descriptor(stream, self.old_item.clone());
        PacketSerializer::put_network_item_stack_descriptor(stream, self.new_item.clone());
    }
}
