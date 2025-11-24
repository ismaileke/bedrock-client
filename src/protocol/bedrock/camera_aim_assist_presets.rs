use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::camera::camera_aim_assist_category::CameraAimAssistCategory;
use crate::protocol::bedrock::types::camera::camera_aim_assist_preset::CameraAimAssistPreset;

#[derive(serde::Serialize, Debug)]
pub struct CameraAimAssistPresets {
    pub categories: Vec<CameraAimAssistCategory>,
    pub presets: Vec<CameraAimAssistPreset>,
    pub operation: u8
}

pub fn new(categories: Vec<CameraAimAssistCategory>, presets: Vec<CameraAimAssistPreset>, operation: u8) -> CameraAimAssistPresets {
    CameraAimAssistPresets { categories, presets, operation }
}

impl Packet for CameraAimAssistPresets {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraAimAssistPresets.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.categories.len() as u32);
        for category in &self.categories {
            category.write(&mut stream);
        }
        stream.put_var_u32(self.presets.len() as u32);
        for preset in &self.presets {
            preset.write(&mut stream);
        }
        stream.put_byte(self.operation);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CameraAimAssistPresets {
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
        let operation = stream.get_byte();

        CameraAimAssistPresets { categories, presets, operation }
    }

    fn debug(&self) {
        println!("Categories: {:?}", self.categories);
        println!("Presets: {:?}", self.presets);
        println!("Operation: {}", self.operation);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
