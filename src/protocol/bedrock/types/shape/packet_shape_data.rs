use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::shape::primitive_shape_type::PrimitiveShapeType;
use crate::protocol::bedrock::types::shape::primitive_shape_payload::PrimitiveShapePayload;
use crate::utils::color::Color;
use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::types::shape::primitive_shape_arrow_payload::PrimitiveShapeArrowPayload;
use crate::protocol::bedrock::types::shape::primitive_shape_box_payload::PrimitiveShapeBoxPayload;
use crate::protocol::bedrock::types::shape::primitive_shape_circle_or_sphere_payload::PrimitiveShapeCircleOrSpherePayload;
use crate::protocol::bedrock::types::shape::primitive_shape_cone_payload::PrimitiveShapeConePayload;
use crate::protocol::bedrock::types::shape::primitive_shape_cylinder_payload::PrimitiveShapeCylinderPayload;
use crate::protocol::bedrock::types::shape::primitive_shape_ellipsoid_payload::PrimitiveShapeEllipsoidPayload;
use crate::protocol::bedrock::types::shape::primitive_shape_line_payload::PrimitiveShapeLinePayload;
use crate::protocol::bedrock::types::shape::primitive_shape_pyramid_payload::PrimitiveShapePyramidPayload;
use crate::protocol::bedrock::types::shape::primitive_shape_text_payload::PrimitiveShapeTextPayload;

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
    pub dimension_id: Option<i32>,
    pub attached_to_entity_id: Option<u64>,
    pub payload: Option<PrimitiveShapePayload>
}

impl PacketShapeData {

    pub fn remove(network_id: u64, dimension_id: Option<i32>) -> PacketShapeData {
        PacketShapeData {
            network_id,
            shape_type: None,
            location: None,
            scale: None,
            rotation: None,
            total_time_left: None,
            maximum_render_distance: None,
            color: None,
            dimension_id,
            attached_to_entity_id: None,
            payload: None,
        }
    }

    pub fn line(network_id: u64, location: Vec<f32>, line_end_location: Vec<f32>, color: Option<Color>, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData {
            network_id,
            shape_type: Some(PrimitiveShapeType::LINE),
            location: Some(location),
            scale: None,
            rotation: None,
            total_time_left: None,
            maximum_render_distance: None,
            color,
            dimension_id,
            attached_to_entity_id,
            payload: Some(PrimitiveShapePayload::Line(PrimitiveShapeLinePayload::new(line_end_location))),
        }
    }

    pub fn shape_box(network_id: u64, location: Vec<f32>, box_bound: Vec<f32>, scale: Option<f32>, color: Option<Color>, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData {
            network_id,
            shape_type: Some(PrimitiveShapeType::BOX),
            location: Some(location),
            scale,
            rotation: None,
            total_time_left: None,
            maximum_render_distance: None,
            color,
            dimension_id,
            attached_to_entity_id,
            payload: Some(PrimitiveShapePayload::Box(PrimitiveShapeBoxPayload::new(box_bound))),
        }
    }

    pub fn sphere(network_id: u64, location: Vec<f32>, scale: Option<f32>, color: Option<Color>, segments: u8, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData {
            network_id,
            shape_type: Some(PrimitiveShapeType::SPHERE),
            location: Some(location),
            scale,
            rotation: None,
            total_time_left: None,
            maximum_render_distance: None,
            color,
            dimension_id,
            attached_to_entity_id,
            payload: Some(PrimitiveShapePayload::CircleOrSphere(PrimitiveShapeCircleOrSpherePayload::new(segments))),
        }
    }

    pub fn circle(network_id: u64, location: Vec<f32>, scale: Option<f32>, color: Option<Color>, segments: u8, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData {
            network_id,
            shape_type: Some(PrimitiveShapeType::CIRCLE),
            location: Some(location),
            scale,
            rotation: None,
            total_time_left: None,
            maximum_render_distance: None,
            color,
            dimension_id,
            attached_to_entity_id,
            payload: Some(PrimitiveShapePayload::CircleOrSphere(PrimitiveShapeCircleOrSpherePayload::new(segments))),
        }
    }

