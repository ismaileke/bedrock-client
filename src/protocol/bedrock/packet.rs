use binary_utils::binary::Stream;

pub trait Packet: Send {
    fn id(&self) -> u16;
    fn encode(&mut self) -> Vec<u8>;
    fn decode(stream: &mut Stream) -> Self
    where
        Self: Sized;
}
