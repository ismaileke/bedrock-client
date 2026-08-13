use crate::protocol::bedrock::types::attribute_layer_settings_weight_float::AttributeLayerSettingsWeightFloat;
use crate::protocol::bedrock::types::attribute_layer_settings_weight_string::AttributeLayerSettingsWeightString;
use binary_utils::binary::Writer;
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum AttributeLayerSettingsWeight {
    String(AttributeLayerSettingsWeightString),
    Float(AttributeLayerSettingsWeightFloat),
}

impl AttributeLayerSettingsWeight {
    pub const FLOAT: u32 = 0;
    pub const STRING: u32 = 1;

    pub fn id(&self) -> u32 {
        match self {
            AttributeLayerSettingsWeight::String(_) => Self::STRING,
            AttributeLayerSettingsWeight::Float(_) => Self::FLOAT,
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        match self {
            AttributeLayerSettingsWeight::String(r) => r.write(stream),
            AttributeLayerSettingsWeight::Float(r) => r.write(stream),
        }
    }
}
