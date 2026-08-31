use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::stack_response::item_stack_response_container_info::ItemStackResponseContainerInfo;

#[derive(serde::Serialize, Debug)]
pub struct ItemStackResponseEntry {
    pub status: u8,
    pub request_id: i32,
    pub container_infos: Vec<ItemStackResponseContainerInfo>,
}

impl ItemStackResponseEntry {
    pub const STATUS_OK: u8 = 0;
    pub const STATUS_ERROR: u8 = 1;
    pub const STATUS_INVALID_REQUEST_ACTION_TYPE: u8 = 2;
    pub const STATUS_ACTION_REQUEST_NOT_ALLOWED: u8 = 3;
    pub const STATUS_SCREEN_HANDLER_END_REQUEST_FAILED: u8 = 4;
    pub const STATUS_ITEM_REQUEST_ACTION_HANDLER_COMMIT_FAILED: u8 = 5;
    pub const STATUS_INVALID_REQUEST_CRAFT_ACTION_TYPE: u8 = 6;
    pub const STATUS_INVALID_CRAFT_REQUEST: u8 = 7;
    pub const STATUS_INVALID_CRAFT_REQUEST_SCREEN: u8 = 8;
    pub const STATUS_INVALID_CRAFT_RESULT: u8 = 9;
    pub const STATUS_INVALID_CRAFT_RESULT_INDEX: u8 = 10;
    pub const STATUS_INVALID_CRAFT_RESULT_ITEM: u8 = 11;
    pub const STATUS_INVALID_ITEM_NET_ID: u8 = 12;
    pub const STATUS_MISSING_CREATED_OUTPUT_CONTAINER: u8 = 13;
    pub const STATUS_FAILED_TO_SET_CREATED_ITEM_OUTPUT_SLOT: u8 = 14;
    pub const STATUS_REQUEST_ALREADY_IN_PROGRESS: u8 = 15;
    pub const STATUS_FAILED_TO_INIT_SPARSE_CONTAINER: u8 = 16;
    pub const STATUS_RESULT_TRANSFER_FAILED: u8 = 17;
    pub const STATUS_EXPECTED_ITEM_SLOT_NOT_FULLY_CONSUMED: u8 = 18;
    pub const STATUS_EXPECTED_ANYWHERE_ITEM_NOT_FULLY_CONSUMED: u8 = 19;
    pub const STATUS_ITEM_ALREADY_CONSUMED_FROM_SLOT: u8 = 20;
    pub const STATUS_CONSUMED_TOO_MUCH_FROM_SLOT: u8 = 21;
    pub const STATUS_MISMATCH_SLOT_EXPECTED_CONSUMED_ITEM: u8 = 22;
    pub const STATUS_MISMATCH_SLOT_EXPECTED_CONSUMED_ITEM_NET_ID_VARIANT: u8 = 23;
    pub const STATUS_FAILED_TO_MATCH_EXPECTED_SLOT_CONSUMED_ITEM: u8 = 24;
    pub const STATUS_FAILED_TO_MATCH_EXPECTED_ALLOWED_ANYWHERE_CONSUMED_ITEM: u8 = 25;
    pub const STATUS_CONSUMED_ITEM_OUT_OF_ALLOWED_SLOT_RANGE: u8 = 26;
    pub const STATUS_CONSUMED_ITEM_NOT_ALLOWED: u8 = 27;
    pub const STATUS_PLAYER_NOT_IN_CREATIVE_MODE: u8 = 28;
    pub const STATUS_INVALID_EXPERIMENTAL_RECIPE_REQUEST: u8 = 29;
    pub const STATUS_FAILED_TO_CRAFT_CREATIVE: u8 = 30;
    pub const STATUS_FAILED_TO_GET_LEVEL_RECIPE: u8 = 31;
    pub const STATUS_FAILED_TO_FIND_RECIPE_BY_NET_ID: u8 = 32;
    pub const STATUS_MISMATCHED_CRAFTING_SIZE: u8 = 33;
    pub const STATUS_MISSING_INPUT_SPARSE_CONTAINER: u8 = 34;
    pub const STATUS_MISMATCHED_RECIPE_FOR_INPUT_GRID_ITEMS: u8 = 35;
    pub const STATUS_EMPTY_CRAFT_RESULTS: u8 = 36;
    pub const STATUS_FAILED_TO_ENCHANT: u8 = 37;
    pub const STATUS_MISSING_INPUT_ITEM: u8 = 38;
    pub const STATUS_INSUFFICIENT_PLAYER_LEVEL_TO_ENCHANT: u8 = 39;
    pub const STATUS_MISSING_MATERIAL_ITEM: u8 = 40;
    pub const STATUS_MISSING_ACTOR: u8 = 41;
    pub const STATUS_UNKNOWN_PRIMARY_EFFECT: u8 = 42;
    pub const STATUS_PRIMARY_EFFECT_OUT_OF_RANGE: u8 = 43;
    pub const STATUS_PRIMARY_EFFECT_UNAVAILABLE: u8 = 44;
    pub const STATUS_SECONDARY_EFFECT_OUT_OF_RANGE: u8 = 45;
    pub const STATUS_SECONDARY_EFFECT_UNAVAILABLE: u8 = 46;
    pub const STATUS_DST_CONTAINER_EQUAL_TO_CREATED_OUTPUT_CONTAINER: u8 = 47;
    pub const STATUS_DST_CONTAINER_AND_SLOT_EQUAL_TO_SRC_CONTAINER_AND_SLOT: u8 = 48;
    pub const STATUS_FAILED_TO_VALIDATE_SRC_SLOT: u8 = 49;
    pub const STATUS_FAILED_TO_VALIDATE_DST_SLOT: u8 = 50;
    pub const STATUS_INVALID_ADJUSTED_AMOUNT: u8 = 51;
    pub const STATUS_INVALID_ITEM_SET_TYPE: u8 = 52;
    pub const STATUS_INVALID_TRANSFER_AMOUNT: u8 = 53;
    pub const STATUS_CANNOT_SWAP_ITEM: u8 = 54;
    pub const STATUS_CANNOT_PLACE_ITEM: u8 = 55;
    pub const STATUS_UNHANDLED_ITEM_SET_TYPE: u8 = 56;
    pub const STATUS_INVALID_REMOVED_AMOUNT: u8 = 57;
    pub const STATUS_INVALID_REGION: u8 = 58;
    pub const STATUS_CANNOT_DROP_ITEM: u8 = 59;
    pub const STATUS_CANNOT_DESTROY_ITEM: u8 = 60;
    pub const STATUS_INVALID_SOURCE_CONTAINER: u8 = 61;
    pub const STATUS_ITEM_NOT_CONSUMED: u8 = 62;
    pub const STATUS_INVALID_NUM_CRAFTS: u8 = 63;
    pub const STATUS_INVALID_CRAFT_RESULT_STACK_SIZE: u8 = 64;
    pub const STATUS_CANNOT_REMOVE_ITEM: u8 = 65;
    pub const STATUS_CANNOT_CONSUME_ITEM: u8 = 66;
    pub const STATUS_SCREEN_STACK_ERROR: u8 = 67;

