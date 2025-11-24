use std::fmt::Debug;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::bool_pack_setting::BoolPackSetting;
use crate::protocol::bedrock::types::float_pack_setting::FloatPackSetting;
use crate::protocol::bedrock::types::string_pack_setting::StringPackSetting;

#[derive(serde::Serialize, Debug)]
pub enum PackSetting {
    Float(FloatPackSetting),
    Bool(BoolPackSetting),
    String(StringPackSetting)
}

impl PackSetting {
    pub fn id(&self) -> u32 {
        match self {
            PackSetting::Float(r) => r.id(),
            PackSetting::Bool(r) => r.id(),
            PackSetting::String(r) => r.id()
        }
    }

    pub fn name(&self) -> &str {
        match self {
            PackSetting::Float(r) => r.name(),
            PackSetting::Bool(r) => r.name(),
            PackSetting::String(r) => r.name()
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        match self {
            PackSetting::Float(r) => r.write(stream),
            PackSetting::Bool(r) => r.write(stream),
            PackSetting::String(r) => r.write(stream)
        }
    }
}