    pub fn text(network_id: u64, location: Vec<f32>, text: String, use_rotation: bool, background_color: Option<Color>, depth_test: bool, show_backface: bool, show_text_backface: bool, color: Option<Color>, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData {
            network_id,
            shape_type: Some(PrimitiveShapeType::TEXT),
            location: Some(location),
            scale: None,
            rotation: None,
            total_time_left: None,
            maximum_render_distance: None,
            color,
            dimension_id,
            attached_to_entity_id,
            payload: Some(PrimitiveShapePayload::Text(PrimitiveShapeTextPayload::new(text, use_rotation, background_color, depth_test, show_backface, show_text_backface))),
        }
    }

    pub fn arrow(network_id: u64, location: Vec<f32>, line_end_location: Vec<f32>, scale: Option<f32>, color: Option<Color>, arrow_head_length: Option<f32>, arrow_head_radius: Option<f32>, segments: Option<u8>, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData {
            network_id,
            shape_type: Some(PrimitiveShapeType::ARROW),
            location: Some(location),
            scale,
            rotation: None,
            total_time_left: None,
            maximum_render_distance: None,
            color,
            dimension_id,
            attached_to_entity_id,
            payload: Some(PrimitiveShapePayload::Arrow(PrimitiveShapeArrowPayload::new(Some(line_end_location), arrow_head_length, arrow_head_radius, segments))),
        }
    }

    pub fn cylinder(network_id: u64, location: Vec<f32>, scale: Option<f32>, color: Option<Color>, radius_x: Vec<f32>, radius_z: Vec<f32>, height: f32, segments: u8,  dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData {
            network_id,
            shape_type: Some(PrimitiveShapeType::CYLINDER),
            location: Some(location),
            scale,
            rotation: None,
            total_time_left: None,
            maximum_render_distance: None,
            color,
            dimension_id,
            attached_to_entity_id,
            payload: Some(PrimitiveShapePayload::Cylinder(PrimitiveShapeCylinderPayload::new(radius_x, radius_z, height, segments))),
        }
    }

    pub fn pyramid(network_id: u64, location: Vec<f32>, width: f32, height: f32, depth: Option<f32>, color: Option<Color>, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData {
            network_id,
            shape_type: Some(PrimitiveShapeType::PYRAMID),
            location: Some(location),
            scale: None,
            rotation: None,
            total_time_left: None,
            maximum_render_distance: None,
            color,
            dimension_id,
            attached_to_entity_id,
            payload: Some(PrimitiveShapePayload::Pyramid(PrimitiveShapePyramidPayload::new(width, depth, height)))
        }
    }

    pub fn ellipsoid(network_id: u64, location: Vec<f32>, radii: Vec<f32>, segments_per_axis: u8, color: Option<Color>, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData {
            network_id,
            shape_type: Some(PrimitiveShapeType::ELLIPSOID),
            location: Some(location),
            scale: None,
            rotation: None,
            total_time_left: None,
            maximum_render_distance: None,
            color,
            dimension_id,
            attached_to_entity_id,
            payload: Some(PrimitiveShapePayload::Ellipsoid(PrimitiveShapeEllipsoidPayload::new(radii, segments_per_axis)))
        }
    }

    pub fn cone(network_id: u64, location: Vec<f32>, radii: Vec<f32>, height: f32, segments: u8, color: Option<Color>, dimension_id: Option<i32>, attached_to_entity_id: Option<u64>) -> PacketShapeData {
        PacketShapeData {
            network_id,
            shape_type: Some(PrimitiveShapeType::CONE),
            location: Some(location),
            scale: None,
            rotation: None,
            total_time_left: None,
            maximum_render_distance: None,
            color,
            dimension_id,
            attached_to_entity_id,
            payload: Some(PrimitiveShapePayload::Cone(PrimitiveShapeConePayload::new(radii, height, segments)))
        }
    }

