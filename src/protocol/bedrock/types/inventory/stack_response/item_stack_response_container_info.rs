use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::full_container_name::FullContainerName;
use crate::protocol::bedrock::types::inventory::stack_response::item_stack_response_slot_info::ItemStackResponseSlotInfo;

#[derive(Debug)]
pub struct ItemStackResponseContainerInfo {
    container_name: FullContainerName,
    slots: Vec<ItemStackResponseSlotInfo>
}

impl ItemStackResponseContainerInfo {
    pub fn new(container_name: FullContainerName, slots: Vec<ItemStackResponseSlotInfo>) -> ItemStackResponseContainerInfo {
        ItemStackResponseContainerInfo { container_name, slots }
    }

    pub fn read(stream: &mut Stream) -> ItemStackResponseContainerInfo {
        let container_name = FullContainerName::read(stream);
        let slots_count = stream.get_unsigned_var_int();
        let mut slots = Vec::new();
        for _ in 0..slots_count {
            slots.push(ItemStackResponseSlotInfo::read(stream));
        }

        ItemStackResponseContainerInfo{ container_name, slots }
    }

    pub fn write(&self, stream: &mut Stream) {
        self.container_name.write(stream);
        stream.put_unsigned_var_int(self.slots.len() as u32);
        for slot in &self.slots {
            slot.write(stream);
        }
    }
}