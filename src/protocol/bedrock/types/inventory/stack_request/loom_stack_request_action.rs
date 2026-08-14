use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct LoomStackRequestAction {
    pattern_id: String,
    repetitions: u8,
}

impl LoomStackRequestAction {
    pub fn new(pattern_id: String, repetitions: u8) -> LoomStackRequestAction {
        LoomStackRequestAction {
            pattern_id,
            repetitions,
        }
    }

    pub fn read(stream: &mut Reader) -> LoomStackRequestAction {
        let pattern_id = PacketSerializer::get_string(stream);
        let repetitions = stream.get_u8();

        LoomStackRequestAction {
            pattern_id,
            repetitions,
        }
    }

    pub fn write(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.pattern_id);
        stream.put_u8(self.repetitions);
    }
}
