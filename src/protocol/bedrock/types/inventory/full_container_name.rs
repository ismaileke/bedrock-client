use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct FullContainerName {
    container_id: u8,
    dynamic_id: Option<u32>,
}

impl FullContainerName {
    pub fn new(container_id: u8, dynamic_id: Option<u32>) -> FullContainerName {
        FullContainerName {
            container_id,
            dynamic_id,
        }
    }

    pub fn read(stream: &mut Stream) -> FullContainerName {
        let container_id = stream.get_byte();
        let dynamic_id = PacketSerializer::read_optional(stream, |s| s.get_u32_le());

        FullContainerName {
            container_id,
            dynamic_id,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_byte(self.container_id);
        PacketSerializer::write_optional(stream, &self.dynamic_id, |s, v| s.put_u32_le(*v));
    }
}
