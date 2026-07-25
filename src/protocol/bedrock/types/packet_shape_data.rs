use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::script_debug_shape_type::ScriptDebugShapeType;
use crate::utils::color::Color;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct PacketShapeData {
    pub network_id: u64,
    pub shape_type: Option<u8>, //see types/script_debug_shape_type.rs
    pub location: Option<Vec<f32>>,
    pub scale: Option<f32>,
    pub rotation: Option<Vec<f32>>,
    pub total_time_left: Option<f32>,
    pub maximum_render_distance: Option<f32>,
    pub color: Option<Color>,
    pub text: Option<String>,
    pub use_rotation: Option<bool>,
    pub background_color: Option<Color>,
    pub depth_test: Option<bool>,
    pub show_backface: Option<bool>,
    pub show_text_backface: Option<bool>,
    pub box_bound: Option<Vec<f32>>,
    pub line_end_location: Option<Vec<f32>>,
    pub arrow_head_length: Option<f32>,
    pub arrow_head_radius: Option<f32>,
    pub segments: Option<u8>,
    pub dimension_id: Option<i32>,
    pub attached_to_entity_id: Option<u64>,
}

impl PacketShapeData {
    pub fn new(
        network_id: u64,
        shape_type: Option<u8>,
        location: Option<Vec<f32>>,
        scale: Option<f32>,
        rotation: Option<Vec<f32>>,
        total_time_left: Option<f32>,
        maximum_render_distance: Option<f32>,
        color: Option<Color>,
        text: Option<String>,
        use_rotation: Option<bool>,
        background_color: Option<Color>,
        depth_test: Option<bool>,
        show_backface: Option<bool>,
        show_text_backface: Option<bool>,
        box_bound: Option<Vec<f32>>,
        line_end_location: Option<Vec<f32>>,
        arrow_head_length: Option<f32>,
        arrow_head_radius: Option<f32>,
        segments: Option<u8>,
        dimension_id: Option<i32>,
        attached_to_entity_id: Option<u64>,
    ) -> PacketShapeData {
        PacketShapeData {
            network_id,
            shape_type,
            location,
            scale,
            rotation,
            total_time_left,
            maximum_render_distance,
            color,
            text,
            use_rotation,
            background_color,
            depth_test,
            show_backface,
            show_text_backface,
            box_bound,
            line_end_location,
            arrow_head_length,
            arrow_head_radius,
            segments,
            dimension_id,
            attached_to_entity_id,
        }
    }

    pub fn remove(network_id: u64, dimension_id: Option<i32>) -> PacketShapeData {
        PacketShapeData::new(network_id, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, dimension_id, None)
    }

    pub fn line(network_id: u64, location: Vec<f32>, line_end_location: Vec<f32>, color: Option<Color>, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData::new(network_id, Some(ScriptDebugShapeType::LINE), Some(location), None, None, None, None, color, None, None, None, None, None, None, None, Some(line_end_location), None, None, None, dimension_id, attached_to_entity_id)
    }

    pub fn shape_box(network_id: u64, location: Vec<f32>, box_bound: Vec<f32>, scale: Option<f32>, color: Option<Color>, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData::new(network_id, Some(ScriptDebugShapeType::BOX), Some(location), scale, None, None, None, color, None, None, None, None, None, None, Some(box_bound), None, None, None, None, dimension_id, attached_to_entity_id)
    }

    pub fn sphere(network_id: u64, location: Vec<f32>, scale: Option<f32>, color: Option<Color>, segments: Option<u8>, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData::new(network_id, Some(ScriptDebugShapeType::SPHERE), Some(location), scale, None, None, None, color, None, None, None, None, None, None, None, None, None, None, segments, dimension_id, attached_to_entity_id)
    }

    pub fn circle(network_id: u64, location: Vec<f32>, scale: Option<f32>, color: Option<Color>, segments: Option<u8>, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData::new(network_id, Some(ScriptDebugShapeType::CIRCLE), Some(location), scale, None, None, None, color, None, None, None, None, None, None, None, None, None, None, segments, dimension_id, attached_to_entity_id)
    }

    pub fn text(network_id: u64, location: Vec<f32>, text: String, use_rotation: bool, background_color: Option<Color>, depth_test: bool, show_backface: bool, show_text_backface: bool, color: Option<Color>, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData::new(network_id, Some(ScriptDebugShapeType::TEXT), Some(location), None, None, None, None, color, Some(text), Some(use_rotation), background_color, Some(depth_test), Some(show_backface), Some(show_text_backface), None, None, None, None, None, dimension_id, attached_to_entity_id)
    }

    pub fn arrow(network_id: u64, location: Vec<f32>, line_end_location: Vec<f32>, scale: Option<f32>, color: Option<Color>, arrow_head_length: Option<f32>, arrow_head_radius: Option<f32>, segments: Option<u8>, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData::new(network_id, Some(ScriptDebugShapeType::ARROW), Some(location), scale, None, None, None, color, None, None, None, None, None, None, None, Some(line_end_location), arrow_head_length, arrow_head_radius, segments, dimension_id, attached_to_entity_id)
    }

    pub fn read(stream: &mut Stream) -> PacketShapeData {
        let network_id = stream.get_var_u64();
        let shape_type = PacketSerializer::read_optional(stream, |s| s.get_byte());
        let location = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let scale = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let rotation = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let total_time_left = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let maximum_render_distance = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let color = PacketSerializer::read_optional(stream, |s| Color::from_argb(s.get_u32_le()));
        let text = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));
        let use_rotation = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let background_color = PacketSerializer::read_optional(stream, |s| Color::from_argb(s.get_u32_le()));
        let depth_test = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let show_backface = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let show_text_backface = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let box_bound = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let line_end_location = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let arrow_head_length = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let arrow_head_radius = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let segments = PacketSerializer::read_optional(stream, |s| s.get_byte());
        let dimension_id = PacketSerializer::read_optional(stream, |s| s.get_var_i32());
        let attached_to_entity_id = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_actor_runtime_id(s));

        PacketShapeData {
            network_id,
            shape_type,
            location,
            scale,
            rotation,
            total_time_left,
            maximum_render_distance,
            color,
            text,
            use_rotation,
            background_color,
            depth_test,
            show_backface,
            show_text_backface,
            box_bound,
            line_end_location,
            arrow_head_length,
            arrow_head_radius,
            segments,
            dimension_id,
            attached_to_entity_id,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u64(self.network_id);
        PacketSerializer::write_optional(stream, &self.shape_type, |s, v| s.put_byte(*v));
        PacketSerializer::write_optional(stream, &self.location, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.scale, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.rotation, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.total_time_left, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.maximum_render_distance, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.color, |s, v| s.put_u32_le(v.to_argb()));
        PacketSerializer::write_optional(stream, &self.text, |s, v| PacketSerializer::put_string(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.use_rotation, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.background_color, |s, v| s.put_u32_le(v.to_argb()));
        PacketSerializer::write_optional(stream, &self.depth_test, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.show_backface, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.show_text_backface, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.box_bound, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.line_end_location, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.arrow_head_length, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.arrow_head_radius, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.segments, |s, v| s.put_byte(*v));
        PacketSerializer::write_optional(stream, &self.dimension_id, |s, v| s.put_var_i32(*v));
        PacketSerializer::write_optional(stream, &self.attached_to_entity_id, |s, v| PacketSerializer::put_actor_runtime_id(s, *v));
    }
}
