use crate::protocol::bedrock::types::recipe::recipe_ingredient::RecipeIngredient;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct RecipeUnlockingRequirement {
    unlocking_context: i32,
    unlocking_ingredients: Option<Vec<RecipeIngredient>>
}

impl RecipeUnlockingRequirement {
    pub const CONTEXT_NONE: i32 = 0;
    pub const CONTEXT_ALWAYS_UNLOCKED : i32= 1;
    pub const CONTEXT_PLAYER_IN_WATER: i32 = 2;
    pub const CONTEXT_PLAYER_HAS_MANY_ITEMS: i32 = 3;

    pub fn new(unlocking_context: i32, unlocking_ingredients: Option<Vec<RecipeIngredient>>) -> RecipeUnlockingRequirement {
        RecipeUnlockingRequirement { unlocking_context, unlocking_ingredients }
    }

    pub fn read(stream: &mut Reader) -> RecipeUnlockingRequirement {
        let unlocking_context = stream.get_var_i32();
        let mut unlocking_ingredients = None;
        if stream.get_bool() {
            let mut unlocking_ingredients2 = Vec::new();
            let count = stream.get_var_u32();
            for _ in 0..count {
                unlocking_ingredients2.push(RecipeIngredient::read(stream));
            }
            unlocking_ingredients = Some(unlocking_ingredients2);
        }

        RecipeUnlockingRequirement { unlocking_context, unlocking_ingredients }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_i32(self.unlocking_context);
        stream.put_bool(self.unlocking_ingredients.is_some());
        if let Some(unlocking_ingredients) = &self.unlocking_ingredients {
            stream.put_var_u32(unlocking_ingredients.len() as u32);
            for ingredient in unlocking_ingredients {
                ingredient.write(stream);
            }
        }
    }
}
