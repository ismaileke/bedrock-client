use crate::protocol::bedrock::types::recipe::default_item_descriptor::DefaultItemDescriptor;
use crate::protocol::bedrock::types::recipe::molang_item_descriptor::MolangItemDescriptor;
use crate::protocol::bedrock::types::recipe::tag_item_descriptor::TagItemDescriptor;
use crate::protocol::bedrock::types::recipe::item_descriptor_type::ItemDescriptorType;
use binary_utils::binary::Writer;
use std::fmt::Debug;


#[derive(serde::Serialize, Debug, Clone)]
pub enum ItemDescriptor {
    Default(DefaultItemDescriptor),
    Molang(MolangItemDescriptor),
    Tag(TagItemDescriptor)
}

impl ItemDescriptor {
    pub fn type_id(&self) -> u32 {
        match self {
            ItemDescriptor::Default(_) => ItemDescriptorType::DEFAULT,
            ItemDescriptor::Molang(_) => ItemDescriptorType::MOLANG,
            ItemDescriptor::Tag(_) => ItemDescriptorType::TAG,
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        match self {
            ItemDescriptor::Default(d) => d.write(stream),
            ItemDescriptor::Molang(d) => d.write(stream),
            ItemDescriptor::Tag(d) => d.write(stream),
        }
    }
}
