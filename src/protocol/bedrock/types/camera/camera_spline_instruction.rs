use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::camera::camera_rotation_option::CameraRotationOption;

#[derive(Debug)]
pub struct CameraSplineInstruction {
    pub total_time: f32,
    pub ease_type: u8, // see types/camera/camera_set_instruction_ease_type.rs
    pub curve: Vec<Vec<f32>>,
    pub progress_key_frames: Vec<Vec<f32>>,
    pub rotation_options: Vec<CameraRotationOption>
}

impl CameraSplineInstruction {
    pub fn new(total_time: f32, ease_type: u8, curve: Vec<Vec<f32>>, progress_key_frames: Vec<Vec<f32>>, rotation_options: Vec<CameraRotationOption>) -> CameraSplineInstruction {
        CameraSplineInstruction{ total_time, ease_type, curve, progress_key_frames, rotation_options }
    }

    pub fn read(stream: &mut Stream) -> CameraSplineInstruction {
        let total_time = stream.get_l_float();
        let ease_type = stream.get_byte();

        let mut curve = Vec::new();
        let curve_count = stream.get_unsigned_var_int();
        for _ in 0..curve_count {
            curve.push(PacketSerializer::get_vector3(stream));
        }

        let mut progress_key_frames = Vec::new();
        let progress_key_frames_count = stream.get_unsigned_var_int();
        for _ in 0..progress_key_frames_count {
            progress_key_frames.push(PacketSerializer::get_vector2(stream));
        }

        let mut rotation_options = Vec::new();
        let rotation_options_count = stream.get_unsigned_var_int();
        for _ in 0..rotation_options_count {
            rotation_options.push(CameraRotationOption::read(stream));
        }

        CameraSplineInstruction{ total_time, ease_type, curve, progress_key_frames, rotation_options }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_float(self.total_time);
        stream.put_byte(self.ease_type);

        stream.put_unsigned_var_int(self.curve.len() as u32);
        for curve in &self.curve {
            PacketSerializer::put_vector3(stream, curve.clone());
        }

        stream.put_unsigned_var_int(self.progress_key_frames.len() as u32);
        for progress_key_frames in &self.progress_key_frames {
            PacketSerializer::put_vector2(stream, progress_key_frames.clone());
        }

        stream.put_unsigned_var_int(self.rotation_options.len() as u32);
        for rotation_options in &self.rotation_options {
            rotation_options.write(stream);
        }
    }
}