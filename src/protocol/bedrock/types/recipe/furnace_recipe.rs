use binary_utils::binary::Stream;
use crate::protocol::bedrock::crafting_data::CraftingData;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;
use crate::protocol::bedrock::types::recipe::recipe_with_type_id::RecipeWithTypeId;

#[derive(Debug)]
pub struct FurnaceRecipe {
    type_id: i32,
    input_id: i32,
    input_meta: Option<i32>,
    result: ItemStack,
    block_name: String
}

impl FurnaceRecipe {
    pub fn new(type_id: i32, input_id: i32, input_meta: Option<i32>, result: ItemStack, block_name: String) -> FurnaceRecipe {
        FurnaceRecipe{ type_id, input_id, input_meta, block_name, result }
    }

    pub fn get_type_ids() -> Vec<i32> {
        Vec::from([
            CraftingData::ENTRY_FURNACE,
            CraftingData::ENTRY_FURNACE_DATA
        ])
    }

    pub fn read(type_id: i32, stream: &mut Stream) -> FurnaceRecipe {
        let input_id = stream.get_var_int();
        let mut input_meta = None;
        if type_id == CraftingData::ENTRY_FURNACE_DATA {
            input_meta = Some(stream.get_var_int());
        }
        let result = PacketSerializer::get_item_stack_without_stack_id(stream);
        let block_name = PacketSerializer::get_string(stream);

        FurnaceRecipe{ type_id, input_id, input_meta, block_name, result }
    }
}

impl RecipeWithTypeId for FurnaceRecipe {
    fn get_selected_type_id(&self) -> i32 {
        self.type_id
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put_var_int(self.input_id);
        if self.get_selected_type_id() == CraftingData::ENTRY_FURNACE_DATA {
            stream.put_var_int(self.input_meta.unwrap());
        }
        PacketSerializer::put_item_stack_without_stack_id(stream, &self.result);
        PacketSerializer::put_string(stream, self.block_name.clone());
    }
}