use crate::protocol::bedrock::types::shape::primitive_shape_arrow_payload::PrimitiveShapeArrowPayload;
use crate::protocol::bedrock::types::shape::primitive_shape_box_payload::PrimitiveShapeBoxPayload;
use crate::protocol::bedrock::types::shape::primitive_shape_circle_or_sphere_payload::PrimitiveShapeCircleOrSpherePayload;
use crate::protocol::bedrock::types::shape::primitive_shape_cone_payload::PrimitiveShapeConePayload;
use crate::protocol::bedrock::types::shape::primitive_shape_cylinder_payload::PrimitiveShapeCylinderPayload;
use crate::protocol::bedrock::types::shape::primitive_shape_ellipsoid_payload::PrimitiveShapeEllipsoidPayload;
use crate::protocol::bedrock::types::shape::primitive_shape_line_payload::PrimitiveShapeLinePayload;
use crate::protocol::bedrock::types::shape::primitive_shape_pyramid_payload::PrimitiveShapePyramidPayload;
use crate::protocol::bedrock::types::shape::primitive_shape_text_payload::PrimitiveShapeTextPayload;
use crate::protocol::bedrock::types::shape::primitive_shape_type::PrimitiveShapeType;
use binary_utils::binary::{Reader, Writer};
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum PrimitiveShapePayload {
    Line(PrimitiveShapeLinePayload),
    Box(PrimitiveShapeBoxPayload),
    CircleOrSphere(PrimitiveShapeCircleOrSpherePayload),
    Text(PrimitiveShapeTextPayload),
    Arrow(PrimitiveShapeArrowPayload),
    Cylinder(PrimitiveShapeCylinderPayload),
    Pyramid(PrimitiveShapePyramidPayload),
    Ellipsoid(PrimitiveShapeEllipsoidPayload),
    Cone(PrimitiveShapeConePayload)
}

impl PrimitiveShapePayload {
    pub fn id(&self) -> u32 {
        match self {
            PrimitiveShapePayload::Line(_) => PrimitiveShapeType::PAYLOAD_TYPE_LINE,
            PrimitiveShapePayload::Box(_) => PrimitiveShapeType::PAYLOAD_TYPE_BOX,
            PrimitiveShapePayload::CircleOrSphere(_) => PrimitiveShapeType::PAYLOAD_TYPE_CIRCLE_OR_SPHERE,
            PrimitiveShapePayload::Text(_) => PrimitiveShapeType::PAYLOAD_TYPE_TEXT,
            PrimitiveShapePayload::Arrow(_) => PrimitiveShapeType::PAYLOAD_TYPE_ARROW,
            PrimitiveShapePayload::Cylinder(_) => PrimitiveShapeType::PAYLOAD_TYPE_CYLINDER,
            PrimitiveShapePayload::Pyramid(_) => PrimitiveShapeType::PAYLOAD_TYPE_PYRAMID,
            PrimitiveShapePayload::Ellipsoid(_) => PrimitiveShapeType::PAYLOAD_TYPE_ELLIPSOID,
            PrimitiveShapePayload::Cone(_) => PrimitiveShapeType::PAYLOAD_TYPE_CONE,
        }
    }

    pub fn read(stream: &mut Reader) -> PrimitiveShapePayload {
        let primitive_shape_type = stream.get_var_u32();
        match primitive_shape_type {
            PrimitiveShapeType::PAYLOAD_TYPE_LINE => PrimitiveShapePayload::Line(PrimitiveShapeLinePayload::read(stream)),
            PrimitiveShapeType::PAYLOAD_TYPE_BOX => PrimitiveShapePayload::Box(PrimitiveShapeBoxPayload::read(stream)),
            PrimitiveShapeType::PAYLOAD_TYPE_CIRCLE_OR_SPHERE => PrimitiveShapePayload::CircleOrSphere(PrimitiveShapeCircleOrSpherePayload::read(stream)),
            PrimitiveShapeType::PAYLOAD_TYPE_TEXT => PrimitiveShapePayload::Text(PrimitiveShapeTextPayload::read(stream)),
            PrimitiveShapeType::PAYLOAD_TYPE_ARROW => PrimitiveShapePayload::Arrow(PrimitiveShapeArrowPayload::read(stream)),
            PrimitiveShapeType::PAYLOAD_TYPE_CYLINDER => PrimitiveShapePayload::Cylinder(PrimitiveShapeCylinderPayload::read(stream)),
            PrimitiveShapeType::PAYLOAD_TYPE_PYRAMID => PrimitiveShapePayload::Pyramid(PrimitiveShapePyramidPayload::read(stream)),
            PrimitiveShapeType::PAYLOAD_TYPE_ELLIPSOID => PrimitiveShapePayload::Ellipsoid(PrimitiveShapeEllipsoidPayload::read(stream)),
            PrimitiveShapeType::PAYLOAD_TYPE_CONE => PrimitiveShapePayload::Cone(PrimitiveShapeConePayload::read(stream)),
            _ => panic!("Primitive shape type not handled: {}", primitive_shape_type),
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        match self {
            PrimitiveShapePayload::Line(r) => r.write(stream),
            PrimitiveShapePayload::Box(r) => r.write(stream),
            PrimitiveShapePayload::CircleOrSphere(r) => r.write(stream),
            PrimitiveShapePayload::Text(r) => r.write(stream),
            PrimitiveShapePayload::Arrow(r) => r.write(stream),
            PrimitiveShapePayload::Cylinder(r) => r.write(stream),
            PrimitiveShapePayload::Pyramid(r) => r.write(stream),
            PrimitiveShapePayload::Ellipsoid(r) => r.write(stream),
            PrimitiveShapePayload::Cone(r) => r.write(stream),
        }
    }
}
