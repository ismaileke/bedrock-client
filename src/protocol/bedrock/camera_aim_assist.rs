use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct CameraAimAssist {
    pub preset_id: String,
    pub view_angle: Vec<f32>,
    pub distance: f32,
    pub target_mode: u8, //see types/camera/camera_aim_assist_target_mode
    pub action_type: u8, //see types/camera/camera_aim_assist_action_type
    pub show_debug_render: bool
}

pub fn new(preset_id: String, view_angle: Vec<f32>, distance: f32, target_mode: u8, action_type: u8, show_debug_render: bool) -> CameraAimAssist {
    CameraAimAssist { preset_id, view_angle, distance, target_mode, action_type, show_debug_render }
}

impl Packet for CameraAimAssist {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraAimAssist.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.preset_id.clone());
        PacketSerializer::put_vector2(&mut stream, self.view_angle.clone());
        stream.put_f32_le(self.distance);
        stream.put_byte(self.target_mode);
        stream.put_byte(self.action_type);
        stream.put_bool(self.show_debug_render);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> CameraAimAssist {
        let mut stream = Stream::new(bytes, 0);

        let preset_id = PacketSerializer::get_string(&mut stream);
        let view_angle = PacketSerializer::get_vector2(&mut stream);
        let distance = stream.get_f32_le();
        let target_mode = stream.get_byte();
        let action_type = stream.get_byte();
        let show_debug_render = stream.get_bool();

        CameraAimAssist { preset_id, view_angle, distance, target_mode, action_type, show_debug_render }
    }

    fn debug(&self) {
        println!("Preset ID: {}", self.preset_id);
        println!("View angle: {:?}", self.view_angle);
        println!("Distance: {}", self.distance);
        println!("Target mode: {}", self.target_mode);
        println!("Action type: {}", self.action_type);
        println!("Show debug render: {}", self.show_debug_render);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
