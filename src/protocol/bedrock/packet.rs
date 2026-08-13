use binary_utils::binary::{Reader, Writer};

pub trait Packet: Send {
    fn id(&self) -> u16;
    fn encode(&mut self, stream: &mut Writer);
    fn decode(stream: &mut Reader) -> Self
    where
        Self: Sized;
}
