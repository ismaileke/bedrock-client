use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::camera::camera_preset::CameraPreset;

pub struct CameraPresets {
    pub presets: Vec<CameraPreset>
}

pub fn new(presets: Vec<CameraPreset>) -> CameraPresets {
    CameraPresets { presets }
}

impl Packet for CameraPresets {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraPresets.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.presets.len() as u32);
        for preset in self.presets.iter() {
            preset.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CameraPresets {
        let mut presets = Vec::<CameraPreset>::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            presets.push(CameraPreset::read(stream));
        }

        CameraPresets { presets }
    }

    fn debug(&self) {
        println!("Presets {:?}", self.presets);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
