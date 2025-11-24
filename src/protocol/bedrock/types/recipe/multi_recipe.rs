use binary_utils::binary::Stream;
use crate::protocol::bedrock::crafting_data::CraftingData;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct MultiRecipe {
    type_id: i32,
    recipe_id: String,
    recipe_net_id: u32
}

impl MultiRecipe {
    pub const TYPE_REPAIR_ITEM: &'static str = "00000000-0000-0000-0000-000000000001";
    pub const TYPE_MAP_EXTENDING: &'static str = "D392B075-4BA1-40AE-8789-AF868D56F6CE";
    pub const TYPE_MAP_EXTENDING_CARTOGRAPHY: &'static str = "8B36268C-1829-483C-A0F1-993B7156A8F2";
    pub const TYPE_MAP_CLONING: &'static str = "85939755-BA10-4D9D-A4CC-EFB7A8E943C4";
    pub const TYPE_MAP_CLONING_CARTOGRAPHY: &'static str = "442D85ED-8272-4543-A6F1-418F90DED05D";
    pub const TYPE_MAP_UPGRADING: &'static str = "AECD2294-4B94-434B-8667-4499BB2C9327";
    pub const TYPE_MAP_UPGRADING_CARTOGRAPHY: &'static str = "98C84B38-1085-46BD-B1CE-DD38C159E6CC";
    pub const TYPE_BOOK_CLONING: &'static str = "D1CA6B84-338E-4F2F-9C6B-76CC8B4BD98D";
    pub const TYPE_BANNER_DUPLICATE: &'static str = "B5C5D105-75A2-4076-AF2B-923EA2BF4BF0";
    pub const TYPE_BANNER_ADD_PATTERN: &'static str = "D81AAEAF-E172-4440-9225-868DF030D27B";
    pub const TYPE_FIREWORKS: &'static str = "00000000-0000-0000-0000-000000000002";
    pub const TYPE_MAP_LOCKING_CARTOGRAPHY: &'static str = "602234E4-CAC1-4353-8BB7-B1EBFF70024B";
    
    pub fn new(type_id: i32, recipe_id: String, recipe_net_id: u32) -> MultiRecipe {
        MultiRecipe{ type_id, recipe_id, recipe_net_id }
    }

    pub fn get_type_ids() -> Vec<i32> {
        Vec::from([
            CraftingData::ENTRY_MULTI
        ])
    }

    pub fn get_selected_type_id(&self) -> i32 {
        self.type_id
    }

    pub fn read(type_id: i32, stream: &mut Stream) -> MultiRecipe {
        let recipe_id = PacketSerializer::get_uuid(stream);
        let recipe_net_id = PacketSerializer::read_recipe_net_id(stream);

        MultiRecipe{ type_id, recipe_id, recipe_net_id }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_uuid(stream, self.recipe_id.clone());
        PacketSerializer::write_recipe_net_id(stream, self.recipe_net_id);
    }
}
