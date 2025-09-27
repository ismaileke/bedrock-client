pub struct ScriptDebugShapeType {}

impl ScriptDebugShapeType {
    pub const LINE: u8 = 0;
    pub const BOX: u8 = 1;
    pub const SPHERE: u8 = 2;
    pub const CIRCLE: u8 = 3;
    pub const TEXT: u8 = 4;
    pub const ARROW: u8 = 5;
    pub const TEST: u8 = Self::TEXT; //deprecated
}
