use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::pack_setting::PackSetting;
use crate::protocol::bedrock::types::pack_setting_type::PackSettingType;

#[derive(Debug)]
pub struct BoolPackSetting {
    pub name: String,
    pub value: bool
}

impl BoolPackSetting {
    pub fn new(name: String, value: bool) -> BoolPackSetting {
        BoolPackSetting{ name, value }
    }

    pub fn read(stream: &mut Stream, name: String) -> BoolPackSetting {
        BoolPackSetting{ name, value: stream.get_bool() }
    }
}

impl PackSetting for BoolPackSetting {
    fn id(&self) -> u32 {
        PackSettingType::BOOL
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put_bool(self.value);
    }
}


