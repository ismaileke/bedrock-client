pub struct PrimitiveShapeType {}

impl PrimitiveShapeType {
    pub const LINE: u8 = 0;
    pub const BOX: u8 = 1;
    pub const SPHERE: u8 = 2;
    pub const CIRCLE: u8 = 3;
    pub const TEXT: u8 = 4;
    pub const ARROW: u8 = 5;
    pub const CYLINDER: u8 = 6;
    pub const PYRAMID: u8 = 7;
    pub const ELLIPSOID: u8 = 8;
    pub const CONE: u8 = 9;

    pub const PAYLOAD_TYPE_NONE: u32 = 0;
    pub const PAYLOAD_TYPE_ARROW: u32 = 1;
    pub const PAYLOAD_TYPE_TEXT: u32 = 2;
    pub const PAYLOAD_TYPE_BOX: u32 = 3;
    pub const PAYLOAD_TYPE_LINE: u32 = 4;
    pub const PAYLOAD_TYPE_CIRCLE_OR_SPHERE: u32 = 5;
    pub const PAYLOAD_TYPE_CYLINDER: u32 = 6;
    pub const PAYLOAD_TYPE_PYRAMID: u32 = 7;
    pub const PAYLOAD_TYPE_ELLIPSOID: u32 = 8;
    pub const PAYLOAD_TYPE_CONE: u32 = 9;

    pub fn get_payload_type(shape_type: u8) -> u32 {
        match shape_type {
            PrimitiveShapeType::LINE => PrimitiveShapeType::PAYLOAD_TYPE_LINE,
            PrimitiveShapeType::BOX => PrimitiveShapeType::PAYLOAD_TYPE_BOX,
            PrimitiveShapeType::SPHERE => PrimitiveShapeType::PAYLOAD_TYPE_CIRCLE_OR_SPHERE,
            PrimitiveShapeType::CIRCLE => PrimitiveShapeType::PAYLOAD_TYPE_CIRCLE_OR_SPHERE,
            PrimitiveShapeType::TEXT => PrimitiveShapeType::PAYLOAD_TYPE_TEXT,
            PrimitiveShapeType::ARROW => PrimitiveShapeType::PAYLOAD_TYPE_ARROW,
            PrimitiveShapeType::CYLINDER => PrimitiveShapeType::PAYLOAD_TYPE_CYLINDER,
            PrimitiveShapeType::PYRAMID => PrimitiveShapeType::PAYLOAD_TYPE_PYRAMID,
            PrimitiveShapeType::ELLIPSOID => PrimitiveShapeType::PAYLOAD_TYPE_ELLIPSOID,
            PrimitiveShapeType::CONE => PrimitiveShapeType::PAYLOAD_TYPE_CONE,
            _ => panic!("Invalid shape type: {}", shape_type),
        }
    }
    
}
