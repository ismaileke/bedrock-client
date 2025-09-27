use std::any::Any;

pub trait Packet: Any {
    fn id(&self) -> u16;
    fn encode(&mut self) -> Vec<u8>;
    fn decode(bytes: Vec<u8>) -> Self
    where
        Self: Sized;
    fn debug(&self);
    fn as_any(&self) -> &dyn Any;
}