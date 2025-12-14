use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::recipe::recipe_ingredient::RecipeIngredient;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CraftRecipeAutoStackRequestAction {
    recipe_id: u32,
    repetitions: u8,
    repetitions2: u8,
    ingredients: Vec<RecipeIngredient>,
}

impl CraftRecipeAutoStackRequestAction {
    pub fn new(
        recipe_id: u32,
        repetitions: u8,
        repetitions2: u8,
        ingredients: Vec<RecipeIngredient>,
    ) -> CraftRecipeAutoStackRequestAction {
        CraftRecipeAutoStackRequestAction {
            recipe_id,
            repetitions,
            repetitions2,
            ingredients,
        }
    }

    pub fn read(stream: &mut Stream) -> CraftRecipeAutoStackRequestAction {
        let recipe_id = PacketSerializer::read_recipe_net_id(stream);
        let repetitions = stream.get_byte();
        let repetitions2 = stream.get_byte();
        let len = stream.get_byte();
        let mut ingredients = Vec::new();
        for _ in 0..len {
            ingredients.push(PacketSerializer::get_recipe_ingredient(stream));
        }

        CraftRecipeAutoStackRequestAction {
            recipe_id,
            repetitions,
            repetitions2,
            ingredients,
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::write_recipe_net_id(stream, self.recipe_id);
        stream.put_byte(self.repetitions);
        stream.put_byte(self.repetitions2);
        stream.put_byte(self.ingredients.len() as u8);
        for ingredient in self.ingredients.iter_mut() {
            PacketSerializer::put_recipe_ingredient(stream, ingredient);
        }
    }
}
