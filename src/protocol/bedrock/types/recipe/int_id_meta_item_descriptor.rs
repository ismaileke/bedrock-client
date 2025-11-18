use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::recipe::item_descriptor::ItemDescriptor;
use crate::protocol::bedrock::types::recipe::item_descriptor_type::ItemDescriptorType;

#[derive(Debug)]
pub struct IntIdMetaItemDescriptor {
    id: i16,
    meta: i16
}

impl IntIdMetaItemDescriptor {
    pub fn new(id: i16, meta: i16) -> IntIdMetaItemDescriptor {
        IntIdMetaItemDescriptor{ id, meta }
    }

    pub fn read(stream: &mut Stream) -> IntIdMetaItemDescriptor {
        let id = stream.get_i16_le();
        let mut meta = 0;
        if id != 0 {
            meta = stream.get_i16_le();
        }

        IntIdMetaItemDescriptor{ id, meta }
    }
}

impl ItemDescriptor for IntIdMetaItemDescriptor {
    fn get_type_id(&self) -> u8 {
        ItemDescriptorType::INT_ID_META
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put_i16_le(self.id);
        if self.id != 0 {
            stream.put_i16_le(self.meta);
        }
    }
}


