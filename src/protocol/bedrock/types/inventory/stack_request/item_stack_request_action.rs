use std::fmt::Debug;
use binary_utils::binary::Stream;

pub trait ItemStackRequestAction: Debug {
    fn get_type_id(&self) -> u8;

    fn write(&mut self, stream: &mut Stream);
}