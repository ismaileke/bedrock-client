use crate::protocol::bedrock::types::pack_setting_type::PackSettingType;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BoolPackSetting {
    pub name: String,
    pub value: bool,
}

impl BoolPackSetting {
    pub fn id(&self) -> u32 {
        PackSettingType::BOOL
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(name: String, value: bool) -> BoolPackSetting {
        BoolPackSetting { name, value }
    }

    pub fn read(stream: &mut Stream, name: String) -> BoolPackSetting {
        BoolPackSetting {
            name,
            value: stream.get_bool(),
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_bool(self.value);
    }
}
