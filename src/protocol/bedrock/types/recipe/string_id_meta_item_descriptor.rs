use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct StringIdMetaItemDescriptor {
    id: String,
    meta: u16,
}

impl StringIdMetaItemDescriptor {
    pub fn new(id: String, meta: u16) -> StringIdMetaItemDescriptor {
        StringIdMetaItemDescriptor { id, meta }
    }

    pub fn read(stream: &mut Stream) -> StringIdMetaItemDescriptor {
        let id = PacketSerializer::get_string(stream);
        let meta = stream.get_u16_le();

        StringIdMetaItemDescriptor { id, meta }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.id.clone());
        stream.put_u16_le(self.meta);
    }
}
