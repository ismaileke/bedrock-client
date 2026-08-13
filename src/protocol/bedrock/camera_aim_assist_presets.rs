use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::camera::camera_aim_assist_category::CameraAimAssistCategory;
use crate::protocol::bedrock::types::camera::camera_aim_assist_preset::CameraAimAssistPreset;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CameraAimAssistPresets {
    pub categories: Vec<CameraAimAssistCategory>,
    pub presets: Vec<CameraAimAssistPreset>,
    pub operation: u8,
}

impl Packet for CameraAimAssistPresets {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraAimAssistPresets.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.categories.len() as u32);
        for category in &self.categories {
            category.write(stream);
        }
        stream.put_var_u32(self.presets.len() as u32);
        for preset in &self.presets {
            preset.write(stream);
        }
        stream.put_u8(self.operation);
    }

    fn decode(stream: &mut Reader) -> CameraAimAssistPresets {
        let mut categories = Vec::new();
        let mut presets = Vec::new();
        let mut count = stream.get_var_u32();
        for _ in 0..count {
            categories.push(CameraAimAssistCategory::read(stream));
        }
        count = stream.get_var_u32();
        for _ in 0..count {
            presets.push(CameraAimAssistPreset::read(stream));
        }
        let operation = stream.get_u8();

        CameraAimAssistPresets {
            categories,
            presets,
            operation,
        }
    }
}
