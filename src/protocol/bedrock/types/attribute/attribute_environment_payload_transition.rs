use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::types::attribute::attribute_transition_settings::AttributeTransitionSettings;
use crate::protocol::bedrock::types::attribute::attribute_value::AttributeValue;

#[derive(serde::Serialize, Debug)]
pub struct AttributeEnvironmentPayloadTransition {
    pub from_attribute: AttributeValue,
    pub to_attribute: AttributeValue,
    pub settings: AttributeTransitionSettings
}

impl AttributeEnvironmentPayloadTransition {
    pub fn read(stream: &mut Reader) -> AttributeEnvironmentPayloadTransition {
        let from_attribute = AttributeValue::read(stream);
        let to_attribute = AttributeValue::read(stream);
        let settings = AttributeTransitionSettings::read(stream);
        AttributeEnvironmentPayloadTransition { from_attribute, to_attribute, settings }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u32(self.from_attribute.id());
        self.from_attribute.write(stream);
        stream.put_var_u32(self.to_attribute.id());
        self.to_attribute.write(stream);
        self.settings.write(stream);
    }
}
