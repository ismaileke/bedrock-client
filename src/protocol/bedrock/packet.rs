use std::any::Any;
use binary_utils::binary::Stream;

pub trait Packet: Any {
    fn id(&self) -> u16;
    fn encode(&mut self) -> Vec<u8>;
    fn decode(stream: &mut Stream) -> Self
    where
        Self: Sized;
    fn debug(&self);
    fn as_any(&self) -> &dyn Any;
}