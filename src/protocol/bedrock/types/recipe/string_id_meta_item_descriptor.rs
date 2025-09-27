use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::recipe::item_descriptor::ItemDescriptor;
use crate::protocol::bedrock::types::recipe::item_descriptor_type::ItemDescriptorType;

#[derive(Debug)]
pub struct StringIdMetaItemDescriptor {
    id: String,
    meta: u16
}

impl StringIdMetaItemDescriptor {
    pub fn new(id: String, meta: u16) -> StringIdMetaItemDescriptor {
        StringIdMetaItemDescriptor{ id, meta }
    }

    pub fn read(stream: &mut Stream) -> StringIdMetaItemDescriptor {
        let id = PacketSerializer::get_string(stream);
        let meta = stream.get_l_short();

        StringIdMetaItemDescriptor{ id, meta }
    }
}

impl ItemDescriptor for StringIdMetaItemDescriptor {
    fn get_type_id(&self) -> u8 {
        ItemDescriptorType::STRING_ID_META
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.id.clone());
        stream.put_l_short(self.meta);
    }
}


