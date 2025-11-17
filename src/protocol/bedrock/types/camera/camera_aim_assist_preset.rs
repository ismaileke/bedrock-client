use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::camera::camera_aim_assist_preset_item_settings::CameraAimAssistPresetItemSettings;

#[derive(Debug)]
pub struct CameraAimAssistPreset {
    pub identifier: String,
    pub exclusion_list: Vec<String>,
    pub liquid_targeting_list: Vec<String>,
    pub item_settings: Vec<CameraAimAssistPresetItemSettings>,
    pub default_item_settings: Option<String>,
    pub default_hand_settings: Option<String>
}

impl CameraAimAssistPreset {
    pub fn new(
        identifier: String,
        exclusion_list: Vec<String>,
        liquid_targeting_list: Vec<String>,
        item_settings: Vec<CameraAimAssistPresetItemSettings>,
        default_item_settings: Option<String>,
        default_hand_settings: Option<String>
    ) -> CameraAimAssistPreset {
        CameraAimAssistPreset{ identifier, exclusion_list, liquid_targeting_list, item_settings, default_item_settings, default_hand_settings }
    }

    pub fn read(stream: &mut Stream) -> CameraAimAssistPreset {
        let identifier = PacketSerializer::get_string(stream);
        let mut exclusion_list = Vec::new();
        let mut liquid_targeting_list = Vec::new();
        let mut item_settings = Vec::new();
        let mut len = stream.get_unsigned_var_int();
        for _i in 0..len {
            exclusion_list.push(PacketSerializer::get_string(stream));
        }
        len = stream.get_unsigned_var_int();
        for _i in 0..len {
            liquid_targeting_list.push(PacketSerializer::get_string(stream));
        }
        len = stream.get_unsigned_var_int();
        for _i in 0..len {
            item_settings.push(CameraAimAssistPresetItemSettings::read(stream));
        }
        let default_item_settings = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));
        let default_hand_settings = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));

        CameraAimAssistPreset{ identifier, exclusion_list, liquid_targeting_list, item_settings, default_item_settings, default_hand_settings }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.identifier.clone());
        stream.put_unsigned_var_int(self.exclusion_list.len() as u32);
        for exclusion in &self.exclusion_list {
            PacketSerializer::put_string(stream, exclusion.clone());
        }
        stream.put_unsigned_var_int(self.liquid_targeting_list.len() as u32);
        for liquid_target in &self.liquid_targeting_list {
            PacketSerializer::put_string(stream, liquid_target.clone());
        }
        stream.put_unsigned_var_int(self.item_settings.len() as u32);
        for item_setting in &self.item_settings {
            item_setting.write(stream);
        }
        PacketSerializer::write_optional(stream, &self.default_item_settings, |s, v| PacketSerializer::put_string(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.default_hand_settings, |s, v| PacketSerializer::put_string(s, v.clone()));
    }
}