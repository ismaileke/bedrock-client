use crate::protocol::bedrock::types::entity::attribute_modifier::AttributeModifier;

#[derive(Debug)]
pub struct Attribute {
    pub id: String,
    pub min: f32,
    pub max: f32,
    pub current: f32,
    pub default: f32,
    pub modifiers: Vec<AttributeModifier>
}
