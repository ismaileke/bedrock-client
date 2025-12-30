use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: f32,
    pub shake_type: u8,
    pub shake_action: u8,
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
        stream.put_var_u32(self.id() as u32);

        stream.put_f32_le(self.intensity);
        stream.put_f32_le(self.duration);
        stream.put_byte(self.shake_type);
        stream.put_byte(self.shake_action);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CameraShake {
        let intensity = stream.get_f32_le();
        let duration = stream.get_f32_le();
        let shake_type = stream.get_byte();
        let shake_action = stream.get_byte();

        CameraShake { intensity, duration, shake_type, shake_action }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
