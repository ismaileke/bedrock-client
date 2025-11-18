use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::pack_setting::PackSetting;
use crate::protocol::bedrock::types::pack_setting_type::PackSettingType;

#[derive(Debug)]
pub struct StringPackSetting {
    pub name: String,
    pub value: String
}

impl StringPackSetting {
    pub fn new(name: String, value: String) -> StringPackSetting {
        StringPackSetting{ name, value }
    }

    pub fn read(stream: &mut Stream, name: String) -> StringPackSetting {
        StringPackSetting{ name, value: PacketSerializer::get_string(stream) }
    }
}

impl PackSetting for StringPackSetting {
    fn id(&self) -> u32 {
        PackSettingType::STRING
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.value.clone());
    }
}


