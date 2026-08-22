use crate::protocol::bedrock::crafting_data::CraftingData;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;
use crate::protocol::bedrock::types::recipe::recipe_ingredient::RecipeIngredient;
use crate::protocol::bedrock::types::recipe::recipe_unlocking_requirement::RecipeUnlockingRequirement;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ShapelessRecipe {
    type_id: i32,
    recipe_id: String,
    inputs: Vec<RecipeIngredient>,
    outputs: Vec<ItemStack>,
    uuid: String,
    block_name: String,
    priority: i32,
    unlocking_requirement: Option<RecipeUnlockingRequirement>,
    recipe_net_id: i32,
}

impl ShapelessRecipe {
    pub fn new(
        type_id: i32,
        recipe_id: String,
        inputs: Vec<RecipeIngredient>,
        outputs: Vec<ItemStack>,
        uuid: String,
        block_name: String,
        priority: i32,
        unlocking_requirement: Option<RecipeUnlockingRequirement>,
        recipe_net_id: i32,
    ) -> ShapelessRecipe {
        ShapelessRecipe {
            type_id,
            recipe_id,
            inputs,
            outputs,
            uuid,
            block_name,
            priority,
            unlocking_requirement,
            recipe_net_id,
        }
    }

    pub fn get_type_ids() -> Vec<i32> {
        Vec::from([
            CraftingData::ENTRY_SHAPELESS,
            CraftingData::ENTRY_USER_DATA_SHAPELESS,
            CraftingData::ENTRY_SHAPELESS_CHEMISTRY,
        ])
    }

    pub fn get_selected_type_id(&self) -> i32 {
        self.type_id
    }

    pub fn read(type_id: i32, stream: &mut Reader) -> ShapelessRecipe {
        let recipe_id = PacketSerializer::get_string(stream);
        let mut inputs = Vec::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            inputs.push(RecipeIngredient::read(stream));
        }
        let mut outputs = Vec::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            outputs.push(PacketSerializer::get_item_stack_without_stack_id(stream));
        }
        let uuid = PacketSerializer::get_uuid(stream);
        let block_name = PacketSerializer::get_string(stream);
        let priority = stream.get_var_i32();
        let unlocking_requirement = PacketSerializer::read_optional(stream, |s| RecipeUnlockingRequirement::read(s));
        let recipe_net_id = PacketSerializer::read_recipe_net_id(stream);

        ShapelessRecipe {
            type_id,
            recipe_id,
            inputs,
            outputs,
            uuid,
            block_name,
            priority,
            unlocking_requirement,
            recipe_net_id,
        }
    }

    pub fn write(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.recipe_id);
        stream.put_var_u32(self.inputs.len() as u32);
        for input in self.inputs.iter() {
            input.write(stream);
        }
        stream.put_var_u32(self.outputs.len() as u32);
        for output in &self.outputs {
            PacketSerializer::put_item_stack_without_stack_id(stream, output);
        }
        PacketSerializer::put_uuid(stream, &self.uuid);
        PacketSerializer::put_string(stream, &self.block_name);
        stream.put_var_i32(self.priority);
        PacketSerializer::write_optional(stream, &self.unlocking_requirement, |s, v| v.write(s));
        PacketSerializer::write_recipe_net_id(stream, self.recipe_net_id);
    }
}
