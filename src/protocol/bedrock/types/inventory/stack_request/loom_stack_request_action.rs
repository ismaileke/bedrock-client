use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

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

    pub fn read(stream: &mut Stream) -> LoomStackRequestAction {
        let pattern_id = PacketSerializer::get_string(stream);
        let repetitions = stream.get_byte();

        LoomStackRequestAction {
            pattern_id,
            repetitions,
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.pattern_id.clone());
        stream.put_byte(self.repetitions);
    }
}
