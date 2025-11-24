use std::fmt::Debug;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::recipe::complex_alias_item_descriptor::ComplexAliasItemDescriptor;
use crate::protocol::bedrock::types::recipe::int_id_meta_item_descriptor::IntIdMetaItemDescriptor;
use crate::protocol::bedrock::types::recipe::item_descriptor_type::ItemDescriptorType;
use crate::protocol::bedrock::types::recipe::molang_item_descriptor::MolangItemDescriptor;
use crate::protocol::bedrock::types::recipe::string_id_meta_item_descriptor::StringIdMetaItemDescriptor;
use crate::protocol::bedrock::types::recipe::tag_item_descriptor::TagItemDescriptor;

#[derive(serde::Serialize, Debug)]
pub enum ItemDescriptor {
    IntIDMeta(IntIdMetaItemDescriptor),
    Molang(MolangItemDescriptor),
    Tag(TagItemDescriptor),
    StringIDMeta(StringIdMetaItemDescriptor),
    ComplexAlias(ComplexAliasItemDescriptor)
}

impl ItemDescriptor {
    pub fn type_id(&self) -> u8 {
        match self {
            ItemDescriptor::IntIDMeta(_) => ItemDescriptorType::INT_ID_META,
            ItemDescriptor::Molang(_) => ItemDescriptorType::MOLANG,
            ItemDescriptor::Tag(_) => ItemDescriptorType::TAG,
            ItemDescriptor::StringIDMeta(_) => ItemDescriptorType::STRING_ID_META,
            ItemDescriptor::ComplexAlias(_) =>  ItemDescriptorType::COMPLEX_ALIAS
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        match self {
            ItemDescriptor::IntIDMeta(d) => d.write(stream),
            ItemDescriptor::Molang(d) => d.write(stream),
            ItemDescriptor::Tag(d) => d.write(stream),
            ItemDescriptor::StringIDMeta(d) => d.write(stream),
            ItemDescriptor::ComplexAlias(d) => d.write(stream),
        }
    }
}