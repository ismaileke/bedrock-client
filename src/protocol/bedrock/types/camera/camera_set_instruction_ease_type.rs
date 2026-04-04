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

impl CameraSetInstructionEaseType {
    pub fn name(value: u8) -> &'static str {
        match value {
            Self::LINEAR => "linear",
            Self::SPRING => "spring",
            Self::IN_SINE => "in_sine",
            Self::OUT_SINE => "out_sine",
            Self::IN_OUT_SINE => "in_out_sine",
            Self::IN_QUAD => "in_quad",
            Self::OUT_QUAD => "out_quad",
            Self::IN_OUT_QUAD => "in_out_quad",
            Self::IN_CUBIC => "in_cubic",
            Self::OUT_CUBIC => "out_cubic",
            Self::IN_OUT_CUBIC => "in_out_cubic",
            Self::IN_QUART => "in_quart",
            Self::OUT_QUART => "out_quart",
            Self::IN_OUT_QUART => "in_out_quart",
            Self::IN_QUINT => "in_quint",
            Self::OUT_QUINT => "out_quint",
            Self::IN_OUT_QUINT => "in_out_quint",
            Self::IN_EXPO => "in_expo",
            Self::OUT_EXPO => "out_expo",
            Self::IN_OUT_EXPO => "in_out_expo",
            Self::IN_CIRC => "in_circ",
            Self::OUT_CIRC => "out_circ",
            Self::IN_OUT_CIRC => "in_out_circ",
            Self::IN_BACK => "in_back",
            Self::OUT_BACK => "out_back",
            Self::IN_OUT_BACK => "in_out_back",
            Self::IN_ELASTIC => "in_elastic",
            Self::OUT_ELASTIC => "out_elastic",
            Self::IN_OUT_ELASTIC => "in_out_elastic",
            Self::IN_BOUNCE => "in_bounce",
            Self::OUT_BOUNCE => "out_bounce",
            Self::IN_OUT_BOUNCE => "in_out_bounce",
            _ => "unknown",
        }
    }
}