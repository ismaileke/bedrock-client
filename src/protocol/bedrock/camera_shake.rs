use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct CameraShake {
    pub intensity: f32,
    pub duration: f32,
    pub shake_type: u8,
    pub shake_action: u8
}

pub fn new(intensity: f32, duration: f32, shake_type: u8, shake_action: u8) -> CameraShake {
    CameraShake { intensity, duration, shake_type, shake_action }
}

impl CameraShake {
    pub const TYPE_POSITIONAL: u8 = 0;
    pub const TYPE_ROTATIONAL: u8 = 1;

    pub const ACTION_ADD: u8 = 0;
    pub const ACTION_STOP: u8 = 1;
}

impl Packet for CameraShake {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraShake.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_l_float(self.intensity);
        stream.put_l_float(self.duration);
        stream.put_byte(self.shake_type);
        stream.put_byte(self.shake_action);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> CameraShake {
        let mut stream = Stream::new(bytes, 0);

        let intensity = stream.get_l_float();
        let duration = stream.get_l_float();
        let shake_type = stream.get_byte();
        let shake_action = stream.get_byte();

        CameraShake { intensity, duration, shake_type, shake_action }
    }

    fn debug(&self) {
        println!("Intensity: {}", self.intensity);
        println!("Duration: {}", self.duration);
        println!("Shake Type: {}", self.shake_type);
        println!("Shake Action: {}", self.shake_action);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