    pub fn read(stream: &mut Reader) -> PacketShapeData {
        let network_id = stream.get_var_u64();
        let shape_type = PacketSerializer::read_optional(stream, |s| s.get_u8());
        let location = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let scale = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let rotation = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let total_time_left = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let maximum_render_distance = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let color = PacketSerializer::read_optional(stream, |s| Color::from_argb(s.get_u32_le()));
        let dimension_id = PacketSerializer::read_optional(stream, |s| s.get_var_i32());
        let attached_to_entity_id = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_actor_runtime_id(s));
        let payload_type = stream.get_var_u32();

        //WTF IS THIS HORROR SHOW
        if (shape_type.is_some() && payload_type != (shape_type.unwrap() as u32) && payload_type != PrimitiveShapeType::PAYLOAD_TYPE_NONE) ||
            (shape_type.is_none() && payload_type != PrimitiveShapeType::PAYLOAD_TYPE_NONE) {
            panic!("Unexpected payload type {} for provided shape type {} (not set)", payload_type, shape_type.expect("PacketShapeData Line 217"));
        }

        let payload = match payload_type {
            PrimitiveShapeType::PAYLOAD_TYPE_NONE => None,
            PrimitiveShapeType::PAYLOAD_TYPE_ARROW => Some(PrimitiveShapePayload::Arrow(PrimitiveShapeArrowPayload::read(stream))),
            PrimitiveShapeType::PAYLOAD_TYPE_TEXT => Some(PrimitiveShapePayload::Text(PrimitiveShapeTextPayload::read(stream))),
            PrimitiveShapeType::PAYLOAD_TYPE_BOX => Some(PrimitiveShapePayload::Box(PrimitiveShapeBoxPayload::read(stream))),
            PrimitiveShapeType::PAYLOAD_TYPE_LINE => Some(PrimitiveShapePayload::Line(PrimitiveShapeLinePayload::read(stream))),
            PrimitiveShapeType::PAYLOAD_TYPE_CIRCLE_OR_SPHERE => Some(PrimitiveShapePayload::CircleOrSphere(PrimitiveShapeCircleOrSpherePayload::read(stream))),
            PrimitiveShapeType::PAYLOAD_TYPE_CYLINDER => Some(PrimitiveShapePayload::Cylinder(PrimitiveShapeCylinderPayload::read(stream))),
            PrimitiveShapeType::PAYLOAD_TYPE_PYRAMID => Some(PrimitiveShapePayload::Pyramid(PrimitiveShapePyramidPayload::read(stream))),
            PrimitiveShapeType::PAYLOAD_TYPE_ELLIPSOID => Some(PrimitiveShapePayload::Ellipsoid(PrimitiveShapeEllipsoidPayload::read(stream))),
            PrimitiveShapeType::PAYLOAD_TYPE_CONE => Some(PrimitiveShapePayload::Cone(PrimitiveShapeConePayload::read(stream))),
            _ => panic!("Unknown payload type {}", payload_type)
        };

        PacketShapeData {
            network_id,
            shape_type,
            location,
            scale,
            rotation,
            total_time_left,
            maximum_render_distance,
            color,
            dimension_id,
            attached_to_entity_id,
            payload,
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u64(self.network_id);
        PacketSerializer::write_optional(stream, &self.shape_type, |s, v| s.put_u8(*v));
        PacketSerializer::write_optional(stream, &self.location, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.scale, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.rotation, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.total_time_left, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.maximum_render_distance, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.color, |s, v| s.put_u32_le(v.to_argb()));
        PacketSerializer::write_optional(stream, &self.dimension_id, |s, v| s.put_var_i32(*v));
        PacketSerializer::write_optional(stream, &self.attached_to_entity_id, |s, v| PacketSerializer::put_actor_runtime_id(s, *v));
        if let Some(payload) = &self.payload {
            stream.put_var_u32(payload.id());
            payload.write(stream);
        } else {
            stream.put_var_u32(PrimitiveShapeType::PAYLOAD_TYPE_NONE);
        }
    }
}
