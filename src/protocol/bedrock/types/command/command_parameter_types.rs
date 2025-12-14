pub struct CommandParameterType {}

impl CommandParameterType {
    pub const INT: u32 = 1; // int
    pub const VAL: u32 = 3; // float
    pub const RVAL: u32 = 4; // value
    pub const WILDCARD_INT: u32 = 5; // wildcard int
    pub const OPERATOR: u32 = 6; // operator
    pub const COMPARE_OPERATOR: u32 = 7; // compare operator
    pub const SELECTION: u32 = 8; // target
    pub const WILDCARD_SELECTION: u32 = 10; // target
    pub const PATH_COMMAND: u32 = 17; // filepath
    pub const FULL_INTEGER_RANGE: u32 = 23; // integer range
    pub const EQUIPMENT_SLOT_ENUM: u32 = 47; // equipment slots
    pub const ID: u32 = 56; // string
    pub const POSITION: u32 = 64; // x y z
    pub const POSITION_FLOAT: u32 = 65; // x y z
    pub const MESSAGE_ROOT: u32 = 68; // message
    pub const RAWTEXT: u32 = 70; // text
    pub const JSON_OBJECT: u32 = 74; // json
    pub const BLOCK_STATE_ARRAY: u32 = 84; // block states
    pub const CODE_BUILDER_ARGS: u32 = 87; // command
}
