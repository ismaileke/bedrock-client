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
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.presets.len() as u32);
        for preset in self.presets.iter() {
            preset.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> CameraPresets {
        let mut stream = Stream::new(bytes, 0);

        let mut presets = Vec::<CameraPreset>::new();
        let count = stream.get_unsigned_var_int();
        for _ in 0..count {
            presets.push(CameraPreset::read(&mut stream));
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
