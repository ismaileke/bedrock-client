use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::attribute_layer_settings_weight::AttributeLayerSettingsWeight;
use crate::protocol::bedrock::types::attribute_layer_settings_weight_float::AttributeLayerSettingsWeightFloat;
use crate::protocol::bedrock::types::attribute_layer_settings_weight_string::AttributeLayerSettingsWeightString;

#[derive(serde::Serialize, Debug)]
pub struct AttributeLayerSettings {
    pub priority: i32,
    pub weight: AttributeLayerSettingsWeight,
    pub enabled: bool,
    pub transitions_paused: bool
}

impl AttributeLayerSettings {
    pub fn new(priority: i32, weight: AttributeLayerSettingsWeight, enabled: bool, transitions_paused: bool) -> AttributeLayerSettings {
        AttributeLayerSettings { priority, weight, enabled, transitions_paused }
    }

    pub fn read(stream: &mut Stream) -> AttributeLayerSettings {
        let priority = stream.get_i32_le();
        let attribute_layer_settings_weight_type = stream.get_var_u32();
        let weight = match attribute_layer_settings_weight_type {
            AttributeLayerSettingsWeight::STRING => AttributeLayerSettingsWeight::String(AttributeLayerSettingsWeightString::read(stream)),
            AttributeLayerSettingsWeight::FLOAT => AttributeLayerSettingsWeight::Float(AttributeLayerSettingsWeightFloat::read(stream)),
            _ => panic!("Unknown attribute layer settings weight type: {}", attribute_layer_settings_weight_type)
        };
        let enabled = stream.get_bool();
        let transitions_paused = stream.get_bool();

        AttributeLayerSettings { priority, weight, enabled, transitions_paused }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_i32_le(self.priority);
        stream.put_var_u32(self.weight.id());
        self.weight.write(stream);
        stream.put_bool(self.enabled);
        stream.put_bool(self.transitions_paused);
    }
}
