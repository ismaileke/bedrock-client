use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::recipe::recipe_ingredient::RecipeIngredient;

#[derive(Debug)]
pub struct RecipeUnlockingRequirement {
    unlocking_ingredients: Option<Vec<RecipeIngredient>>
}

impl RecipeUnlockingRequirement {
    pub fn new(unlocking_ingredients: Option<Vec<RecipeIngredient>>) -> RecipeUnlockingRequirement {
        RecipeUnlockingRequirement{ unlocking_ingredients }
    }

    pub fn read(stream: &mut Stream) -> RecipeUnlockingRequirement {
        //I don't know what the point of this structure is. It could easily have been a list<RecipeIngredient> instead.
        //It's basically just an optional list, which could have been done by an empty list wherever it's not needed.
        let unlocking_context = stream.get_bool();
        let mut unlocking_ingredients = None;
        if !unlocking_context {
            let mut unlocking_ingredients2 = Vec::new();
            let count = stream.get_unsigned_var_int();
            for _ in 0..count {
                unlocking_ingredients2.push(PacketSerializer::get_recipe_ingredient(stream));
            }
            unlocking_ingredients = Some(unlocking_ingredients2);
        }

        RecipeUnlockingRequirement{ unlocking_ingredients }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_bool(self.unlocking_ingredients.is_none());
        if let Some(unlocking_ingredients) = self.unlocking_ingredients.as_mut() {
            stream.put_unsigned_var_int(unlocking_ingredients.len() as u32);
            for ingredient in unlocking_ingredients {
                PacketSerializer::put_recipe_ingredient(stream, ingredient);
            }
        }
    }
}