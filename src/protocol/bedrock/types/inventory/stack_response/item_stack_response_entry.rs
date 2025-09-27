use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::stack_response::item_stack_response_container_info::ItemStackResponseContainerInfo;

#[derive(Debug)]
pub struct ItemStackResponseEntry {
    result: u8,
    request_id: i32,
    container_infos: Vec<ItemStackResponseContainerInfo>
}

impl ItemStackResponseEntry {
    pub const RESULT_OK: u8 = 0;
    pub const RESULT_ERROR: u8 = 1;

    pub fn new(result: u8, request_id: i32, container_infos: Vec<ItemStackResponseContainerInfo>) -> ItemStackResponseEntry {
        if result != Self::RESULT_OK && container_infos.len() != 0 {
            panic!("Container infos must be empty if rejecting the request")
        }

        ItemStackResponseEntry { result, request_id, container_infos }
    }

    pub fn read(stream: &mut Stream) -> ItemStackResponseEntry {
        let result = stream.get_byte();
        let request_id = PacketSerializer::read_item_stack_request_id(stream);
        let mut container_infos = Vec::new();
        if result == Self::RESULT_OK {
            let container_infos_count = stream.get_unsigned_var_int();
            for _ in 0..container_infos_count {
                container_infos.push(ItemStackResponseContainerInfo::read(stream));
            }
        }

        ItemStackResponseEntry{ result, request_id, container_infos }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_byte(self.result);
        PacketSerializer::write_item_stack_request_id(stream, self.request_id);
        if self.result == Self::RESULT_OK {
            stream.put_unsigned_var_int(self.container_infos.len() as u32);
            for container_info in &self.container_infos {
                container_info.write(stream);
            }
        }
    }
}