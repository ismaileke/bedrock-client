use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::recipe::default_item_descriptor::DefaultItemDescriptor;
use crate::protocol::bedrock::types::recipe::item_descriptor::ItemDescriptor;
use crate::protocol::bedrock::types::recipe::item_descriptor_type::ItemDescriptorType;
use crate::protocol::bedrock::types::recipe::molang_item_descriptor::MolangItemDescriptor;
use crate::protocol::bedrock::types::recipe::tag_item_descriptor::TagItemDescriptor;

#[derive(serde::Serialize, Debug, Clone)]
pub struct RecipeIngredient {
    pub descriptor: Option<ItemDescriptor>,
    pub count: i32
}

impl RecipeIngredient {
    const TYPE_NAME: &'static str = "name";
    const TYPE_MOLANG: &'static str = "molang";
    const TYPE_ITEM_TAG: &'static str = "item_tag";

    const EMPTY_AUX_VALUE: i32 = 0x7fff;

    pub fn new(descriptor: Option<ItemDescriptor>, count: i32) -> RecipeIngredient {
        RecipeIngredient { descriptor, count }
    }

    pub fn read(stream: &mut Reader) -> RecipeIngredient {
        let valid = stream.get_var_i32();
        if valid == 0 {
            let _ = stream.get_var_i32(); //aux value, always 0x7fff
            let count = stream.get_var_i32();
            return RecipeIngredient::new(None, count);
        }
        let descriptor_type = PacketSerializer::get_string(stream);
        let descriptor = match descriptor_type.as_str() {
            Self::TYPE_NAME => ItemDescriptor::Default(DefaultItemDescriptor::read(stream)),
            Self::TYPE_MOLANG => ItemDescriptor::Molang(MolangItemDescriptor::read(stream)),
            Self::TYPE_ITEM_TAG => ItemDescriptor::Tag(TagItemDescriptor::read(stream)),
            _ => panic!("Unknown item descriptor type {}", descriptor_type),
        };
        if descriptor.type_id() == ItemDescriptorType::TAG {
            let _ = stream.get_var_i32(); //aux value, always 0x7fff
        }
        let count = stream.get_var_i32();

        RecipeIngredient::new(Some(descriptor), count)
    }

    pub fn write(&self, stream: &mut Writer) {
        if let Some(descriptor) = &self.descriptor {
            stream.put_var_u32(1);
            PacketSerializer::put_string(stream, match descriptor{
                ItemDescriptor::Default(_) => Self::TYPE_NAME,
                ItemDescriptor::Molang(_) => Self::TYPE_MOLANG,
                ItemDescriptor::Tag(_) => Self::TYPE_ITEM_TAG
            });
            descriptor.write(stream);
            if descriptor.type_id() == ItemDescriptorType::TAG {
                stream.put_var_i32(Self::EMPTY_AUX_VALUE);
            }
            stream.put_var_i32(self.count);
        } else {
            stream.put_var_u32(0);
            stream.put_var_i32(Self::EMPTY_AUX_VALUE);
            stream.put_var_i32(self.count);
        }
    }
}