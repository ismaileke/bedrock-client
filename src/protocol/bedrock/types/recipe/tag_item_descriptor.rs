use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::recipe::item_descriptor::ItemDescriptor;
use crate::protocol::bedrock::types::recipe::item_descriptor_type::ItemDescriptorType;

#[derive(Debug)]
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
}

impl ItemDescriptor for TagItemDescriptor {
    fn get_type_id(&self) -> u8 {
        ItemDescriptorType::TAG
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.tag.clone());
    }
}