    pub fn new(
        status: u8,
        request_id: i32,
        container_infos: Vec<ItemStackResponseContainerInfo>,
    ) -> ItemStackResponseEntry {
        if status != Self::STATUS_OK && container_infos.len() != 0 {
            panic!("Container infos must be empty if rejecting the request");
        }

        ItemStackResponseEntry { status, request_id, container_infos }
    }

    pub fn read(stream: &mut Reader) -> ItemStackResponseEntry {
        let status = stream.get_u8();
        let request_id = PacketSerializer::read_item_stack_request_id(stream);
        let mut container_infos = Vec::new();
        let has_containers = stream.get_bool();
        if has_containers {
            let containers_present = stream.get_bool();
            if containers_present {
                let len = stream.get_var_u32();
                for _ in 0..len {
                    container_infos.push(ItemStackResponseContainerInfo::read(stream));
                }
            }
        }

        ItemStackResponseEntry { status, request_id, container_infos }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_u8(self.status);
        PacketSerializer::write_item_stack_request_id(stream, self.request_id);
        let has_containers = self.container_infos.len() != 0;
        stream.put_bool(has_containers);
        if has_containers {
            let containers_present = self.container_infos.len() != 0;
            stream.put_bool(containers_present);
            if containers_present {
                stream.put_var_u32(self.container_infos.len() as u32);
                for container_info in self.container_infos.iter() {
                    container_info.write(stream);
                }
            }
        }
    }
}
