use std::fmt::Debug;
use binary_utils::binary::Stream;

pub trait PackSetting: Debug {
    fn id(&self) -> u32;
    fn name(&self) -> &str;
    fn write(&mut self, stream: &mut Stream);
}
