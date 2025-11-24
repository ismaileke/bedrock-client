use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::pack_setting_type::PackSettingType;

#[derive(serde::Serialize, Debug)]
pub struct FloatPackSetting {
    pub name: String,
    pub value: f32
}

impl FloatPackSetting {
    pub fn id(&self) -> u32 {
        PackSettingType::FLOAT
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(name: String, value: f32) -> FloatPackSetting {
        FloatPackSetting{ name, value }
    }

    pub fn read(stream: &mut Stream, name: String) -> FloatPackSetting {
        FloatPackSetting{ name, value: stream.get_f32_le() }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_f32_le(self.value);
    }
}
