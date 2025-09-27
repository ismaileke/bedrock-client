use std::fmt::Debug;
use binary_utils::binary::Stream;

pub trait PlayerBlockAction: Debug {
    fn get_action_type(&self) -> i32;
    fn write(&mut self, stream: &mut Stream);
}