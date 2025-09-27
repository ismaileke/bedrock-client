use std::fmt::Debug;
use binary_utils::binary::Stream;

pub trait RecipeWithTypeId: Debug {
    fn get_selected_type_id(&self) -> i32;

    fn write(&mut self, stream: &mut Stream);
}