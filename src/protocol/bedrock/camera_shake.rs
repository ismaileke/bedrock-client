use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

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
        BedrockPacketType::IDCameraShake.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_f32_le(self.intensity);
        stream.put_f32_le(self.duration);
        stream.put_u8(self.shake_type);
        stream.put_u8(self.shake_action);
    }

    fn decode(stream: &mut Reader) -> CameraShake {
        let intensity = stream.get_f32_le();
        let duration = stream.get_f32_le();
        let shake_type = stream.get_u8();
        let shake_action = stream.get_u8();

        CameraShake { intensity, duration, shake_type, shake_action }
    }
}
