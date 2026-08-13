use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CameraAimAssist {
    pub preset_id: String,
    pub view_angle: Vec<f32>,
    pub distance: f32,
    pub target_mode: u8, /// see types/camera/camera_aim_assist_target_mode
    pub action_type: u8, /// see types/camera/camera_aim_assist_action_type
    pub show_debug_render: bool,
}

impl Packet for CameraAimAssist {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraAimAssist.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.preset_id.clone());
        PacketSerializer::put_vector2(stream, self.view_angle.clone());
        stream.put_f32_le(self.distance);
        stream.put_u8(self.target_mode);
        stream.put_u8(self.action_type);
        stream.put_bool(self.show_debug_render);
    }

    fn decode(stream: &mut Reader) -> CameraAimAssist {
        let preset_id = PacketSerializer::get_string(stream);
        let view_angle = PacketSerializer::get_vector2(stream);
        let distance = stream.get_f32_le();
        let target_mode = stream.get_u8();
        let action_type = stream.get_u8();
        let show_debug_render = stream.get_bool();

        CameraAimAssist {
            preset_id,
            view_angle,
            distance,
            target_mode,
            action_type,
            show_debug_render,
        }
    }
}
