use std::fmt::Debug;
use binary_utils::binary::Stream;

pub trait MetadataProperty: Debug {
    fn id(&self) -> u32;
    fn write(&mut self, stream: &mut Stream);
}