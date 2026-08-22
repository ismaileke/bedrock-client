use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::recipe::recipe_ingredient::RecipeIngredient;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CraftRecipeAutoStackRequestAction {
    recipe_id: i32,
    repetitions: u8,
    ingredients: Vec<RecipeIngredient>,
}

impl CraftRecipeAutoStackRequestAction {
    pub fn new(recipe_id: i32, repetitions: u8, ingredients: Vec<RecipeIngredient>) -> CraftRecipeAutoStackRequestAction {
        CraftRecipeAutoStackRequestAction { recipe_id, repetitions, ingredients }
    }

    pub fn read(stream: &mut Reader) -> CraftRecipeAutoStackRequestAction {
        let recipe_id = PacketSerializer::read_recipe_net_id(stream);
        let repetitions = stream.get_u8();
        let len = stream.get_var_u32();
        let mut ingredients = Vec::with_capacity(len as usize);
        for _ in 0..len {
            ingredients.push(PacketSerializer::get_recipe_ingredient(stream));
        }

        CraftRecipeAutoStackRequestAction { recipe_id, repetitions, ingredients }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::write_recipe_net_id(stream, self.recipe_id);
        stream.put_u8(self.repetitions);
        stream.put_var_u32(self.ingredients.len() as u32);
        for ingredient in self.ingredients.iter() {
            PacketSerializer::put_recipe_ingredient(stream, ingredient);
        }
    }
}
