pub struct CameraSetInstructionEaseType {}

impl CameraSetInstructionEaseType {
    pub const LINEAR: u8 = 0;
    pub const SPRING: u8 = 1;
    pub const IN_QUAD: u8 = 2;
    pub const OUT_QUAD: u8 = 3;
    pub const IN_OUT_QUAD: u8 = 4;
    pub const IN_CUBIC: u8 = 5;
    pub const OUT_CUBIC: u8 = 6;
    pub const IN_OUT_CUBIC: u8 = 7;
    pub const IN_QUART: u8 = 8;
    pub const OUT_QUART: u8 = 9;
    pub const IN_OUT_QUART: u8 = 10;
    pub const IN_QUINT: u8 = 11;
    pub const OUT_QUINT: u8 = 12;
    pub const IN_OUT_QUINT: u8 = 13;
    pub const IN_SINE: u8 = 14;
    pub const OUT_SINE: u8 = 15;
    pub const IN_OUT_SINE: u8 = 16;
    pub const IN_EXPO: u8 = 17;
    pub const OUT_EXPO: u8 = 18;
    pub const IN_OUT_EXPO: u8 = 19;
    pub const IN_CIRC: u8 = 20;
    pub const OUT_CIRC: u8 = 21;
    pub const IN_OUT_CIRC: u8 = 22;
    pub const IN_BOUNCE: u8 = 23;
    pub const OUT_BOUNCE: u8 = 24;
    pub const IN_OUT_BOUNCE: u8 = 25;
    pub const IN_BACK: u8 = 26;
    pub const OUT_BACK: u8 = 27;
    pub const IN_OUT_BACK: u8 = 28;
    pub const IN_ELASTIC: u8 = 29;
    pub const OUT_ELASTIC: u8 = 30;
    pub const IN_OUT_ELASTIC: u8 = 31;
}
