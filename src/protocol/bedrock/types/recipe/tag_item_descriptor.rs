use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct TagItemDescriptor {
    tag: String
}

impl TagItemDescriptor {
    pub fn new(tag: String) -> TagItemDescriptor {
        TagItemDescriptor{ tag }
    }

    pub fn read(stream: &mut Stream) -> TagItemDescriptor {
        let tag = PacketSerializer::get_string(stream);

        TagItemDescriptor{ tag }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.tag.clone());
    }
}
