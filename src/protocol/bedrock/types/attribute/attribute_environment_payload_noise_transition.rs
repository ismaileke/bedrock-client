use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::types::attribute::attribute_noise_transition_settings::AttributeNoiseTransitionSettings;
use crate::protocol::bedrock::types::attribute::attribute_value::AttributeValue;

#[derive(serde::Serialize, Debug)]
pub struct AttributeEnvironmentPayloadNoiseTransition {
    pub from_attribute: AttributeValue,
    pub to_attribute: AttributeValue,
    pub settings: AttributeNoiseTransitionSettings
}

impl AttributeEnvironmentPayloadNoiseTransition {
    pub fn read(stream: &mut Reader) -> AttributeEnvironmentPayloadNoiseTransition {
        let from_attribute = AttributeValue::read(stream);
        let to_attribute = AttributeValue::read(stream);
        let settings = AttributeNoiseTransitionSettings::read(stream);
        AttributeEnvironmentPayloadNoiseTransition { from_attribute, to_attribute, settings }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u32(self.from_attribute.id());
        self.from_attribute.write(stream);
        stream.put_var_u32(self.to_attribute.id());
        self.to_attribute.write(stream);
        self.settings.write(stream);
    }
}
