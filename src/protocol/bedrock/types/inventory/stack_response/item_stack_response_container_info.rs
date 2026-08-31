use crate::protocol::bedrock::types::inventory::full_container_name::FullContainerName;
use crate::protocol::bedrock::types::inventory::stack_response::item_stack_response_slot_info::ItemStackResponseSlotInfo;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ItemStackResponseContainerInfo {
    pub container_name: FullContainerName,
    pub slots: Vec<ItemStackResponseSlotInfo>,
}

impl ItemStackResponseContainerInfo {
    pub fn new(container_name: FullContainerName, slots: Vec<ItemStackResponseSlotInfo>) -> ItemStackResponseContainerInfo {
        ItemStackResponseContainerInfo { container_name, slots }
    }

    pub fn read(stream: &mut Reader) -> ItemStackResponseContainerInfo {
        let container_name = FullContainerName::read(stream);
        let slots_count = stream.get_var_u32();
        let mut slots = Vec::new();
        for _ in 0..slots_count {
            slots.push(ItemStackResponseSlotInfo::read(stream));
        }

        ItemStackResponseContainerInfo { container_name, slots }
    }

    pub fn write(&self, stream: &mut Writer) {
        self.container_name.write(stream);
        stream.put_var_u32(self.slots.len() as u32);
        for slot in &self.slots {
            slot.write(stream);
        }
    }
}
