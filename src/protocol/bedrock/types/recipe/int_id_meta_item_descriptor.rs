use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct IntIdMetaItemDescriptor {
    id: i16,
    meta: i16,
}

impl IntIdMetaItemDescriptor {
    pub fn new(id: i16, meta: i16) -> IntIdMetaItemDescriptor {
        IntIdMetaItemDescriptor { id, meta }
    }

    pub fn read(stream: &mut Stream) -> IntIdMetaItemDescriptor {
        let id = stream.get_i16_le();
        let mut meta = 0;
        if id != 0 {
            meta = stream.get_i16_le();
        }

        IntIdMetaItemDescriptor { id, meta }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_i16_le(self.id);
        if self.id != 0 {
            stream.put_i16_le(self.meta);
        }
    }
}
