use binary_utils::binary::Stream;
use std::any::Any;

pub trait Packet: Any + Send {
    fn id(&self) -> u16;
    fn encode(&mut self) -> Vec<u8>;
    fn decode(stream: &mut Stream) -> Self
    where
        Self: Sized;
    fn as_any(&self) -> &dyn Any;
    fn as_json(&self) -> String;
}
