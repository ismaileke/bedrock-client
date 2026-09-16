use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::pack_setting_type::PackSettingType;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct StringArrayPackSetting {
    pub name: String,
    pub value: Vec<String>
}

impl StringArrayPackSetting {
    pub fn id(&self) -> u32 {
        PackSettingType::STRING_ARRAY
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(name: String, value: Vec<String>) -> StringArrayPackSetting {
        StringArrayPackSetting { name, value }
    }

    pub fn read(stream: &mut Reader, name: String) -> StringArrayPackSetting {
        let len = stream.get_var_u32();
        let mut value = Vec::with_capacity(len as usize);
        for _ in 0..len {
            value.push(PacketSerializer::get_string(stream));
        }

        StringArrayPackSetting { name, value }
    }

    pub fn write(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.value.len() as u32);
        for value in &self.value {
            PacketSerializer::put_string(stream, value);
        }
    }
}
