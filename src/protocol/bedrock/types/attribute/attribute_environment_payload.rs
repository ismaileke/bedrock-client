
use binary_utils::binary::{Reader, Writer};
use std::fmt::Debug;
use crate::protocol::bedrock::types::attribute::attribute_environment_payload_constant::AttributeEnvironmentPayloadConstant;
use crate::protocol::bedrock::types::attribute::attribute_environment_payload_noise_transition::AttributeEnvironmentPayloadNoiseTransition;
use crate::protocol::bedrock::types::attribute::attribute_environment_payload_transition::AttributeEnvironmentPayloadTransition;

#[derive(serde::Serialize, Debug)]
pub enum AttributeEnvironmentPayload {
    Constant(AttributeEnvironmentPayloadConstant),
    Transition(AttributeEnvironmentPayloadTransition),
    NoiseTransition(AttributeEnvironmentPayloadNoiseTransition),
}

impl AttributeEnvironmentPayload {
    pub const CONSTANT: u32 = 0;
    pub const TRANSITION: u32 = 1;
    pub const NOISE_TRANSITION: u32 = 2;

    pub fn id(&self) -> u32 {
        match self {
            AttributeEnvironmentPayload::Constant(_) => Self::CONSTANT,
            AttributeEnvironmentPayload::Transition(_) => Self::TRANSITION,
            AttributeEnvironmentPayload::NoiseTransition(_) => Self::NOISE_TRANSITION,
        }
    }

    pub fn read(stream: &mut Reader) -> AttributeEnvironmentPayload {
        let attribute_env_payload_value_type = stream.get_var_u32();
        match attribute_env_payload_value_type {
            AttributeEnvironmentPayload::CONSTANT => AttributeEnvironmentPayload::Constant(AttributeEnvironmentPayloadConstant::read(stream)),
            AttributeEnvironmentPayload::TRANSITION => AttributeEnvironmentPayload::Transition(AttributeEnvironmentPayloadTransition::read(stream)),
            AttributeEnvironmentPayload::NOISE_TRANSITION => AttributeEnvironmentPayload::NoiseTransition(AttributeEnvironmentPayloadNoiseTransition::read(stream)),
            _ => panic!("Attribute Environment Payload value type not handled: {}", attribute_env_payload_value_type),
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        match self {
            AttributeEnvironmentPayload::Constant(r) => r.write(stream),
            AttributeEnvironmentPayload::Transition(r) => r.write(stream),
            AttributeEnvironmentPayload::NoiseTransition(r) => r.write(stream),
        }
    }
}
