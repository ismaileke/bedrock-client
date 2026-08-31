use crate::protocol::bedrock::crafting_data::CraftingData;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;
use crate::protocol::bedrock::types::recipe::recipe_ingredient::RecipeIngredient;
use crate::protocol::bedrock::types::recipe::recipe_unlocking_requirement::RecipeUnlockingRequirement;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ShapedRecipe {
    pub type_id: i32,
    pub recipe_id: String,
    pub inputs: Vec<Vec<RecipeIngredient>>,
    pub outputs: Vec<ItemStack>,
    pub uuid: String,
    pub block_name: String,
    pub priority: i32,
    pub symmetric: bool,
    pub unlocking_requirement: Option<RecipeUnlockingRequirement>,
    pub recipe_net_id: i32,
}

impl ShapedRecipe {
    pub fn new(
        type_id: i32,
        recipe_id: String,
        inputs: Vec<Vec<RecipeIngredient>>,
        outputs: Vec<ItemStack>,
        uuid: String,
        block_name: String,
        priority: i32,
        symmetric: bool,
        unlocking_requirement: Option<RecipeUnlockingRequirement>,
        recipe_net_id: i32,
    ) -> ShapedRecipe {
        let rows = inputs.len();
        if rows < 1 || rows > 3 {
            panic!("Expected 1, 2 or 3 input rows");
        }
        ShapedRecipe {
            type_id,
            recipe_id,
            inputs,
            outputs,
            uuid,
            block_name,
            priority,
            symmetric,
            unlocking_requirement,
            recipe_net_id,
        }
    }

    pub fn get_type_ids() -> Vec<i32> {
        Vec::from([
            CraftingData::ENTRY_SHAPED,
            CraftingData::ENTRY_SHAPED_CHEMISTRY,
        ])
    }

    pub fn get_selected_type_id(&self) -> i32 {
        self.type_id
    }

    pub fn get_width(&self) -> usize {
        self.inputs.get(0).unwrap().len()
    }

    pub fn get_height(&self) -> usize {
        self.inputs.len()
    }

    pub fn read(type_id: i32, stream: &mut Reader) -> ShapedRecipe {
        let recipe_id = PacketSerializer::get_string(stream);
        let width = stream.get_var_i32();
        let _height = stream.get_var_i32();
        let mut ingredients = Vec::new();
        let ingredients_count = stream.get_var_u32();
        for _ in 0..ingredients_count {
            ingredients.push(RecipeIngredient::read(stream));
        }
        let inputs = ingredients
            .chunks(width.max(1) as usize)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<RecipeIngredient>>>();
        let mut outputs = Vec::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            outputs.push(PacketSerializer::get_item_stack_without_stack_id(stream));
        }
        let uuid = PacketSerializer::get_uuid(stream);
        let block_name = PacketSerializer::get_string(stream);
        let priority = stream.get_var_i32();
        let symmetric = stream.get_bool();
        let unlocking_requirement = PacketSerializer::read_optional(stream, |s| RecipeUnlockingRequirement::read(s));
        let recipe_net_id = PacketSerializer::read_recipe_net_id(stream);

        ShapedRecipe {
            type_id,
            recipe_id,
            inputs,
            outputs,
            uuid,
            block_name,
            priority,
            symmetric,
            unlocking_requirement,
            recipe_net_id,
        }
    }

    pub fn write(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.recipe_id);
        stream.put_var_i32(self.get_width() as i32);
        stream.put_var_i32(self.get_height() as i32);
        stream.put_var_u32(self.get_width() as u32 * self.get_height() as u32);
        for row in self.inputs.iter_mut() {
            for ingredient in row {
                ingredient.write(stream);
            }
        }
        stream.put_var_u32(self.outputs.len() as u32);
        for output in &self.outputs {
            PacketSerializer::put_item_stack_without_stack_id(stream, output);
        }
        PacketSerializer::put_uuid(stream, &self.uuid);
        PacketSerializer::put_string(stream, &self.block_name);
        stream.put_var_i32(self.priority);
        stream.put_bool(self.symmetric);
        PacketSerializer::write_optional(stream, &self.unlocking_requirement, |s, v| v.write(s));
        PacketSerializer::write_recipe_net_id(stream, self.recipe_net_id);
    }
}
