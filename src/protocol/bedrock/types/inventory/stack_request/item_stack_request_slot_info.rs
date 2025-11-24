use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::full_container_name::FullContainerName;

#[derive(serde::Serialize, Debug)]
pub struct ItemStackRequestSlotInfo {
    container_name: FullContainerName,
    slot_id: u8,
    stack_id: i32
}

impl ItemStackRequestSlotInfo {
    pub fn new(container_name: FullContainerName, slot_id: u8, stack_id: i32) -> ItemStackRequestSlotInfo {
        ItemStackRequestSlotInfo{ container_name, slot_id, stack_id }
    }

    pub fn read(stream: &mut Stream) -> ItemStackRequestSlotInfo {
        let container_name = FullContainerName::read(stream);
        let slot_id = stream.get_byte();
        let stack_id = PacketSerializer::read_item_stack_net_id_variant(stream);

        ItemStackRequestSlotInfo{ container_name, slot_id, stack_id }
    }

    pub fn write(&self, stream: &mut Stream) {
        self.container_name.write(stream);
        stream.put_byte(self.slot_id);
        PacketSerializer::write_item_stack_net_id_variant(stream, self.stack_id);
    }
}