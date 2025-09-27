use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::script_debug_shape_type::ScriptDebugShapeType;
use crate::utils::color::Color;

#[derive(Debug)]
pub struct PacketShapeData {
    pub network_id: u64,
    pub shape_type: Option<u8>, //see types/script_debug_shape_type.rs
    pub location: Option<Vec<f32>>,
    pub scale: Option<f32>,
    pub rotation: Option<Vec<f32>>,
    pub total_time_left: Option<f32>,
    pub color: Option<Color>,
    pub text: Option<String>,
    pub box_bound: Option<Vec<f32>>,
    pub line_end_location: Option<Vec<f32>>,
    pub arrow_head_length: Option<f32>,
    pub arrow_head_radius: Option<f32>,
    pub segments: Option<u8>
}

impl PacketShapeData {
    pub fn new(
        network_id: u64,
        shape_type: Option<u8>,
        location: Option<Vec<f32>>,
        scale: Option<f32>,
        rotation: Option<Vec<f32>>,
        total_time_left: Option<f32>,
        color: Option<Color>,
        text: Option<String>,
        box_bound: Option<Vec<f32>>,
        line_end_location: Option<Vec<f32>>,
        arrow_head_length: Option<f32>,
        arrow_head_radius: Option<f32>,
        segments: Option<u8>
    ) -> PacketShapeData {
        PacketShapeData{
            network_id,
            shape_type,
            location,
            scale,
            rotation,
            total_time_left,
            color,
            text,
            box_bound,
            line_end_location,
            arrow_head_length,
            arrow_head_radius,
            segments
        }
    }

    pub fn remove(network_id: u64) -> PacketShapeData {
        PacketShapeData::new(network_id, None, None, None, None, None, None, None, None, None, None, None, None)
    }

    pub fn line(network_id: u64, location: Vec<f32>, line_end_location: Vec<f32>, color: Option<Color>) -> PacketShapeData {
        PacketShapeData::new(network_id, Some(ScriptDebugShapeType::LINE), Some(location), None, None, None, color, None, None, Some(line_end_location), None, None, None)
    }

    pub fn shape_box(network_id: u64, location: Vec<f32>, box_bound: Vec<f32>, scale: Option<f32>, color: Option<Color>) -> PacketShapeData {
        PacketShapeData::new(network_id, Some(ScriptDebugShapeType::BOX), Some(location), scale, None, None, color, None, Some(box_bound), None, None, None, None)
    }

    pub fn sphere(network_id: u64, location: Vec<f32>, scale: Option<f32>, color: Option<Color>, segments: Option<u8>) -> PacketShapeData {
        PacketShapeData::new(network_id, Some(ScriptDebugShapeType::SPHERE), Some(location), scale, None, None, color, None, None, None, None, None, segments)
    }

    pub fn circle(network_id: u64, location: Vec<f32>, scale: Option<f32>, color: Option<Color>, segments: Option<u8>) -> PacketShapeData {
        PacketShapeData::new(network_id, Some(ScriptDebugShapeType::CIRCLE), Some(location), scale, None, None, color, None, None, None, None, None, segments)
    }

    pub fn text(network_id: u64, location: Vec<f32>, text: String, color: Option<Color>) -> PacketShapeData {
        PacketShapeData::new(network_id, Some(ScriptDebugShapeType::TEXT), Some(location), None, None, None, color, Some(text), None, None, None, None, None)
    }

    pub fn arrow(network_id: u64, location: Vec<f32>, line_end_location: Vec<f32>, scale: Option<f32>, color: Option<Color>, arrow_head_length: Option<f32>, arrow_head_radius: Option<f32>, segments: Option<u8>) -> PacketShapeData {
        PacketShapeData::new(network_id, Some(ScriptDebugShapeType::ARROW), Some(location), scale, None, None, color, None, None, Some(line_end_location), arrow_head_length, arrow_head_radius, segments)
    }

    pub fn read(stream: &mut Stream) -> PacketShapeData {
        let network_id = stream.get_unsigned_var_long();
        let shape_type = PacketSerializer::read_optional(stream, |s| s.get_byte());
        let location = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let scale = PacketSerializer::read_optional(stream, |s| s.get_l_float());
        let rotation = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let total_time_left = PacketSerializer::read_optional(stream, |s| s.get_l_float());
        let color = PacketSerializer::read_optional(stream, |s| Color::from_argb(s.get_l_int()));
        let text = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));
        let box_bound = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let line_end_location = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let arrow_head_length = PacketSerializer::read_optional(stream, |s| s.get_l_float());
        let arrow_head_radius = PacketSerializer::read_optional(stream, |s| s.get_l_float());
        let segments = PacketSerializer::read_optional(stream, |s| s.get_byte());

        PacketShapeData {
            network_id,
            shape_type,
            location,
            scale,
            rotation,
            total_time_left,
            color,
            text,
            box_bound,
            line_end_location,
            arrow_head_length,
            arrow_head_radius,
            segments,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_unsigned_var_long(self.network_id);
        PacketSerializer::write_optional(stream, &self.shape_type, |s, v| s.put_byte(*v));
        PacketSerializer::write_optional(stream, &self.location, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.scale, |s, v| s.put_l_float(*v));
        PacketSerializer::write_optional(stream, &self.rotation, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.total_time_left, |s, v| s.put_l_float(*v));
        PacketSerializer::write_optional(stream, &self.color, |s, v| s.put_l_int(v.to_argb()));
        PacketSerializer::write_optional(stream, &self.text, |s, v| PacketSerializer::put_string(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.box_bound, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.line_end_location, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.arrow_head_length, |s, v| s.put_l_float(*v));
        PacketSerializer::write_optional(stream, &self.arrow_head_radius, |s, v| s.put_l_float(*v));
        PacketSerializer::write_optional(stream, &self.segments, |s, v| s.put_byte(*v));
    }
}