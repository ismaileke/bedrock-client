use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::camera::camera_preset::CameraPreset;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CameraPresets {
    pub presets: Vec<CameraPreset>,
}

impl Packet for CameraPresets {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraPresets.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.presets.len() as u32);
        for preset in self.presets.iter() {
            preset.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> CameraPresets {
        let mut presets = Vec::<CameraPreset>::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            presets.push(CameraPreset::read(stream));
        }

        CameraPresets { presets }
    }
}
