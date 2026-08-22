use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug, Clone)]
pub struct TagItemDescriptor {
    tag: String,
}

impl TagItemDescriptor {
    pub fn new(tag: String) -> TagItemDescriptor {
        TagItemDescriptor { tag }
    }

    pub fn read(stream: &mut Reader) -> TagItemDescriptor {
        let tag = PacketSerializer::get_string(stream);

        TagItemDescriptor { tag }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.tag);
    }
}
