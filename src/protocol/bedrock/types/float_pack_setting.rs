use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::pack_setting::PackSetting;
use crate::protocol::bedrock::types::pack_setting_type::PackSettingType;

#[derive(Debug)]
pub struct FloatPackSetting {
    pub name: String,
    pub value: f32
}

impl FloatPackSetting {
    pub fn new(name: String, value: f32) -> FloatPackSetting {
        FloatPackSetting{ name, value }
    }

    pub fn read(stream: &mut Stream, name: String) -> FloatPackSetting {
        FloatPackSetting{ name, value: stream.get_l_float() }
    }
}

impl PackSetting for FloatPackSetting {
    fn id(&self) -> u32 {
        PackSettingType::FLOAT
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put_l_float(self.value);
    }
}


