use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::camera::camera_aim_assist_category::CameraAimAssistCategory;
use crate::protocol::bedrock::types::camera::camera_aim_assist_preset::CameraAimAssistPreset;

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
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.categories.len() as u32);
        for category in &self.categories {
            category.write(&mut stream);
        }
        stream.put_unsigned_var_int(self.presets.len() as u32);
        for preset in &self.presets {
            preset.write(&mut stream);
        }
        stream.put_byte(self.operation);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> CameraAimAssistPresets {
        let mut stream = Stream::new(bytes, 0);

        let mut categories = Vec::new();
        let mut presets = Vec::new();
        let mut count = stream.get_unsigned_var_int();
        for _ in 0..count {
            categories.push(CameraAimAssistCategory::read(&mut stream));
        }
        count = stream.get_unsigned_var_int();
        for _ in 0..count {
            presets.push(CameraAimAssistPreset::read(&mut stream));
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
}
